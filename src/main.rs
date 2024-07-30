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

fn main() -> anyhow::Result<()> {
    let args = cli::parse_args()?;
    match args.help {
        Some(HelpType::Long) => {
            if let Some(op) = args.op {
                print_op_help(op);
            } else {
                print_long_help();
            }
            return Ok(());
        }
        Some(HelpType::Short) => {
            print_short_help();
            return Ok(());
        }
        _ => {}
    }
    dbg!(args);

    /*let cache_file = cli_args
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

    let get_dir = |dir: &Option<PathBuf>, global: bool| -> anyhow::Result<PathBuf> {
        Ok(if global {
            PathBuf::new()
        } else if let Some(d) = dir {
            d.into()
        } else {
            std::env::current_dir()?
        })
    };

    let best_cmd =
        |name: &str, dir: &PathBuf, strict_match: bool| -> anyhow::Result<Option<Command>> {
            let best_cmd = c.find_cmd(&name, &dir)?;
            if strict_match {
                // Only look for specific command
                return Ok(best_cmd);
            }
            if best_cmd.is_some() {
                // Have a best command, might as well use it
                return Ok(best_cmd);
            }
            // Fall back to fetching all with name
            let mut cmds = c.find_cmds_by_name(&name)?;
            if cmds.len() == 0 {
                Ok(None)
            } else if cmds.len() == 1 {
                // Only one, might as well use it
                Ok(Some(cmds.pop().unwrap()))
            } else {
                // We have multiple matches, should ask user
                let mut map = std::collections::HashMap::new();
                for cmd in cmds.into_iter() {
                    println!("{:+}", cmd);
                    map.insert(cmd.id, cmd);
                }
                print!("\nCommand id: ");
                std::io::stdout().flush()?;
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .context("Failed to read from STDIN")?;
                let id: i64 = input.trim().parse().context("Malformed id")?;
                Ok(map.remove(&id))
            }
        };

    match cli_args.command {
        CliCommand::Add {
            global,
            dir,
            name,
            command,
            args,
            env,
        } => {
            let d = get_dir(&dir, global)?;
            let cmd = Command {
                id: 0,
                name: name.clone(),
                command,
                args,
                envs: env,
                dir: d,
            };
            if c.insert(&cmd)? {
                println!("Created {cmd}");
            } else {
                Err(anyhow::anyhow!(
                    "Failed to create command {name}: already exists"
                ))?
            }
        }
        CliCommand::Remove {
            global,
            dir,
            cwd,
            id,
            name,
        } => {
            if let Some(id) = id {
                c.delete_by_id(id)?;
            } else {
                let name = name.unwrap();
                let d = get_dir(&dir, global)?;
                let cmd = best_cmd(&name, &d, global || dir.is_some() || cwd)?
                    .context(format!("No matches found for command {name}"))?;
                if c.delete_by_id(cmd.id)? {
                    println!("Deleted {}", cmd);
                } else {
                    println!("No matching command found, nothing was deleted");
                }
            }
        }
        CliCommand::Exec {
            global,
            dir,
            cwd,
            name,
        } => {
            let d = get_dir(&dir, global)?;
            let cmd = best_cmd(&name, &d, global || dir.is_some() || cwd)?
                .context(format!("No matches found for command {name}"))?;
            cmd.exec()?;
        }
        CliCommand::List { id } => {
            if id {
                for cmd in c.fetch_all()? {
                    println!("{:+}", cmd);
                }
            } else {
                for cmd in c.fetch_all()? {
                    println!("{}", cmd);
                }
            }
        }
        CliCommand::Clear => {
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
    }*/

    Ok(())
}
