pub const PACKAGE_JSON_NOT_FOUND: &str = "a `package.json` file was not found in this package.";
pub const PACKAGE_JSON_FOUND_AT: &str = "a `package.json` file was found at:";
pub const SCRIPTS_NOT_FOUND: &str =
    "the `scripts` field was not found in your `package.json` file.";
pub const SCRIPT_TERMINATED_BY_SIGNAL: &str = "script was terminated by signal (e.g. Ctrl-C or Cmd-C)";

pub const SCRIPT_SIGNAL_EXIT_CODE: i32 = 420;
pub const MAX_RECURSION_DEPTH: usize = 8;

pub fn script_not_found(script: &str) -> String {
    format!("the script `{script}` was not found.")
}

pub fn script_not_ok(code: i32) -> String {
    format!("script exited with exit code {code}")
}
