use std::env;

use bstr::BStr;
use custard_util::{fnv_map, get_current_dir_name, input};
use dialoguer::{theme::ColorfulTheme, Input};
use eyre::Result;
use node_semver::Version;
use package_json::{
    validate_package_name, validate_spdx, validate_version, PackageJson, Repository,
};
use rust_i18n::t;
use slug::slugify;

use crate::consts::NO_TEST_SPECIFIED;

mod default;
mod git_remote;
mod welcome;
mod write;

use git_remote::find_origin_remote;
use write::write_package_json_prompt;

pub async fn invoke(yes: bool) -> Result<()> {
    if yes {
        let default = default::invoke()?;
        write_package_json_prompt(default, false).await?;
        return Ok(());
    }

    welcome::show();

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
