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
use clap::Parser;
use cli::{Cli, CliCommand};

fn main() -> anyhow::Result<()> {
    let cli_args = Cli::parse();

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
                todo!();
            }
        };

    match cli_args.command {
        CliCommand::Push {
            global,
            dir,
            name,
            command,
            args,
        } => {
            let d = get_dir(&dir, global)?;
            if c.insert(Command {
                id: 0,
                name: name.clone(),
                command,
                args,
                dir: d,
            })? {
                println!("Created command: {name}");
            } else {
                Err(anyhow::anyhow!(
                    "Failed to create command for {name}, already exists"
                ))?
            }
        }
        CliCommand::Pop {
            global,
            dir,
            cwd,
            name,
        } => {
            let d = get_dir(&dir, global)?;
            let cmd = best_cmd(&name, &d, global || dir.is_some() || cwd)?
                .context(format!("No matches found for command {name}"))?;
            c.delete_by_id(cmd.id)?;
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
        CliCommand::List => {
            for cmd in c.fetch_all()? {
                println!("{}", cmd);
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
    }

    Ok(())
}
