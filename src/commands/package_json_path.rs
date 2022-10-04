use eyre::Result;
use owo_colors::OwoColorize;

use crate::consts::{PACKAGE_JSON_FOUND_AT, MAX_RECURSION_DEPTH};
use crate::util::{find_package_json};

pub fn invoke() -> Result<()> {
    let path = find_package_json()?;

    println!(
        "{}\n{}\n{})",
        PACKAGE_JSON_FOUND_AT.green().bold(),
        path.to_string_lossy(),
        format!("(max depth: {MAX_RECURSION_DEPTH}").black()
    );

    Ok(())
}
