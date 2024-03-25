use crate::command_store::{ArgTable, CommandTable};

#[derive(Debug)]
pub struct Command {
    pub id: i64,
    pub name: String,
    pub command: String,
    // Due to Sqlite not considering NULL as unique, an empty string here signifies None
    pub dir: String,
    pub args: Vec<String>
}

impl Command {
    pub fn exec(mut self) -> anyhow::Result<()> {
        if self.dir.len() != 0 {
            std::env::set_current_dir(self.dir)?;
        }
        // execvp requires program name to be first arg too
        self.args.insert(0, self.command.clone());
        Err(exec::execvp(self.command, self.args))?
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Command: {}", self.name)?; 
        writeln!(f, "\texec: {} {}", self.command, self.args.join(" "))?;
        if self.dir.len() == 0 {
            writeln!(f, "\tscope: Global")?; 
        } else {
            writeln!(f, "\tscope: {}", self.dir)?; 
        }
        Ok(())
    }
}

impl From<(CommandTable, Vec<ArgTable>)> for Command {
    fn from((cmd_row, arg_rows): (CommandTable, Vec<ArgTable>)) -> Self {
        Self {
            id: cmd_row.id,
            name: cmd_row.name,
            command: cmd_row.command,
            dir: cmd_row.dir,
            args: arg_rows.into_iter().map(|a| a.data).collect()
        } 
    }
}