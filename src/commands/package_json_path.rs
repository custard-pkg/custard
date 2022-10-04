use eyre::Result;
use owo_colors::OwoColorize;
use rust_i18n::t;

use crate::consts::MAX_RECURSION_DEPTH;
use crate::util::find_package_json;

pub fn invoke() -> Result<()> {
    let path = find_package_json()?;

    println!(
        "{}\n{}\n{})",
        t!("package-json-found-at").green().bold(),
        path.to_string_lossy(),
        format!("(max depth: {MAX_RECURSION_DEPTH}").black()
    );

    Ok(())
}
