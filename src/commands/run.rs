use eyre::Result;
use owo_colors::OwoColorize;

use crate::consts::{script_not_found, SCRIPTS_NOT_FOUND};
use crate::package_json::PackageJson;
use crate::util::user_error;

use std::process::{Command, ExitStatus};

pub fn invoke(script_name: String, args: Option<Vec<String>>) -> Result<()> {
    let args = args.unwrap_or_default();
    let package_json = PackageJson::from_package_json_file()?;

    match package_json.scripts {
        Some(scripts) => match scripts.get(&script_name) {
            Some(script_content) => {
                let pre_script_name = format!("pre{script_name}");
                let post_script_name = format!("post{script_name}");

                // Run prescript...
                if let Some(script_content) = scripts.get(&pre_script_name) {
                    run_script(pre_script_name, script_content, vec![])?
                }

                // ...then the script itself...
                run_script(script_name, script_content, args)?;

                // ...and finally the postscript
                if let Some(script_content) = scripts.get(&post_script_name) {
                    run_script(post_script_name, script_content, vec![])?
                }
            }
            _ => user_error(&script_not_found(&script_name), exitcode::CONFIG),
        },

        // The `scripts` field was not found
        _ => user_error(SCRIPTS_NOT_FOUND, exitcode::CONFIG),
    }

    Ok(())
}

fn run_script(name: String, content: &str, args: Vec<String>) -> Result<()> {
    println!("{} script `{name}`", "Running".green().bold());

    // Windows does not have `sh`
    let mut command = if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    };

    // Make the shell quiet
    let command = if cfg!(target_os = "windows") {
        command.arg("/C").args(args)
    } else {
        command.arg("-c").args(args)
    };

    let status = command.status()?;

    Ok(())
}
