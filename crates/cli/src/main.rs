#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]
#![deny(unsafe_code)]
use clap::Parser;
use eyre::Result;

mod cli;
mod commands;
mod consts;

use cli::{Cli, Command::*};
use commands::*;

rust_i18n::i18n!("../../locales");

#[tokio::main]
async fn main() -> Result<()> {
    rust_i18n::set_locale("en");
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        PackageJsonPath => package_json_path::invoke()?,
        Run {
            script,
            args,
            script_shell,
            if_present,
            ignore_scripts,
        } => {
            run::invoke(
                script,
                args,
                script_shell,
                if_present,
                ignore_scripts,
                false,
            )
            .await?
        }
        Test {
            args,
            script_shell,
            if_present,
            ignore_scripts,
        } => lifecycle("test", script_shell, args, if_present, ignore_scripts).await?,
        Init { yes } => init::invoke(yes).await?,
    }

    Ok(())
}

async fn lifecycle(
    name: &str,
    script_shell: String,
    args: Option<Vec<String>>,
    if_present: bool,
    ignore_scripts: bool,
) -> Result<()> {
    run::invoke(
        Some(name.into()),
        args,
        script_shell,
        true,
        if_present,
        ignore_scripts,
    )
    .await
}
