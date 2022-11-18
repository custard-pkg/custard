#[cfg(not(tarpaulin_include))]
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
        /// The name of the script you want to run.
        script: Option<String>,

        /// Any arguments you want to pass to the script.
        args: Option<Vec<String>>,

        /// The shell to use when running scripts.
        /// Defaults to '/bin/sh' on POSIX systems and to 'cmd.exe' on Windows
        #[arg(long, default_value_t = get_platform_shell())]
        script_shell: String,

        /// If true, `custard` will not exit with an error code when
        /// `run` is invoked for a script that isn't defined in the `scripts`
        /// section of `package.json`.
        #[arg(long, default_value_t = false)]
        if_present: bool,

        /// If set, `custard` won't run any pre or post scripts.
        #[arg(long, default_value_t = false)]
        ignore_scripts: bool,
    },

    /// Test a package.
    Test {
        /// Any arguments you want to pass to the script
        args: Option<Vec<String>>,

        /// The shell to use when running scripts.
        /// Defaults to '/bin/sh' on POSIX systems and to 'cmd.exe' on Windows
        #[arg(long, default_value_t = get_platform_shell())]
        script_shell: String,

        /// If true, `custard` will not exit with an error code when
        /// `run` is invoked for a script that isn't defined in the `scripts`
        /// section of `package.json`.
        #[arg(long, default_value_t = false)]
        if_present: bool,

        /// If set, `custard` won't run any pre or post scripts.
        #[arg(long, default_value_t = false)]
        ignore_scripts: bool,
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

#[cfg(not(tarpaulin_include))]
fn get_platform_shell() -> String {
    if cfg!(windows) { "cmd.exe" } else { "/bin/sh" }.into()
}
