#[allow(unused)]
pub struct ArgRow {
    pub id: i64,
    pub cmd_id: i64,
    pub data: String,
}

impl ArgRow {
    pub fn init(c: &rusqlite::Connection) -> rusqlite::Result<()> {
        c.execute(
            r#"
            CREATE TABLE IF NOT EXISTS cxd_arg (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                cmd_id      INTEGER NOT NULL,
                data        TEXT NOT NULL,
                UNIQUE(id)
                FOREIGN KEY(cmd_id) REFERENCES cxd_cmd(id)
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
        let cmd_id: i64 = row.get("cmd_id")?;
        let data: String = row.get("data")?;
        Ok(Self { id, cmd_id, data })
    }
}
