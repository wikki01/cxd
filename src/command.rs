use std::{os::unix::process::CommandExt, path::PathBuf};

use crate::{
    command_store::{ArgRow, CmdRow, EnvRow},
    error::{CxdError, Result},
};

#[derive(Debug)]
pub struct Command {
    pub id: i64,
    pub name: String,
    pub command: String,
    // Due to Sqlite not considering NULL as unique, an empty string here signifies None
    pub dir: PathBuf,
    pub args: Vec<String>,
    pub envs: Vec<(String, String)>,
}

impl Command {
    pub fn new(cmd_row: CmdRow, arg_rows: Vec<ArgRow>, env_rows: Vec<EnvRow>) -> Self {
        Self {
            id: cmd_row.id,
            name: cmd_row.name,
            command: cmd_row.cmd,
            dir: cmd_row.dir.into(),
            args: arg_rows.into_iter().map(|a| a.data).collect(),
            envs: env_rows.into_iter().map(|a| (a.key, a.value)).collect(),
        }
    }

    pub fn exec(self) -> Result<()> {
        if self.dir.components().next().is_some() {
            std::env::set_current_dir(self.dir)
                .map_err(|e| CxdError::Exec(self.name.clone(), e))?;
        }
        // execvp requires program name to be first arg too
        Err(CxdError::Exec(
            self.name,
            std::process::Command::new(self.command.clone())
                .args(self.args)
                .envs(self.envs)
                .exec(),
        ))
    }
}

impl std::fmt::Display for Command {
    /// Command formatting - the plus (`+`) flag can be used to display the ID.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{}]: {}", self.id, self.name)?;
        if self.envs.len() == 1 {
            writeln!(f, "  env: {}={}", self.envs[0].0, self.envs[0].1)?;
        } else if self.envs.len() > 1 {
            writeln!(f, "  env:")?;
            for (k, v) in self.envs.iter() {
                writeln!(f, "    {k}={v}")?;
            }
        }
        if self.dir.components().next().is_some() {
            writeln!(f, "  dir: {}", self.dir.to_str().unwrap_or("invalid path"))?;
        }
        write!(f, "  cmd: {} {}", self.command, self.args.join(" "))?;
        Ok(())
    }
}
