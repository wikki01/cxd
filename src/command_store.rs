use std::path::{Path, PathBuf};

use crate::command::Command;
use rusqlite::{ffi::Error, Connection, ErrorCode};

pub struct ArgTable {
    pub id: i64,
    pub command_id: i64,
    pub data: String,
}

impl ArgTable {
    fn create(c: &Connection) -> rusqlite::Result<()> {
        c.execute(
            r#"
            CREATE TABLE IF NOT EXISTS arg (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                command_id  INTEGER NOT NULL,
                data        TEXT NOT NULL,
                UNIQUE(id)
                FOREIGN KEY(command_id) REFERENCES command(id)
                ON DELETE CASCADE ON UPDATE CASCADE
            )
        "#,
            (),
        )?;
        Ok(())
    }
}

impl<'a> TryFrom<&rusqlite::Row<'a>> for ArgTable {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'a>) -> Result<Self, Self::Error> {
        let id: i64 = row.get("id")?;
        let command_id: i64 = row.get("command_id")?;
        let data: String = row.get("data")?;
        Ok(Self {
            id,
            command_id,
            data,
        })
    }
}
pub struct CommandTable {
    pub id: i64,
    pub name: String,
    pub command: String,
    pub dir: String,
}

impl CommandTable {
    fn create(c: &Connection) -> rusqlite::Result<()> {
        c.execute(
            r#"
            CREATE TABLE IF NOT EXISTS command (
                id      INTEGER PRIMARY KEY AUTOINCREMENT,
                name    TEXT NOT NULL,
                command TEXT NOT NULL,
                dir     TEXT NOT NULL,
                UNIQUE(id)
                UNIQUE(name, dir)
            )
        "#,
            (),
        )?;
        Ok(())
    }
}

impl<'a> TryFrom<&rusqlite::Row<'a>> for CommandTable {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'a>) -> Result<Self, Self::Error> {
        let id: i64 = row.get("id")?;
        let name: String = row.get("name")?;
        let command: String = row.get("command")?;
        let dir: String = row.get("dir")?;
        Ok(Self {
            id,
            name,
            command,
            dir,
        })
    }
}

pub struct CommandStore {
    c: Connection,
}

impl CommandStore {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let c = Connection::open(path)?;
        // Enable foreign key support
        c.execute("PRAGMA foreign_keys = ON", ())?;
        CommandTable::create(&c)?;
        ArgTable::create(&c)?;
        Ok(Self { c })
    }

    pub fn insert(&self, cmd: Command) -> anyhow::Result<bool> {
        // Creating command entry
        let mut command_stmt = self
            .c
            .prepare("INSERT INTO command (name, command, dir) VALUES (?1, ?2, ?3) RETURNING *")?;
        let mut result =
            command_stmt.query((cmd.name, cmd.command, cmd.dir.to_str().unwrap_or_default()))?;
        let command_row: CommandTable = match result.next() {
            Ok(Some(row)) => CommandTable::try_from(row)?,
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
            let cmd_row = CommandTable::try_from(row)?;

            // Fetching associated args
            let mut args = vec![];
            let mut rows = args_stmt.query([cmd_row.id])?;
            while let Some(row) = rows.next()? {
                args.push(ArgTable::try_from(row)?);
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
            let cmd_row = CommandTable::try_from(row)?;

            // Fetching associated args
            let mut args = vec![];
            let mut rows = args_stmt.query([cmd_row.id])?;
            while let Some(row) = rows.next()? {
                args.push(ArgTable::try_from(row)?);
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
            let cmd_row = CommandTable::try_from(row)?;

            // Fetching associated args
            let mut args = vec![];
            let mut rows = args_stmt.query([cmd_row.id])?;
            while let Some(row) = rows.next()? {
                args.push(ArgTable::try_from(row)?);
            }
            ret.push(Command::from((cmd_row, args)));
        }
        Ok(ret)
    }
}
