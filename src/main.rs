use std::{io::{BufRead, Write}, path::PathBuf};
use anyhow::Context;

mod command;
use command::Command;

mod command_store;
use command_store::CommandStore;

mod cli;
use clap::Parser;
use cli::{Cli, CliCommand};

fn main() -> anyhow::Result<()> {
    let cli_args = Cli::parse();

    let cache_file = cli_args.file
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
        })
    ).and_then(|p|
        Some(p.join("cdb.cache"))
    ).context("No suitable path found for cache file")?;

    let dir = if cli_args.global {
        PathBuf::new()
    } else {
        if let Some(p) = cli_args.dir {
            std::fs::canonicalize(p)?
        } else {
            std::env::current_dir()?
        }
    };

    let c = CommandStore::new(&cache_file)?;
    let mut global_str = "";
    if cli_args.global {
        global_str = " global";
    }

    let best_cmd = |name: &str| -> anyhow::Result<Option<Command>> {
        let best_cmd = c.find_cmd(&name, &dir)?;
        if cli_args.cwd || cli_args.global {
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
        CliCommand::Push { name, command, args } => {
            if c.insert( Command { id: 0, name: name.clone(), command, args, dir } )? {
                println!("Created{global_str} command: {name}");
            } else {
                Err(anyhow::anyhow!("Failed to create{global_str} command for {name}, already exists"))?
            }
        },
        CliCommand::Pop { name } => {
            let cmd = best_cmd(&name)?
                .context(format!("No matches found for{global_str} command {name}"))?;
            c.delete_by_id(cmd.id)?;
        },
        CliCommand::Exec { name } => {
            let cmd = best_cmd(&name)?
                .context(format!("No matches found for{global_str} command {name}"))?;
            cmd.exec()?;
        },
        CliCommand::List => {
            for cmd in c.fetch_all()? {
                println!("{}", cmd);
            }
        },
        CliCommand::Clear => {
            print!("This will remove all saved commands from the store. Continue? [yn]: ");
            std::io::stdout().flush()?;
            let response = std::io::stdin().lock().lines().next()
                .context("Failed to read response from stdin")??;
            if response.to_lowercase() == "y" {
                std::fs::remove_file(cache_file)?;
            }

        }
    }

    Ok(())
}
