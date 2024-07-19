pub struct EnvRow {
    pub id: i64,
    pub command_id: i64,
    pub key: String,
    pub value: String,
}

impl EnvRow {
    pub fn init(c: &rusqlite::Connection) -> rusqlite::Result<()> {
        c.execute(
            r#"
            CREATE TABLE IF NOT EXISTS env (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                command_id  INTEGER NOT NULL,
                key         TEXT NOT NULL,
                value       TEXT NOT NULL,
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

impl<'a> TryFrom<&rusqlite::Row<'a>> for EnvRow {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'a>) -> Result<Self, Self::Error> {
        let id: i64 = row.get("id")?;
        let command_id: i64 = row.get("command_id")?;
        let key: String = row.get("key")?;
        let value: String = row.get("value")?;
        Ok(Self {
            id,
            command_id,
            key,
            value,
        })
    }
}
