use std::fmt::Display;

mod defines;

pub fn print_long_help() {
    print!("{}", defines::LONG_HELP);
}

pub fn print_short_help() {
    print!("{}", defines::SHORT_HELP);
}

pub fn print_op_help(op: Op) {
    use defines::*;
    let (usage, help) = match op {
        Op::Add(_, _, _) => (ADD_LONG_USAGE, ADD_LONG_HELP),
        Op::Remove(_) => (REMOVE_LONG_USAGE, REMOVE_LONG_HELP),
        Op::List => (LIST_LONG_USAGE, LIST_LONG_HELP),
        Op::Clear => (CLEAR_LONG_USAGE, CLEAR_LONG_HELP),
    };
    print!("Usage: {}\n{}", usage, help);
}

fn print_add_help() {
    print!(
        "Usage: {}\n{}",
        defines::ADD_LONG_USAGE,
        defines::ADD_LONG_HELP
    );
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add(String, String, Vec<String>),
    Remove(String),
    List,
    Clear,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Op::*;
        let s = match self {
            Add(_, _, _) => "-a, --add",
            Remove(_) => "-r, --remove",
            List => "-l, --list",
            Clear => "-c, --clear",
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
    pub cwd: bool,
    pub dir: Option<String>,
    pub id: bool,
    pub help: Option<HelpType>,
}

pub fn find_add_args() -> Option<usize> {
    // Add is greedy, and pico-args doesn't like that much

    let mut raw_args = std::env::args();
    let mut requires_adjust = false;
    raw_args
        .position(|a| a == "-a" || a == "--add")
        .and_then(|p| {
            raw_args
                .position(|a| {
                    if a == "--" {
                        requires_adjust = true; // Need to move one forward
                        return false;
                    }
                    !a.starts_with("-") || requires_adjust
                })
                .map(|a| a + p)
        })
}

pub fn parse_args() -> anyhow::Result<CxdArgs> {
    let mut args: Vec<_> = std::env::args_os().collect();
    let mut trunc = None;
    if let Some(i) = find_add_args() {
        // Add is greedy, so we need to protect pico_args
        trunc = Some(args.split_off(i + 1));
        dbg!(&trunc);
    }
    let mut pargs = pico_args::Arguments::from_vec(args);
    let mut args = CxdArgs::default();
    // Parsing top level flags
    if pargs.contains("-h") {
        args.help = Some(HelpType::Short);
    }
    if pargs.contains("--help") {
        args.help = Some(HelpType::Long);
    }
    if let Some(path) = pargs.opt_value_from_str(["-f", "--file"])? {
        args.file = Some(path);
    }
    // Parsing operation
    if pargs.contains(["-a", "--add"]) {
        args.op = Some(Op::Add(String::new(), String::new(), vec![]));
    }
    if let Some(cmd) = pargs.opt_value_from_str(["-r", "--remove"])? {
        let old = args.op.replace(Op::Remove(cmd));
        if let Some(old) = old {
            print_short_help();
            anyhow::bail!(
                "Operation {} and {} are incompatible",
                Op::Remove(String::new()),
                old
            );
        }
    }
    if pargs.contains(["-l", "--list"]) {
        let old = args.op.replace(Op::List);
        if let Some(old) = old {
            print_short_help();
            anyhow::bail!("Operation {} and {} are incompatible", Op::List, old);
        }
    }
    if pargs.contains("--clear") {
        let old = args.op.replace(Op::Clear);
        if let Some(old) = old {
            print_short_help();
            anyhow::bail!("Operation {} and {} are incompatible", Op::Clear, old);
        }
    }

    // Add-specific flags
    args.cwd = pargs.contains(["-c", "--cwd"]);
    if let Some(path) = pargs.opt_value_from_str(["-d", "--dir"])? {
        if let Some(Op::Add(_, _, _)) = args.op {
            print_add_help();
            anyhow::bail!("Option -d, --dir requires operation -a, --add");
        } else if args.cwd {
            print_add_help();
            anyhow::bail!("Options -d, --dir and -c, --cwd are incompatible");
        }
        args.dir = Some(path);
    }
    if let Some(Op::Add(name, cmd, cmd_args)) = &mut args.op {
        let trunc = trunc.unwrap_or_default();
        if trunc.len() < 2 {
            print_add_help();
            anyhow::bail!(
                "Expected 2 arguments <NAME> and <CMD>, found {}",
                trunc.len()
            );
        }
        let trunc = trunc
            .into_iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        *name = trunc[0].clone();
        *cmd = trunc[1].clone();
        for arg in &trunc[2..] {
            cmd_args.push(arg.clone());
        }
    }

    // Id (shared between remove and list)
    args.id = pargs.contains(["-i", "--id"]);
    Ok(args)
}
