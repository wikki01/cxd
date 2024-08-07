#![allow(unused)]
use std::{
    io::{BufRead, Write},
    path::PathBuf,
};

mod command;
use command::Command;

mod command_store;
use command_store::CommandStore;

mod cli;
use cli::{print_long_help, print_op_help, print_short_help, HelpType};

mod error;
use error::{CxdError, Result};

use crate::cli::Op;

fn main() -> Result<()> {
    let mut cli_args = cli::parse_args()?;
    match cli_args.help {
        Some(HelpType::Long) => {
            match cli_args.op {
                Some(op) => print_op_help(op),
                None => print_long_help(),
            }
            return Ok(());
        }
        Some(HelpType::Short) => {
            print_short_help();
            return Ok(());
        }
        _ => {}
    }

    let cache_file = cli_args
        .file
        .and_then(|s| Some(PathBuf::from(s)))
        .or(std::env::var("CXD_CACHE_DIR")
            .or(std::env::var("XDG_CACHE_HOME"))
            .ok()
            .and_then(|p| {
                if p.len() == 0 {
                    None
                } else {
                    Some(PathBuf::from(p).join("cxd.cache"))
                }
            }))
        .or(std::env::var("HOME").ok().and_then(|p| {
            if p.len() == 0 {
                None
            } else {
                Some(PathBuf::from(p).join(".cache").join("cxd.cache"))
            }
        }));

    if let None = cache_file {
        return Err(CxdError::CachePath);
    }
    let cache_file = cache_file.unwrap();

    let c = CommandStore::new(&cache_file)?;

    match cli_args.op {
        Some(Op::Add) => {
            if cli_args.op_args.len() < 2 {
                return Err(CxdError::WrongArgumentCount {
                    name: "add".into(),
                    requires: 2,
                    found: cli_args.op_args.len(),
                });
            }
            let name = cli_args.op_args[0].to_owned();
            let command = cli_args.op_args[1].to_owned();
            let mut args = cli_args.op_args.split_off(2);
            let mut dir = PathBuf::new();
            if cli_args.cwd {
                dir = std::env::current_dir()?;
            } else if let Some(d) = cli_args.dir {
                dir = d.into();
            }
            let cmd = Command {
                id: 0,
                name: name.clone(),
                command,
                args,
                envs: cli_args.env,
                dir,
            };
            if c.insert(&cmd)? {
                println!("Created command: {cmd}");
            } else {
                return Err(CxdError::CommandExists(name));
            }
        }
        Some(Op::Remove) => {
            if cli_args.op_args.len() != 1 {
                return Err(CxdError::WrongArgumentCount {
                    name: "remove".into(),
                    requires: 1,
                    found: cli_args.op_args.len(),
                });
            }
            let cmd = &cli_args.op_args[0];
            let res;
            if cli_args.id {
                res = c.delete_by_id(cmd.parse().map_err(|_| CxdError::ArgumentParse {
                    arg: cmd.into(),
                    reason: "not an integer".into(),
                })?)?;
            } else {
                res = c.delete_by_name(&cmd)?
            }
            if res {
                println!("Removed command {}", cmd);
            } else {
                println!("No matching command found, nothing was deleted");
            }
        }
        Some(Op::List) => {
            for cmd in c.fetch_all()? {
                println!("{}", cmd);
            }
        }
        Some(Op::Clear) => {
            print!("This will remove all saved commands from the store. Continue? [yn]: ");
            std::io::stdout().flush()?;
            let response = std::io::stdin()
                .lock()
                .lines()
                .next()
                .ok_or(CxdError::Stdin)??;
            if response.to_lowercase() == "y" {
                std::fs::remove_file(cache_file)?;
            }
        }
        // Indicates an execution operation
        None => {
            if cli_args.op_args.len() != 1 {
                return Err(CxdError::WrongArgumentCount {
                    name: "exec".into(),
                    requires: 1,
                    found: cli_args.op_args.len(),
                });
            }
            let cmd_name = &cli_args.op_args[0];
            let cmd = c.get_by_name(cmd_name)?;
            match cmd {
                Some(c) => c.exec()?,
                None => return Err(CxdError::CommandNotFound(cmd_name.into())),
            }
        }
    }

    Ok(())
}
