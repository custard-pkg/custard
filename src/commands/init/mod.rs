use std::env;
use std::path::PathBuf;

use dialoguer::{theme::ColorfulTheme, Input};
use eyre::Result;
use node_semver::Version;
use owo_colors::OwoColorize;
use rust_i18n::t;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::consts::NO_TEST_SPECIFIED;
use crate::fnv_map;
use crate::package_json::{PackageJson, Repository};
use crate::util::{get_current_dir_name, input, validate_version};

mod default;

pub async fn invoke(yes: bool) -> Result<()> {
    if yes {
        let default = default::invoke()?;
        write_package_json(default).await?;
        return Ok(());
    }

    // Ask the questions
    let name = input(&t!("package-name-prompt"), Some(get_current_dir_name()?))?;
    let version: Version = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(&t!("package-version-prompt"))
        .default("1.0.0".to_string())
        .validate_with(validate_version)
        .interact_text()?
        .parse()?;
    let description = input(&t!("package-description-prompt"), None)?;
    let entry_point = input(&t!("package-entry-point-prompt"), Some("index.js".into()))?;
    let test_command = input("Test command", Some(NO_TEST_SPECIFIED.into()))?;

    let mut git_repository = input(&t!("package-git-repository-prompt"), None)?;
    if !git_repository.ends_with(".git") {
        git_repository.push_str(".git");
    }

    let author = input(&t!("package-author-prompt"), None)?;

    let package_json = PackageJson {
        name,
        version,
        author,
        license: "MIT".into(),
        description,
        main: entry_point,
        scripts: Some(fnv_map! {
            "test".into() => test_command
        }),
        repository: if git_repository.is_empty() {
            None
        } else {
            Some(Repository {
                r#type: "git".into(),
                url: format!("git+{git_repository}"),
            })
        },
    };

    write_package_json(package_json).await?;

    Ok(())
}

async fn write_package_json(package_json: PackageJson) -> Result<PathBuf> {
    use serde_json::to_string_pretty;

    let current_dir = env::current_dir()?;
    let package_json_path = current_dir.join("package.json");

    let mut file = File::create(&package_json_path).await?;
    file.write_all(to_string_pretty(&package_json)?.as_bytes())
        .await?;

    println!(
        "{} `{}`",
        t!("successfully-wrote-package-json-to").green().bold(),
        package_json_path.to_string_lossy()
    );
    Ok(package_json_path)
}
