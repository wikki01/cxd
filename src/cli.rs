use std::process::{exit, ExitCode, ExitStatus};

mod defines;

fn print_short_help() {
    print!("{}", defines::SHORT_HELP);
}

fn print_long_help() {
    print!("{}", defines::LONG_HELP);
}

fn print_long_help_add() {
    eprintln!(
        "Usage: {}\n{}",
        defines::ADD_LONG_USAGE,
        defines::ADD_LONG_HELP
    );
}

#[derive(Debug)]
pub enum Ops {
    Add(Add),
    Remove,
}

#[derive(Debug)]
pub struct Add {
    pub name: String,
    pub cmd: String,
    pub args: Vec<String>,
}

pub fn try_parse_add() -> anyhow::Result<Option<(Add, usize)>> {
    let mut raw_args = std::env::args();
    if let Some(i) = raw_args.position(|a| a == "-a" || a == "--add") {
        // Add is greedy, and pico-args doesn't like that much
        dbg!(&raw_args);
        let mut iter = raw_args.into_iter();
        // Ignore until we get a -- or arg without -
        let mut name = None;
        for item in &mut iter {
            if item == "--" {
                name = iter.next().map(|i| i.to_owned());
                break;
            } else if !item.starts_with("-") {
                name = Some(item.to_owned());
                break;
            } else if item == "-h" || item == "--help" {
                print_long_help_add();
                anyhow::bail!(""); // TODO: We should catch this and exit successfully
            }
        }
        if let None = name {
            print_long_help_add();
            anyhow::bail!("Missing argument: <NAME>");
        }
        let name = name.unwrap();
        let cmd;
        if let Some(s) = iter.next() {
            cmd = s.to_owned();
        } else {
            print_long_help_add();
            anyhow::bail!("Missing argument: <CMD>");
        }
        let args: Vec<_> = iter.map(|i| i.to_owned()).collect();
        Ok(Some((Add { name, cmd, args }, i)))
    } else {
        Ok(None)
    }
}

pub fn parse_args() -> anyhow::Result<Option<()>> {
    let mut operation = None;
    let mut args: Vec<_> = std::env::args_os().collect();

    if let Some((add, i)) = try_parse_add()? {
        operation = Some(Ops::Add(add));
        args.truncate(i + 1);
    }

    let mut pargs = pico_args::Arguments::from_vec(args);

    if pargs.contains("-a") || pargs.contains("--add") {}

    if pargs.contains("-h") {
        print_short_help();
        return Ok(None);
    }

    if pargs.contains("--help") {
        print_long_help();
        return Ok(None);
    }

    dbg!(operation);

    Ok(Some(()))
}
