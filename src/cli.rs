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
    Run { script: String },

    /// Show the `package.json` path for this package.
    #[clap(aliases = &["package-json", "package-path"])]
    PackageJsonPath,
}
