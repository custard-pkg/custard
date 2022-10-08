use std::env;

use dialoguer::{theme::ColorfulTheme, Input};
use eyre::Result;
use node_semver::Version;

use crate::util::validate_version;

pub fn invoke() -> Result<()> {
    // Ask the questions
    let name = input("Package name", Some(get_current_dir_name()?));
    let version_string = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Package version")
        .default("1.0.0".to_string())
        .validate_with(validate_version)
        .interact_text()
        .unwrap();
    let description = input("Package description", None);
    let entry_point = input("Entry point", Some("index.js".into()));
    let test_command = input("Test command", None);
    let git_repository = input("Git repository", None);
    let author = input("Author", None);

    let version: Version = version_string.parse().unwrap();

    dbg!(
        name,
        version,
        description,
        entry_point,
        test_command,
        git_repository,
        author
    );

    Ok(())
}

pub fn input(prompt: &str, default: Option<String>) -> String {
    let theme = ColorfulTheme::default();
    let mut input = Input::with_theme(&theme);
    input.with_prompt(prompt);
    input.default(default.unwrap_or_default());

    input.interact_text().expect("failed to ask for input")
}

fn get_current_dir_name() -> Result<String> {
    Ok(env::current_dir()?
        .file_name()
        // guaranteed to not fail, `current_dir` always returns a PathBuf
        // with components
        .unwrap()
        .to_string_lossy()
        .to_string())
}
