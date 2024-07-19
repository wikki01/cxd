use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    /// Set the path to the store database file
    ///
    /// Defaults to first of: $XDG_CACHE_HOME/cxd.cache, $HOME/.cache/cxd.cache
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
    /// Add a new command to the store.
    ///
    /// By default, sets the command's working directory to CWD.
    Add {
        /// Add as a global command without associating a specific working directory
        #[arg(long, short)]
        global: bool,

        /// Add using DIR as reference point rather than CWD
        #[arg(long, short, conflicts_with = "global")]
        dir: Option<PathBuf>,

        /// Add an environment variable to the command. May be specified multiple times.
        #[arg(long, short, value_parser = parse_key_value::<String, String>, number_of_values = 1, value_name = "KEY>=<VALUE")]
        env: Vec<(String, String)>,

        /// Name to associate with command
        name: String,
        /// Executable to run, can be bare name within $PATH, or absolute path
        command: String,
        /// Args of command
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Remove a command from the store
    Remove {
        /// Search only global commands
        #[arg(long, short)]
        global: bool,

        /// Search only commands with the CWD registered as their working directory
        #[arg(long, short, conflicts_with = "global")]
        cwd: bool,

        /// Search only commands with DIR registered as their working directory
        #[arg(long, short, conflicts_with = "global", conflicts_with = "cwd")]
        dir: Option<PathBuf>,

        /// Remove a command by a specific internal ID.
        #[arg(long, short, conflicts_with_all = ["dir", "cwd", "global", "name"])]
        id: Option<i64>,

        /// Name of command to remove. Required unless -i/--id specified
        #[arg(required_unless_present = "id")]
        name: Option<String>,
    },
    /// Execute a command in the store
    Exec {
        /// Search only global commands
        #[arg(long, short)]
        global: bool,

        /// Search only commands with the CWD registered as their working directory
        #[arg(long, short, conflicts_with = "global")]
        cwd: bool,

        /// Search only commands with DIR registered as their working directory
        #[arg(long, short, conflicts_with = "global", conflicts_with = "cwd")]
        dir: Option<PathBuf>,

        /// Name of command to execute
        name: String,
    },
    /// List available commands
    List {
        /// Show the internal IDs of each command
        #[arg(long, short)]
        id: bool,
    },
    /// Clear all commands from store
    Clear,
}

/// Parse a single key-value pair
/// "Borrowed" from https://github.com/clap-rs/clap/blob/master/examples/typed-derive.rs
fn parse_key_value<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
