use thiserror::Error;

use crate::cli::Op;

pub type Result<T, E = CxdError> = core::result::Result<T, E>;

#[derive(Error)]
pub enum CxdError {
    #[error("no suitable path found for cache file")]
    CachePath,

    #[error("failed to parse cli args: {0}")]
    CliParse(#[from] pico_args::Error),

    #[error("operations {0} and {1} are incompatible")]
    IncompatibleOperations(Op, Op),

    #[error("options {0} and {1} are incompatible")]
    OptionsIncompatible(String, String),

    #[error("option {name} requires operation {requires}")]
    OptionRequires { name: String, requires: String },

    #[error("{name} requires {requires} arguments, found {found}")]
    WrongArgumentCount {
        name: String,
        requires: usize,
        found: usize,
    },

    #[error("failed to parse argument \"{arg}\": {reason}")]
    ArgumentParse { arg: String, reason: String },

    #[error("command already exists: \"{0}\"")]
    CommandExists(String),

    #[error("command not found: \"{0}\"")]
    CommandNotFound(String),

    #[error("failed to read from stdin")]
    Stdin,

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("exec {0}: {1}")]
    Exec(String, std::io::Error),

    #[error("sql: {0}")]
    Sql(#[from] rusqlite::Error),
}

impl std::fmt::Debug for CxdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}
