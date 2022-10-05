use std::process;

use eyre::Result;
use owo_colors::OwoColorize;
use rust_i18n::t;

use crate::commands::run::util::scripts_field_not_found;
use crate::package_json::PackageJson;

pub fn list_scripts() -> Result<()> {
    let package_json = PackageJson::from_package_json_file()?;
    let scripts = package_json.scripts;

    match scripts {
        Some(scripts) => {
            if scripts.is_empty() {
                println!(
                    "{}",
                    t!("no-scripts-added", package_name = &package_json.name)
                        .red()
                        .bold()
                );
                process::exit(0)
            }

            println!(
                "{}",
                t!(
                    "scripts-in-package",
                    count = &format!("{} scripts", &scripts.len()).purple().to_string(),
                    package_name = &package_json.name
                )
                .bold()
            )
        }
        _ => scripts_field_not_found(),
    }

    Ok(())
}
