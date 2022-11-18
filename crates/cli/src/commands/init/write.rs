use std::env;
use std::path::{Path, PathBuf};

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};
use eyre::Result;
use package_json::PackageJson;
use rust_i18n::t;
use serde_json::to_string_pretty;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn write(
    file: &mut File,
    package_json: &PackageJson,
    package_json_path: &Path,
) -> Result<()> {
    file.write_all(to_string_pretty(&package_json)?.as_bytes())
        .await?;

    println!(
        "{} `{}`",
        t!("init.successfully-wrote-package-json-to").green().bold(),
        package_json_path.to_string_lossy()
    );

    Ok(())
}

pub async fn get_package_json_file() -> Result<(File, PathBuf)> {
    let current_dir = env::current_dir()?;
    let path = current_dir.join("package.json");
    let file = File::create(&path).await?;
    Ok((file, path))
}

pub async fn write_package_json_prompt(
    package_json: PackageJson,
    ask_for_confirmation: bool,
) -> Result<()> {
    if ask_for_confirmation {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(&t!("init.write-package-json"))
            .interact()?
        {
            let (mut file, path) = get_package_json_file().await?;
            write(&mut file, &package_json, &path).await?;
        } else {
            eprintln!("{}", &t!("aborted-operation").red());
        }
    } else {
        let (mut file, path) = get_package_json_file().await?;
        write(&mut file, &package_json, &path).await?;
    }

    Ok(())
}
