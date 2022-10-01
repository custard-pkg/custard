use eyre::Result;
use owo_colors::OwoColorize;

use crate::consts::{script_not_found, SCRIPTS_NOT_FOUND};
use crate::package_json::PackageJson;
use crate::util::user_error;

pub fn invoke(script_name: String) -> Result<()> {
    let package_json = PackageJson::from_package_json_file()?;

    match package_json.scripts {
        Some(scripts) => {
            match scripts.get(&script_name) {
                Some(script_content) => {
                    run_script(script_name, script_content)
                },
                _ => user_error(&script_not_found(&script_name), exitcode::CONFIG)
            }
        }
        _ => user_error(SCRIPTS_NOT_FOUND, exitcode::CONFIG),
    }

    Ok(())
}

fn run_script(name: String, content: &str) {
    println!("")
}
