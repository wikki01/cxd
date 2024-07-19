pub struct ArgRow {
    pub id: i64,
    pub command_id: i64,
    pub data: String,
}

impl ArgRow {
    pub fn init(c: &rusqlite::Connection) -> rusqlite::Result<()> {
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

impl<'a> TryFrom<&rusqlite::Row<'a>> for ArgRow {
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
