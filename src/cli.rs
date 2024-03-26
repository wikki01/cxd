use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    /// Only use global named commands, rather than dynamically dispatching via cwd
    #[arg(long, short)]
    pub global: bool,

    /// Only use commands registered in cwd (or the value of --dir rather if specified)
    #[arg(long, short)]
    pub cwd: bool,

    /// Use dir rather than cwd to dispatch
    #[arg(long, short, conflicts_with = "global")]
    pub dir: Option<PathBuf>,

    /// Set the path to the store database file. Defaults to 
    #[arg(long, short)]
    pub file: Option<String>,

    /// Subcommand
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Push a new command to the store
    Push {
        /// Name to associate with command
        name: String,
        /// Executable to run, can be bare name within $PATH, or absolute path
        command: String,
        /// Args of command, split with -- to prevent argument expansion
        args: Vec<String>,
    },
    /// Pop a command from the store
    Pop {
        /// Name of command to pop
        name: String
    },
    /// Execute a command in the store
    Exec {
        /// Name of command to execute
        name: String
    },
    /// List available commands
    List,
}
