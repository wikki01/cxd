use std::path::{Path, PathBuf};

use crate::command::Command;
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
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let c = Connection::open(path)?;
        // Enable foreign key support
        c.execute("PRAGMA foreign_keys = ON", ())?;
        CmdRow::init(&c)?;
        ArgRow::init(&c)?;
        EnvRow::init(&c)?;
        Ok(Self { c })
    }

    pub fn insert(&self, cmd: &Command) -> anyhow::Result<bool> {
        // Creating command entry
        let mut command_stmt = self
            .c
            .prepare("INSERT INTO command (name, command, dir) VALUES (?1, ?2, ?3) RETURNING *")?;
        let mut result = command_stmt.query((
            &cmd.name,
            &cmd.command,
            cmd.dir.to_str().unwrap_or_default(),
        ))?;
        let command_row: CmdRow = match result.next() {
            Ok(Some(row)) => CmdRow::try_from(row)?,
            Err(rusqlite::Error::SqliteFailure(
                Error {
                    code: ErrorCode::ConstraintViolation,
                    extended_code: 2067,
                },
                _,
            )) => {
                // Already exists
                return Ok(false);
            }
            Err(e) => Err(anyhow::anyhow!("Insert failed: {:?}", e))?,
            _ => Err(anyhow::anyhow!("Insert failed: No rows created"))?,
        };

        // Creating args
        let mut args_stmt = self
            .c
            .prepare("INSERT INTO arg (data, command_id) VALUES (?1, ?2)")?;
        for arg in &cmd.args {
            args_stmt.execute((arg, command_row.id))?;
        }

        // Creating envs
        let mut envs_stmt = self
            .c
            .prepare("INSERT INTO env (key, value, command_id) VALUES (?1, ?2, ?3)")?;
        for env in &cmd.envs {
            envs_stmt.execute((&env.0, &env.1, command_row.id))?;
        }
        Ok(true)
    }

    pub fn find_cmds_by_name(&self, name: &str) -> anyhow::Result<Vec<Command>> {
        let mut command_stmt = self.c.prepare("SELECT * FROM command WHERE name = ?1")?;
        let mut rows = command_stmt.query([name])?;
        self.assemble(&mut rows)
    }

    pub fn find_cmd(&self, name: &str, dir: &PathBuf) -> anyhow::Result<Option<Command>> {
        let mut command_stmt = self
            .c
            .prepare("SELECT * FROM command WHERE name = ?1 AND dir = ?2")?;
        let mut rows = command_stmt.query((name, dir.to_str().unwrap_or_default()))?;

        let mut ret = self.assemble(&mut rows)?;
        if ret.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ret.swap_remove(0)))
        }
    }

    pub fn delete_by_id(&self, id: i64) -> anyhow::Result<bool> {
        let mut delete_cmd_stmt = self.c.prepare("DELETE FROM command WHERE id = ?1")?;
        let rows = delete_cmd_stmt.execute([id])?;

        if rows != 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn fetch_all(&self) -> anyhow::Result<Vec<Command>> {
        let mut command_stmt = self.c.prepare("SELECT * FROM command")?;
        let mut rows = command_stmt.query([])?;
        self.assemble(&mut rows)
    }

    /// Assembles a list of row objects into a list of Command objects.
    /// Performs subqueries to fetch rows with a FK to the suppled row.
    fn assemble(&self, rows: &mut rusqlite::Rows<'_>) -> anyhow::Result<Vec<Command>> {
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
