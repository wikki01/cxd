#![cfg_attr(rustfmt, rustfmt_skip)]

use const_format::{concatcp, str_replace};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const USAGE: &str = "cxd [OPTIONS] <NAME>|<OPERATION>";

const HELP_ARG_DESC: &str = "Show this help message";
const HELP_OP_ARG_DESC: &str = "Show a help message for this operation";
const VERSION_ARG_DESC: &str = "Show the version string";

const FILE_DESC: &str = "File to use as the backing command cache";
const FILE_LONG_USAGE: &str = "-f, --file";
const FILE_LONG_HELP: &str = concatcp!(FILE_DESC, r#"

Defaults to first of: $CXD_CACHE_DIR/cxd.cache, $XDG_CACHE_HOME/cxd.cache, $HOME/.cache/cxd.cache
"#);

const ADD_DESC: &str = "Add a new command to the database";
pub const ADD_LONG_USAGE: &str = "-a, --add [OPTIONS] <NAME> <CMD> [ARG]...";
pub const ADD_LONG_HELP: &str = concatcp!(ADD_DESC, r#"

Arguments:
  <NAME>             Name of command to add, must be unique to database
  <CMD>              Executable to run, may be bare name within $PATH, or absolute path.
  [ARG]              One or more arguments to CMD

Add Options:
  -c, --cwd          Save CWD as command's working directory
  -d, --dir DIR      Save DIR as command's working directory
  -e, --env ENV=VAL  Save an env variable to the command's environment
  -h, --help         "#, HELP_OP_ARG_DESC, r#"
  --version          "#, VERSION_ARG_DESC, r#"
"#);

const REMOVE_DESC: &str = "Remove a command from the database";
pub const REMOVE_LONG_USAGE: &str = "-r, --remove [OPTIONS] <COMMAND>";
pub const REMOVE_LONG_HELP: &str = concatcp!(REMOVE_DESC, r#"

Arguments:
  <COMMAND>          Selector for command, by default this is the command name

Remove Options:
  -i, --id ID        Interpret SELECTOR as the command's internal ID
  -h, --help         "#, HELP_OP_ARG_DESC, r#"
  --version          "#, VERSION_ARG_DESC, r#"
"#);

pub const LIST_DESC: &str = "List available commands";
pub const LIST_LONG_USAGE: &str = "-l, --list [OPTIONS]";
pub const LIST_LONG_HELP: &str = concatcp!(LIST_DESC, r#"

List Options:
  -s, --short        Short output -- name only
  -h, --help         "#, HELP_OP_ARG_DESC, r#"
  --version          "#, VERSION_ARG_DESC, r#"
"#);

pub const CLEAR_LONG_USAGE: &str = "--clear";
pub const CLEAR_DESC: &str = "Clear all commands from the database";
pub const CLEAR_LONG_HELP: &str = concatcp!(CLEAR_DESC, r#"

Clear Options:
  -h, --help         "#, HELP_OP_ARG_DESC, r#"
  --version          "#, VERSION_ARG_DESC, r#"
"#);

pub const LONG_HELP: &str = concatcp!(
r#"Usage: "#, USAGE, r#"
Arguments:
  <NAME>   Name of command to execute

Options:
  "#, FILE_LONG_USAGE, r#"
      "#, str_replace!(FILE_LONG_HELP, "\n", "\n      "), r#"

  -h
      Show the short version of this help message

  --help
      "#, HELP_ARG_DESC, r#"

  --version      
      "#, VERSION_ARG_DESC, r#"

Operations:
  "#, ADD_LONG_USAGE, r#"
      "#, str_replace!(ADD_LONG_HELP, "\n", "\n      "), r#"

  "#, REMOVE_LONG_USAGE, r#"
      "#, str_replace!(REMOVE_LONG_HELP, "\n", "\n      "), r#"

  "#, LIST_LONG_USAGE, r#"
      "#, str_replace!(LIST_LONG_HELP, "\n", "\n      "), r#"

  "#, CLEAR_LONG_USAGE, r#"
      "#, str_replace!(CLEAR_LONG_HELP, "\n", "\n      "), r#"
"#);

pub const SHORT_HELP: &str = concatcp!(
r#"Usage: "#, USAGE, r#"

Arguments:
  <NAME>   Name of command to execute

Options:
  -f, --file <FILE>                "#, FILE_DESC, r#"
  -h                               "#, HELP_ARG_DESC, r#"
  --help                           Show the long version of this help message
  --version                        "#, VERSION_ARG_DESC, r#"

Operations:
  -a, --add <NAME> <CMD> [ARG]...  "#, ADD_DESC, r#"
  -r, --remove <COMMAND>           "#, REMOVE_DESC, r#"
  -l, --list                       "#, LIST_DESC, r#"
  --clear                          "#, CLEAR_DESC, r#"
"#);
