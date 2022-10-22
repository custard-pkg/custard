use std::process::Command;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use eyre::Result;
use predicates::prelude::*;

const EXAMPLE_PACKAGE_JSON: &str = include_str!("text/package-json.hello.json");

#[test]
fn runs_scripts_in_foreground() -> Result<()> {
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("package.json");
    input_file.touch()?;
    input_file.write_str(EXAMPLE_PACKAGE_JSON)?;

    let mut cmd = &mut Command::cargo_bin("custard")?;
    cmd = cmd.current_dir(temp.path());

    cmd.arg("run").arg("hello");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("one two three"));

    Ok(())
}
