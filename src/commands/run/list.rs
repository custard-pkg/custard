use eyre::Result;
use colored::Colorize;
use rust_i18n::t;

use crate::commands::run::util::scripts_field_not_found;
use crate::package_json::PackageJson;

pub async fn scripts() -> Result<()> {
    let package_json = PackageJson::from_package_json_file().await?;
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
            } else {
                println!(
                    "{}\n{}",
                    t!(
                        "scripts-in-package",
                        count = &format!("{} script(s)", &scripts.len()).purple(),
                        package_name = &format!("`{}`", package_json.name).cyan()
                    )
                    .bold(),
                    t!("how-to-run-script").black()
                );

                for (name, content) in scripts {
                    println!(
                        "\n{} {}\n  {}",
                        "-".bold(),
                        name.bold(),
                        content.black().bold()
                    );
                }
            }
        }
        _ => scripts_field_not_found(),
    }

    Ok(())
}
