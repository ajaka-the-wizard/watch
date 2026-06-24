use std::{fs, process};

use anyhow::{Context, Result};
use clap::{Args as ClapArgs, Parser, Subcommand};

const DEFAULT_JSON_CONFIG_EXAMPLE: &str = include_str!("../../defaults/watch.json");
const DEFAULT_TOML_CONFIG_EXAMPLE: &str = include_str!("../../defaults/watch.toml");

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Run,
    Init(InitArgs),
}

#[derive(ClapArgs, Debug, Clone)]
pub struct InitArgs {
    #[arg(long, conflicts_with = "toml")]
    pub json: bool,
    #[arg(long, conflicts_with = "json")]
    pub toml: bool,
}

impl Args {
    pub fn check() -> Result<()> {
        let args = Self::parse();
        if let Some(Command::Init(init)) = args.command {
            let (file_name, contents) = if init.toml {
                ("watch.toml", DEFAULT_TOML_CONFIG_EXAMPLE)
            } else {
                ("watch.json", DEFAULT_JSON_CONFIG_EXAMPLE)
            };

            fs::write(file_name, contents)
                .with_context(|| format!("Failed to write example config file '{}'", file_name))?;
            process::exit(0);
        };
        Ok(())
    }
}
