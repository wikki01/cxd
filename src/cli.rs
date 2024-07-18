use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    /// Set the path to the store database file. Defaults to
    #[arg(long, short)]
    pub file: Option<String>,

    /// Subcommand
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
#[command(after_long_help = r#"The default matching order for commands:
   1. Command matches NAME and CWD
   2. Command matches NAME and only one exists
   3. Allow user to select from all commands matching NAME
"#)]
pub enum CliCommand {
    /// Push a new command to the store
    Push {
        /// Push as a global command
        #[arg(long, short)]
        global: bool,

        /// Push using dir as reference point rather than CWD
        #[arg(long, short, conflicts_with = "global")]
        dir: Option<PathBuf>,

        /// Name to associate with command
        name: String,
        /// Executable to run, can be bare name within $PATH, or absolute path
        command: String,
        /// Args of command, split with -- to prevent argument expansion
        args: Vec<String>,
    },
    /// Pop a command from the store
    Pop {
        /// Only pop global command matching NAME
        #[arg(long, short)]
        global: bool,

        /// Only pop global command matching NAME under CWD
        #[arg(long, short, conflicts_with = "global")]
        cwd: bool,

        /// Only pop command matching NAME under DIR
        #[arg(long, short, conflicts_with = "global", conflicts_with = "cwd")]
        dir: Option<PathBuf>,

        /// Name of command to pop
        name: String,
    },
    /// Execute a command in the store
    Exec {
        /// Only exec global command matching NAME
        #[arg(long, short)]
        global: bool,

        /// Only exec global command matching NAME under CWD
        #[arg(long, short, conflicts_with = "global")]
        cwd: bool,

        /// Only exec command matching NAME under DIR
        #[arg(long, short, conflicts_with = "global", conflicts_with = "cwd")]
        dir: Option<PathBuf>,

        /// Name of command to execute
        name: String,
    },
    /// List available commands
    List,
    /// Clear all commands from store
    Clear,
}
