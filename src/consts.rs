pub const PACKAGE_JSON_NOT_FOUND: &str = "a `package.json` file was not found in this package.";
pub const PACKAGE_JSON_FOUND_AT: &str = "a `package.json` file was found at:";
pub const SCRIPTS_NOT_FOUND: &str =
    "the `scripts` field was not found in your `package.json` file.";

pub fn script_not_found(script: &str) -> String {
    format!("the script `{script}` was not found.")
}
