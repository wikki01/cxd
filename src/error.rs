use thiserror::Error;

use crate::cli::Op;

pub type Result<T, E = CxdError> = core::result::Result<T, E>;

#[derive(Error)]
pub enum CxdError {
    #[error("{0}")]
    Env(String),
    #[error("failed to parse cli args: {0}")]
    CliParse(#[from] pico_args::Error),
    #[error("operations {0} and {1} are incompatible")]
    IncompatibleOperations(Op, Op),
    #[error("options {0} and {1} are incompatible")]
    IncompatibleOptions(String, String),
    #[error("option {0} requires operation {1}")]
    RequiresOption(String, String),
    #[error("{0} requires {1} arguments, found {2}")]
    WrongArgumentCount(String, usize, usize),
    #[error("invalid argument \"{0}\", expected {1}")]
    InvalidArgument(String, String),
    #[error("command already exists: \"{0}\"")]
    CommandExists(String),
    #[error("command not found: \"{0}\"")]
    CommandNotFound(String),
    #[error("failed to read from stdin")]
    Stdin,
    #[error("io error: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("sql error: {0:?}")]
    Sql(#[from] rusqlite::Error),
}

impl std::fmt::Debug for CxdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}
