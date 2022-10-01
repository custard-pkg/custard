use clap::Parser;
use color_eyre::eyre::Context;
use eyre::Result;

mod cli;
mod commands;
mod consts;
mod package_json;
mod util;

use cli::{Cli, Command::*};
use commands::*;

fn main() -> Result<()> {
    color_eyre::install().wrap_err("failed to install color-eyre")?;

    let cli = Cli::parse();

    match cli.command {
        PackageJsonPath => package_json_path::invoke()?,
        Run { script, args } => run::invoke(script, args)?,
    }

    Ok(())
}
