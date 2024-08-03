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
        Op::Exec => ("<NAME>", "  Executes a saved command\n"),
        Op::Add => (ADD_LONG_USAGE, ADD_LONG_HELP),
        Op::Remove => (REMOVE_LONG_USAGE, REMOVE_LONG_HELP),
        Op::List => (LIST_LONG_USAGE, LIST_LONG_HELP),
        Op::Clear => (CLEAR_LONG_USAGE, CLEAR_LONG_HELP),
    };
    print!("Usage: cxd {}\n{}", usage, help);
}

fn print_add_help() {
    print!(
        "Usage: cxd {}\n{}",
        defines::ADD_LONG_USAGE,
        defines::ADD_LONG_HELP
    );
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Exec,
    Add,
    Remove,
    List,
    Clear,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Op::*;
        let s = match self {
            Exec => "exec",
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
    }
    args.remove(0); // Remove $0
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
        if let Some(Op::Add) = args.op {
            print_add_help();
            anyhow::bail!("Option -d, --dir requires operation -a, --add");
        } else if args.cwd {
            print_add_help();
            anyhow::bail!("Options -d, --dir and -c, --cwd are incompatible");
        }
        args.dir = Some(path);
    }
    while let Some(pair) = pargs.opt_value_from_str::<_, String>(["-e", "--env"])? {
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

    // Default to Exec if no op specified
    if args.op.is_none() {
        args.op.insert(Op::Exec);
    }

    for arg in pargs.finish() {
        args.op_args.push(arg.to_string_lossy().into());
    }
    Ok(args)
}
