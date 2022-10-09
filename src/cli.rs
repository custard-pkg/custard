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
    /// Run a script in `package.json`.
    #[clap(aliases = &["run-script", "rum", "urn"])]
    Run {
        script: Option<String>,
        args: Option<Vec<String>>,
    },

    /// Show the `package.json` path for this package.
    #[clap(aliases = &["package-json", "package-path"])]
    PackageJsonPath,

    /// Create a new `package.json` file.
    #[clap(aliases = &["create", "innit"])]
    Init {
        #[arg(short, long)]
        yes: bool,
    },
}
