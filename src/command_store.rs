use std::path::{Path, PathBuf};

use crate::command::Command;
use rusqlite::{ffi::Error, Connection, ErrorCode};

mod arg_row;
mod cmd_row;

pub use arg_row::ArgRow;
pub use cmd_row::CmdRow;

pub struct CommandStore {
    c: Connection,
}

impl CommandStore {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let c = Connection::open(path)?;
        // Enable foreign key support
        c.execute("PRAGMA foreign_keys = ON", ())?;
        CmdRow::create(&c)?;
        ArgRow::create(&c)?;
        Ok(Self { c })
    }

    pub fn insert(&self, cmd: Command) -> anyhow::Result<bool> {
        // Creating command entry
        let mut command_stmt = self
            .c
            .prepare("INSERT INTO command (name, command, dir) VALUES (?1, ?2, ?3) RETURNING *")?;
        let mut result =
            command_stmt.query((cmd.name, cmd.command, cmd.dir.to_str().unwrap_or_default()))?;
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
        for arg in cmd.args {
            args_stmt.execute((arg, command_row.id))?;
        }
        Ok(true)
    }

    pub fn find_cmds_by_name(&self, name: &str) -> anyhow::Result<Vec<Command>> {
        let mut args_stmt = self.c.prepare("SELECT * FROM arg WHERE command_id = ?1")?;
        let mut command_stmt = self.c.prepare("SELECT * FROM command WHERE name = ?1")?;
        let mut rows = command_stmt.query([name])?;

        let mut ret = vec![];
        while let Some(row) = rows.next()? {
            let cmd_row = CmdRow::try_from(row)?;

            // Fetching associated args
            let mut args = vec![];
            let mut rows = args_stmt.query([cmd_row.id])?;
            while let Some(row) = rows.next()? {
                args.push(ArgRow::try_from(row)?);
            }
            ret.push(Command::from((cmd_row, args)));
        }
        Ok(ret)
    }

    pub fn find_cmd(&self, name: &str, dir: &PathBuf) -> anyhow::Result<Option<Command>> {
        let mut args_stmt = self.c.prepare("SELECT * FROM arg WHERE command_id = ?1")?;
        let mut command_stmt = self
            .c
            .prepare("SELECT * FROM command WHERE name = ?1 AND dir = ?2")?;
        let mut rows = command_stmt.query((name, dir.to_str().unwrap_or_default()))?;

        if let Some(row) = rows.next()? {
            let cmd_row = CmdRow::try_from(row)?;

            // Fetching associated args
            let mut args = vec![];
            let mut rows = args_stmt.query([cmd_row.id])?;
            while let Some(row) = rows.next()? {
                args.push(ArgRow::try_from(row)?);
            }
            Ok(Some(Command::from((cmd_row, args))))
        } else {
            Ok(None)
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
        let mut args_stmt = self.c.prepare("SELECT * FROM arg WHERE command_id = ?1")?;
        let mut command_stmt = self.c.prepare("SELECT * FROM command")?;
        let mut rows = command_stmt.query(())?;

        let mut ret = vec![];
        while let Some(row) = rows.next()? {
            let cmd_row = CmdRow::try_from(row)?;

            // Fetching associated args
            let mut args = vec![];
            let mut rows = args_stmt.query([cmd_row.id])?;
            while let Some(row) = rows.next()? {
                args.push(ArgRow::try_from(row)?);
            }
            ret.push(Command::from((cmd_row, args)));
        }
        Ok(ret)
    }
}
