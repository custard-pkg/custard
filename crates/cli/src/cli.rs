use clap::{Parser, Subcommand};

/// Custard - a fast `npm` alternative.
#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run a script in `package.json`.
    #[clap(aliases = &["run-script", "rum", "urn"])]
    Run {
        /// The name of the script you want to run
        script: Option<String>,

        /// Any arguments you want to pass to the script
        args: Option<Vec<String>>,
    },

    /// Test a package.
    Test {
        /// The name of the script you want to run
        script: Option<String>,

        /// Any arguments you want to pass to the script
        args: Option<Vec<String>>,
    },

    /// Show the `package.json` path for this package.
    #[clap(aliases = &["package-json", "package-path"])]
    PackageJsonPath,

    /// Create a new `package.json` file.
    #[clap(aliases = &["create", "innit"])]
    Init {
        #[arg(short, long)]
        /// Skip any prompts and just use the defaults
        yes: bool,
    },
}
