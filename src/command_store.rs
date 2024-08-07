use std::path::{Path, PathBuf};

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

pub struct CommandStore {
    c: Connection,
}

impl CommandStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let c = Connection::open(path)?;
        // Enable foreign key support
        c.execute("PRAGMA foreign_keys = ON", ())?;
        CmdRow::init(&c)?;
        ArgRow::init(&c)?;
        EnvRow::init(&c)?;
        Ok(Self { c })
    }

    pub fn insert(&self, cmd: &Command) -> Result<Option<i64>> {
        // Creating command entry
        let mut command_stmt = self.c.prepare(
            "INSERT INTO command (name, command, dir) VALUES (?1, ?2, ?3) RETURNING (id)",
        )?;
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
            Ok(None) => Err(CxdError::CommandNotFound(cmd.name.clone()))?,
        };

        // Creating args
        let mut args_stmt = self
            .c
            .prepare("INSERT INTO arg (data, command_id) VALUES (?1, ?2)")?;
        for arg in &cmd.args {
            args_stmt.execute((arg, id))?;
        }

        // Creating envs
        let mut envs_stmt = self
            .c
            .prepare("INSERT INTO env (key, value, command_id) VALUES (?1, ?2, ?3)")?;
        for env in &cmd.envs {
            envs_stmt.execute((&env.0, &env.1, id))?;
        }
        Ok(Some(id))
    }

    pub fn get_by_name(&self, name: &str) -> Result<Option<Command>> {
        let mut command_stmt = self.c.prepare("SELECT * FROM command WHERE name = ?1")?;
        let mut rows = command_stmt.query([name])?;
        Ok(self.assemble(&mut rows)?.pop())
    }

    pub fn delete_by_name(&self, name: &str) -> Result<Option<Command>> {
        let mut delete_cmd_stmt = self
            .c
            .prepare("DELETE FROM command WHERE name = ?1 RETURNING *")?;
        let mut rows = delete_cmd_stmt.query([name])?;
        Ok(self.assemble(&mut rows)?.pop())
    }

    pub fn delete_by_id(&self, id: i64) -> Result<Option<Command>> {
        let mut delete_cmd_stmt = self
            .c
            .prepare("DELETE FROM command WHERE id = ?1 RETURNING *")?;
        let mut rows = delete_cmd_stmt.query([id])?;
        Ok(self.assemble(&mut rows)?.pop())
    }

    pub fn fetch_all(&self) -> Result<Vec<Command>> {
        let mut command_stmt = self.c.prepare("SELECT * FROM command")?;
        let mut rows = command_stmt.query([])?;
        self.assemble(&mut rows)
    }

    /// Assembles a list of row objects into a list of Command objects.
    /// Performs subqueries to fetch rows with a FK to the suppled row.
    fn assemble(&self, rows: &mut rusqlite::Rows<'_>) -> Result<Vec<Command>> {
        let mut args_stmt = self.c.prepare("SELECT * FROM arg WHERE command_id = ?1")?;
        let mut envs_stmt = self.c.prepare("SELECT * FROM env WHERE command_id = ?1")?;

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
