#![allow(unused)]
use anyhow::Context;
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

use crate::cli::Op;

fn main() -> anyhow::Result<()> {
    let cli_args = cli::parse_args()?;
    match cli_args.help {
        Some(HelpType::Long) => {
            match cli_args.op {
                Some(Op::Exec(_)) => print_long_help(),
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
        .or(std::env::var("XDG_CACHE_HOME").ok().and_then(|p| {
            if p.len() == 0 {
                None
            } else {
                Some(PathBuf::from(p))
            }
        }))
        .or(std::env::var("HOME").ok().and_then(|p| {
            if p.len() == 0 {
                None
            } else {
                Some(PathBuf::from(p).join(".cache"))
            }
        }))
        .and_then(|p| Some(p.join("cxd.cache")))
        .context("No suitable path found for cache file")?;

    let c = CommandStore::new(&cache_file)?;

    match cli_args.op.unwrap() {
        Op::Add(name, command, args) => {
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
                envs: vec![], // TODO: env,
                dir,
            };
            if c.insert(&cmd)? {
                println!("Created {cmd}");
            } else {
                Err(anyhow::anyhow!(
                    "Failed to create command {name}: already exists"
                ))?
            }
        }
        Op::Remove(cmd) => {
            let res;
            if cli_args.id {
                res = c.delete_by_id(cmd.parse().context("ID is not a number")?)?;
            } else {
                res = c.delete_by_name(&cmd)?
            }
            if res {
                println!("Deleted {}", cmd);
            } else {
                println!("No matching command found, nothing was deleted");
            }
        }
        Op::Exec(cmd_name) => {
            if let None = cmd_name {
                print_short_help();
                anyhow::bail!("Missing argument <NAME>");
            }
            let cmd_name = cmd_name.unwrap();
            let cmd = c.get_by_name(&cmd_name)?;
            match cmd {
                Some(c) => c.exec()?,
                None => anyhow::bail!("No command found: {}", cmd_name),
            }
        }
        Op::List => {
            if cli_args.id {
                for cmd in c.fetch_all()? {
                    println!("{:+}", cmd);
                }
            } else {
                for cmd in c.fetch_all()? {
                    println!("{}", cmd);
                }
            }
        }
        Op::Clear => {
            print!("This will remove all saved commands from the store. Continue? [yn]: ");
            std::io::stdout().flush()?;
            let response = std::io::stdin()
                .lock()
                .lines()
                .next()
                .context("Failed to read response from stdin")??;
            if response.to_lowercase() == "y" {
                std::fs::remove_file(cache_file)?;
            }
        }
    }

    Ok(())
}
