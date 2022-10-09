#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]
#![deny(unsafe_code)]
use clap::Parser;
use eyre::Result;

mod cli;
mod commands;
mod consts;
mod package_json;
mod util;

use cli::{Cli, Command::*};
use commands::*;

rust_i18n::i18n!("locales");

#[tokio::main]
async fn main() -> Result<()> {
    rust_i18n::set_locale("en");
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        PackageJsonPath => package_json_path::invoke()?,
        Run { script, args } => run::invoke(script, args).await?,
        Init { yes } => init::invoke(yes).await?,
    }

    Ok(())
}
