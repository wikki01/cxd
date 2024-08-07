use std::path::Path;

use crate::{
    command::Command,
    error::{CxdError, Result},
};
use rusqlite::{ffi::Error, Connection, ErrorCode};

mod arg_row;
mod cmd_row;
mod env_row;

pub use arg_row::ArgRow;
pub use cmd_row::CmdRow;
pub use env_row::EnvRow;

/// Represents a connection to the database for operating on commands
pub struct CommandStore {
    c: Connection,
}

impl CommandStore {
    /// Initializes the table schemas and required configuration variables
    ///
    /// # Args
    /// * `path` - Path to backing database file, created if not present
    ///
    /// # Returns
    /// A handle to a database connection for operations on commands.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let c = Connection::open(path)?;
        // Enable foreign key support
        c.execute("PRAGMA foreign_keys = ON", ())?;
        CmdRow::init(&c)?;
        ArgRow::init(&c)?;
        EnvRow::init(&c)?;
        Ok(Self { c })
    }

    /// Attempts to insert a command into the database
    ///
    /// # Args
    /// * `cmd` - Command to insert, `cmd.id` will be ignored
    ///
    /// # Returns
    /// The `id` of the newly created command, or `None` if one already exists with matching unique constraints.
    pub fn insert(&self, cmd: &Command) -> Result<Option<i64>> {
        // Creating command entry
        let mut command_stmt = self
            .c
            .prepare("INSERT INTO cxd_cmd (name, cmd, dir) VALUES (?1, ?2, ?3) RETURNING (id)")?;
        let mut result = command_stmt.query((
            &cmd.name,
            &cmd.command,
            cmd.dir.to_str().unwrap_or_default(),
        ))?;
        let id: i64 = match result.next() {
            Ok(Some(row)) => row.get("id")?,
            Err(rusqlite::Error::SqliteFailure(
                Error {
                    code: ErrorCode::ConstraintViolation,
                    extended_code: 2067,
                },
                _,
            )) => {
                // Already exists
                return Ok(None);
            }
            Err(e) => Err(e)?,
            Ok(None) => Err(CxdError::CommandExists(cmd.name.clone()))?,
        };

        // Creating args
        let mut args_stmt = self
            .c
            .prepare("INSERT INTO cxd_arg (data, cmd_id) VALUES (?1, ?2)")?;
        for arg in &cmd.args {
            args_stmt.execute((arg, id))?;
        }

        // Creating envs
        let mut envs_stmt = self
            .c
            .prepare("INSERT INTO cxd_env (key, value, cmd_id) VALUES (?1, ?2, ?3)")?;
        for env in &cmd.envs {
            envs_stmt.execute((&env.0, &env.1, id))?;
        }
        Ok(Some(id))
    }

    /// Attempts to get a command by name
    ///
    /// # Args
    /// * `name` - Name of command to search for
    ///
    /// # Returns
    /// The found command, or `None` if none found.
    pub fn get_by_name(&self, name: &str) -> Result<Option<Command>> {
        let mut command_stmt = self.c.prepare("SELECT * FROM cxd_cmd WHERE name = ?1")?;
        let mut rows = command_stmt.query([name])?;
        Ok(self.assemble(&mut rows)?.pop())
    }

    /// Attempts to delete a command by name
    ///
    /// # Args
    /// * `name` - Name of command to search for and delete
    ///
    /// # Returns
    /// The deleted command, or `None` if none found.
    pub fn delete_by_name(&self, name: &str) -> Result<Option<Command>> {
        let mut delete_cmd_stmt = self
            .c
            .prepare("DELETE FROM cxd_cmd WHERE name = ?1 RETURNING *")?;
        let mut rows = delete_cmd_stmt.query([name])?;
        Ok(self.assemble(&mut rows)?.pop())
    }

    /// Attempts to delete a command by ID
    ///
    /// # Args
    /// * `id` - ID of command to search for and delete
    ///
    /// # Returns
    /// The deleted command, or `None` if none found.
    pub fn delete_by_id(&self, id: i64) -> Result<Option<Command>> {
        let mut delete_cmd_stmt = self
            .c
            .prepare("DELETE FROM cxd_cmd WHERE id = ?1 RETURNING *")?;
        let mut rows = delete_cmd_stmt.query([id])?;
        Ok(self.assemble(&mut rows)?.pop())
    }

    /// Fetches all commands in the database
    pub fn fetch_all(&self) -> Result<Vec<Command>> {
        let mut command_stmt = self.c.prepare("SELECT * FROM cxd_cmd")?;
        let mut rows = command_stmt.query([])?;
        self.assemble(&mut rows)
    }

    /// Assembles a list of row objects into a list of Command objects.
    /// Performs subqueries to fetch rows with a FK to the suppled row.
    fn assemble(&self, rows: &mut rusqlite::Rows<'_>) -> Result<Vec<Command>> {
        let mut args_stmt = self.c.prepare("SELECT * FROM cxd_arg WHERE cmd_id = ?1")?;
        let mut envs_stmt = self.c.prepare("SELECT * FROM cxd_env WHERE cmd_id = ?1")?;

        let mut ret = vec![];
        while let Some(row) = rows.next()? {
            let cmd_row = CmdRow::try_from(row)?;

            // Fetching associated args
            let mut args = vec![];
            let mut rows = args_stmt.query([cmd_row.id])?;
            while let Some(row) = rows.next()? {
                args.push(ArgRow::try_from(row)?);
            }

            // Fetching associated envs
            let mut envs = vec![];
            let mut rows = envs_stmt.query([cmd_row.id])?;
            while let Some(row) = rows.next()? {
                envs.push(EnvRow::try_from(row)?);
            }
            ret.push(Command::new(cmd_row, args, envs));
        }
        Ok(ret)
    }
}
