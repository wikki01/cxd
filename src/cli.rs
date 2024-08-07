use std::fmt::Display;

use crate::error::{CxdError, Result};

mod defines;

pub fn print_version() {
    // Per GNU standards, the version-proper should be after the last space
    println!("cxd {}", defines::VERSION);
}

pub fn print_long_help() {
    print!("{}", defines::LONG_HELP);
}

pub fn print_short_help() {
    print!("{}", defines::SHORT_HELP);
}

pub fn print_op_help(op: Op) {
    use defines::*;
    let help = match op {
        Op::Add => ADD_LONG_HELP,
        Op::Remove => REMOVE_LONG_HELP,
        Op::List => LIST_LONG_HELP,
        Op::Clear => CLEAR_LONG_HELP,
    };
    print_op_usage(op);
    print!("{}", help);
}

pub fn print_op_usage(op: Op) {
    use defines::*;
    let usage = match op {
        Op::Add => ADD_LONG_USAGE,
        Op::Remove => REMOVE_LONG_USAGE,
        Op::List => LIST_LONG_USAGE,
        Op::Clear => CLEAR_LONG_USAGE,
    };
    print!("Usage: cxd {}\n", usage);
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Remove,
    List,
    Clear,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Op::*;
        let s = match self {
            Add => "-a|--add",
            Remove => "-r|--remove",
            List => "-l|--list",
            Clear => "-c|--clear",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum HelpType {
    Short,
    Long,
}

#[derive(Debug, Default)]
pub struct CxdArgs {
    pub file: Option<String>,
    pub op: Option<Op>,
    pub op_args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub cwd: bool,
    pub dir: Option<String>,
    pub id: bool,
    pub help: Option<HelpType>,
    pub version: bool,
}

/// This function is before handing off the parsing to `pico_args`. Add (`--add`) has a special
/// property where it must slurp arbitrary arguments, without colliding with `cxd`'s arguments.
///
/// For example, `cxd --add ls_help ls --help`. We need to capture ["ls", "--help"] without
/// interpreting `--help` as an option for `cxd`.
///
/// This looks through the arguments, and if `--add` is specified, returns the arg position of the
/// first argument to `--add`, `<NAME>`.
pub fn find_add_args() -> Option<usize> {
    // Add is greedy, and pico-args doesn't like that much

    let mut raw_args = std::env::args();
    let mut last = false;
    let mut skip_next = false;
    raw_args
        .position(|a| a == "-a" || a == "--add")
        .and_then(|p| {
            raw_args
                .position(|a| {
                    if last {
                        true
                    } else if skip_next {
                        skip_next = false;
                        false
                    } else if a == "--env" || a == "-e" || a == "--dir" || a == "-d" {
                        skip_next = true;
                        false
                    } else if a == "--" {
                        last = true; // Need to move one forward
                        false
                    } else {
                        !a.starts_with("-")
                    }
                })
                .map(|a| a + p)
        })
}

/// Parse CLI arguments into loosely validated struct
///
/// Invalid flag dependencies and conflicts are returned as `Err`
pub fn parse_args() -> Result<CxdArgs> {
    let mut raw_args: Vec<_> = std::env::args_os().collect();
    let mut args = CxdArgs::default();
    let mut trunc = None;
    if let Some(i) = find_add_args() {
        // Add is greedy, so we need to protect pico_args
        trunc = Some(raw_args.split_off(i + 1));
    }
    raw_args.remove(0); // Remove $0
    let mut pargs = pico_args::Arguments::from_vec(raw_args);
    // Parsing top level flags
    if pargs.contains("-h") {
        args.help = Some(HelpType::Short);
    }
    if pargs.contains("--help") {
        args.help = Some(HelpType::Long);
    }
    if pargs.contains("--version") {
        args.version = true;
    }
    if let Some(path) = pargs.opt_value_from_str(["-f", "--file"])? {
        args.file = Some(path);
    }
    // Parsing operation
    if pargs.contains(["-a", "--add"]) {
        args.op = Some(Op::Add);
    }
    if pargs.contains(["-r", "--remove"]) {
        let old = args.op.replace(Op::Remove);
        if let Some(old) = old {
            print_short_help();
            println!();
            return Err(CxdError::IncompatibleOperations(Op::Remove, old));
        }
    }
    if pargs.contains(["-l", "--list"]) {
        let old = args.op.replace(Op::List);
        if let Some(old) = old {
            print_short_help();
            println!();
            return Err(CxdError::IncompatibleOperations(Op::List, old));
        }
    }
    if pargs.contains("--clear") {
        let old = args.op.replace(Op::Clear);
        if let Some(old) = old {
            print_short_help();
            println!();
            return Err(CxdError::IncompatibleOperations(Op::Clear, old));
        }
    }

    // Add-specific flags
    if pargs.contains(["-c", "--cwd"]) {
        if args.op != Some(Op::Add) {
            return Err(CxdError::OptionRequires {
                name: "-c, --cwd".into(),
                requires: "-a, --add".into(),
            });
        }
    }
    args.cwd = pargs.contains(["-c", "--cwd"]);
    if let Some(path) = pargs.opt_value_from_str(["-d", "--dir"])? {
        if args.cwd {
            return Err(CxdError::OptionsIncompatible(
                "-d, --dir".into(),
                "-c, --cwd".into(),
            ));
        } else if args.op != Some(Op::Add) {
            return Err(CxdError::OptionRequires {
                name: "-d, --dir".into(),
                requires: "-a, --add".into(),
            });
        }
        args.dir = Some(path);
    }
    while let Some(pair) = pargs.opt_value_from_str::<_, String>(["-e", "--env"])? {
        if args.op != Some(Op::Add) {
            return Err(CxdError::OptionRequires {
                name: "-e, --env".into(),
                requires: "-a, --add".into(),
            });
        }
        match pair.split_once('=') {
            Some((k, v)) => args.env.push((k.to_owned(), v.to_owned())),
            None => {
                return Err(CxdError::ArgumentParse {
                    arg: pair,
                    reason: "<KEY>=<VALUE>".into(),
                })
            }
        }
    }

    // Remove-specific arguments
    if pargs.contains(["-i", "--id"]) {
        if args.op != Some(Op::Remove) {
            return Err(CxdError::OptionRequires {
                name: "-i, --id".into(),
                requires: "-r, --remove".into(),
            });
        }
        args.id = true;
    }

    for arg in pargs.finish() {
        args.op_args.push(arg.to_string_lossy().into());
    }

    if let Some(Op::Add) = &mut args.op {
        if args.op_args.len() > 0 {
            return Err(CxdError::ArgumentParse {
                arg: args.op_args.join(" "),
                reason: "unexpected argument".into(),
            })?;
        }
        // Adding 'add' arguments since we chopped them off at the beginning
        for arg in trunc.unwrap_or_default() {
            args.op_args.push(arg.to_string_lossy().into());
        }
    }
    Ok(args)
}
