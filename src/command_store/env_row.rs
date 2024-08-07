#[allow(unused)]
pub struct EnvRow {
    pub id: i64,
    pub cmd_id: i64,
    pub key: String,
    pub value: String,
}

impl EnvRow {
    pub fn init(c: &rusqlite::Connection) -> rusqlite::Result<()> {
        c.execute(
            r#"
            CREATE TABLE IF NOT EXISTS cxd_env (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                cmd_id      INTEGER NOT NULL,
                key         TEXT NOT NULL,
                value       TEXT NOT NULL,
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

impl<'a> TryFrom<&rusqlite::Row<'a>> for EnvRow {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'a>) -> Result<Self, Self::Error> {
        let id: i64 = row.get("id")?;
        let cmd_id: i64 = row.get("cmd_id")?;
        let key: String = row.get("key")?;
        let value: String = row.get("value")?;
        Ok(Self {
            id,
            cmd_id,
            key,
            value,
        })
    }
}
