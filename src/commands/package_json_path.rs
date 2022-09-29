use eyre::Result;
use owo_colors::OwoColorize;

use crate::commands::util::{find_package_json, user_error, MAX_RECURSION_DEPTH};

pub fn invoke() -> Result<()> {
    let path = find_package_json()?;

    match path {
        Some(path) => {
            println!(
                "{}\n{}\n{} {})",
                "a `package.json` file was found at:".green().bold(),
                path.to_string_lossy(),
                "(max depth:".black(),
                MAX_RECURSION_DEPTH.black()
            )
        }
        None => user_error(
            "a `package.json` file was not found in this project.".into(),
            exitcode::NOINPUT,
        ),
    }

    Ok(())
}
