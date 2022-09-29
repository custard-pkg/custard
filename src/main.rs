use color_eyre::eyre::Context;
use eyre::Result;

mod cli;
mod commands;
use commands::*;
use cli::{Cli, Command::*};

use clap::Parser;

fn main() -> Result<()> {
    color_eyre::install().wrap_err("failed to install color-eyre")?;

    let cli = Cli::parse();

    match cli.command {
        PackageJsonPath => package_json_path::invoke()?,
        _ => unreachable!()
    }

    Ok(())
}