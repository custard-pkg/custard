use std::process::Command;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use eyre::Result;
use predicates::prelude::*;
use rust_i18n::t;

const TEST_PACKAGE_JSON: &str = include_str!("text/run.json");

fn dir_with_package_json() -> Result<TempDir> {
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("package.json");
    input_file.touch()?;
    input_file.write_str(TEST_PACKAGE_JSON)?;

    Ok(temp)
}

#[test]
fn runs_scripts_in_foreground() -> Result<()> {
    let temp = dir_with_package_json()?;

    let mut cmd = &mut Command::cargo_bin("custard")?;
    cmd = cmd.current_dir(temp.path());

    cmd.arg("run").arg("hello");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("one two three"));

    Ok(())
}

#[test]
fn prompt_command_on_lifecycle_with_run() -> Result<()> {
    let temp = dir_with_package_json()?;

    let mut cmd = &mut Command::cargo_bin("custard")?;
    cmd = cmd.current_dir(temp.path());

    cmd.arg("run").arg("test");
    cmd.assert().success().stderr(predicate::str::contains(t!(
        "run-with-lifecycle-script-command",
        name = "test"
    )));

    Ok(())
}

rust_i18n::i18n!("../../locales");
