use eyre::Result;
use owo_colors::OwoColorize;
use crate::util::user_error;
use crate::consts::SCRIPTS_NOT_FOUND;

use crate::package_json::PackageJson;

pub fn invoke(scripts: Vec<String>) -> Result<()> {
    let package_json = PackageJson::from_package_json_file()?;

    match package_json.scripts {
        Some(scripts) => {},
        _ => user_error(SCRIPTS_NOT_FOUND, exitcode::CONFIG)
    }

    Ok(())
}
