use clap::{Parser, Subcommand};

/// Custard - a fast `npm` alternative.
#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

/// Doc comment
#[derive(Subcommand)]
pub enum Command {
    /// Run a script in `package.json`
    Run,

    /// Show the `package.json` path for this project.
    PackageJsonPath,
}