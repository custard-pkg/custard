use colored::Colorize;
use custard_util::{find_package_json, MAX_RECURSION_DEPTH};
use eyre::Result;
use rust_i18n::t;

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
