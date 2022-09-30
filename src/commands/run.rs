use std::hint::unreachable_unchecked;

use eyre::Result;
use owo_colors::OwoColorize;

use crate::consts::{script_not_found, SCRIPTS_NOT_FOUND};
use crate::package_json::PackageJson;
use crate::util::user_error;

pub fn invoke(script_args: Vec<String>) -> Result<()> {
    let package_json = PackageJson::from_package_json_file()?;

    match package_json.scripts {
        Some(scripts) => {
            for script in script_args {
                if !scripts.contains_key(&script) {
                    user_error(&script_not_found(&script), exitcode::CONFIG);
                }
                run_script(script, scripts[&script])
            }
        }
        _ => user_error(SCRIPTS_NOT_FOUND, exitcode::CONFIG),
    }

    Ok(())
}

fn run_script(name: String, content: String) {}
