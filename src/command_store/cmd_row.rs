pub struct CmdRow {
    pub id: i64,
    pub name: String,
    pub command: String,
    pub dir: String,
}

impl CmdRow {
    pub fn init(c: &rusqlite::Connection) -> rusqlite::Result<()> {
        c.execute(
            r#"
            CREATE TABLE IF NOT EXISTS command (
                id      INTEGER PRIMARY KEY AUTOINCREMENT,
                name    TEXT NOT NULL,
                command TEXT NOT NULL,
                dir     TEXT NOT NULL,
                UNIQUE(id)
                UNIQUE(name)
            )
        "#,
            (),
        )?;
        Ok(())
    }
}

impl<'a> TryFrom<&rusqlite::Row<'a>> for CmdRow {
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
