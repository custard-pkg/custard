use std::env;
use std::path::{Path, PathBuf};

use bstr::BStr;
use colored::Colorize;
use custard_util::{fnv_map, get_current_dir_name, input};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use eyre::Result;
use node_semver::Version;
use package_json::{
    validate_package_name, validate_spdx, validate_version, PackageJson, Repository,
};
use rust_i18n::t;
use serde_json::to_string_pretty;
use slug::slugify;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::consts::NO_TEST_SPECIFIED;

mod default;
mod git_remote;

use git_remote::find_origin_remote;

pub async fn invoke(yes: bool) -> Result<()> {
    if yes {
        let default = default::invoke()?;
        write_package_json_prompt(default, false).await?;
        return Ok(());
    }

    println!(
        "{}",
        t!("init.welcome.short", command = "`custard init`!")
            .bold()
            .purple()
    );
    println!("{}", t!("init.welcome.long"));

    // Ask the questions
    let name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(&t!("init.package.name-prompt"))
        .default(slugify(get_current_dir_name()?))
        .validate_with(validate_package_name)
        .interact_text()?
        .parse()?;

    let version: Version = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(&t!("init.package.version-prompt"))
        .default("1.0.0".to_string())
        .validate_with(validate_version)
        .interact_text()?
        .parse()?;
    let description = Some(input(&t!("init.package.description-prompt"), None)?);
    let entry_point = input(
        &t!("init.package.entry-point-prompt"),
        Some("index.js".into()),
    )?;
    let test_command = input("Test command", Some(NO_TEST_SPECIFIED.into()))?;

    let mut git_repository = input(
        &t!("init.package.git-repository-prompt"),
        find_origin_remote(&env::current_dir()?)?,
    )?;
    if !git_repository.is_empty() && !git_repository.ends_with(".git") {
        git_repository.push_str(".git");
    }

    let author = Some(input(&t!("init.package.author-prompt"), None)?);
    let license = Some(
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(&t!("init.package.license-prompt"))
            .default("MIT".to_string())
            .validate_with(validate_spdx)
            .interact_text()?
            .parse()?,
    );

    let package_json = PackageJson {
        name,
        version,
        author,
        license,
        description,
        main: Some(entry_point),
        scripts: Some(fnv_map! {
            "test".into() => test_command
        }),
        repository: if git_repository.is_empty() {
            None
        } else {
            Some(Repository {
                r#type: "git".into(),
                url: git_url::parse(BStr::new(&git_repository))?
                    .to_bstring()
                    .try_into()?,
            })
        },
        ..Default::default()
    };

    write_package_json_prompt(package_json, true).await?;

    Ok(())
}

async fn write(
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

async fn get_package_json_file() -> Result<(File, PathBuf)> {
    let current_dir = env::current_dir()?;
    let path = current_dir.join("package.json");
    let file = File::create(&path).await?;
    Ok((file, path))
}

async fn write_package_json_prompt(
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
