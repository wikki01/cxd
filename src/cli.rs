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
}

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

pub fn parse_args() -> anyhow::Result<CxdArgs> {
    let mut raw_args: Vec<_> = std::env::args_os().collect();
    let mut args = CxdArgs::default();
    let mut trunc = None;
    if let Some(i) = find_add_args() {
        // Add is greedy, so we need to protect pico_args
        trunc = Some(raw_args.split_off(i + 1));
        args.op = Some(Op::Add);
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
            anyhow::bail!("Operation {} and {} are incompatible", Op::Remove, old);
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
        if args.cwd {
            print_op_usage(Op::Add);
            anyhow::bail!("Options -d, --dir and -c, --cwd are incompatible");
        } else if args.op != Some(Op::Add) {
            print_op_usage(Op::Add);
            anyhow::bail!("Option -d, --dir requires operation -a, --add");
        }
        args.dir = Some(path);
    }
    while let Some(pair) = pargs.opt_value_from_str::<_, String>(["-e", "--env"])? {
        if args.op != Some(Op::Add) {
            print_op_usage(Op::Add);
            anyhow::bail!("Option -e, --env requires operation -a, --add");
        }
        match pair.split_once('=') {
            Some((k, v)) => args.env.push((k.to_owned(), v.to_owned())),
            None => anyhow::bail!("Failed to parse <KEY>=<VAL> pair: {}", pair),
        }
    }
    if let Some(Op::Add) = &mut args.op {
        // Adding 'add' arguments since we chopped them off at the beginning
        for arg in trunc.unwrap_or_default() {
            args.op_args.push(arg.to_string_lossy().into());
        }
    }

    // Id (shared between remove and list)
    args.id = pargs.contains(["-i", "--id"]);

    for arg in pargs.finish() {
        args.op_args.push(arg.to_string_lossy().into());
    }
    Ok(args)
}
