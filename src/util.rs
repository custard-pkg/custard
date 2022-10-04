use std::hint::unreachable_unchecked;
use std::path::{Path, PathBuf};
use std::{env, process};

use exitcode::ExitCode;
use eyre::Result;
use eyre::WrapErr;
use owo_colors::OwoColorize;

use crate::consts::{PACKAGE_JSON_NOT_FOUND, MAX_RECURSION_DEPTH};

/// Find the closest file/directory with the name you want, to
/// the `_current_dir` Path.
///
/// ```rs
/// fn main() {
///     let cargo_manifest = find_closest_file_or_dir(path, "Cargo.toml");
///     println!("{:?}", cargo_manifest.unwrap());
/// }
/// ```
///
/// This function can fail if the recursion depth is reached.
///
/// Based on `https://github.com/egoist/dum/blob/main/src/run.rs`
pub fn find_closest_file_or_dir(_current_dir: &Path, name: &str) -> Option<PathBuf> {
    let mut current_dir = Path::new(_current_dir);
    let mut closest_file = None;
    let mut current_depth = 0;

    loop {
        let path = current_dir.join(name);
        current_depth += 1;

        if current_depth > MAX_RECURSION_DEPTH {
            break;
        }
        if path.exists() {
            closest_file = Some(path);
            break;
        }
        match current_dir.parent() {
            Some(p) => current_dir = p,
            None => break,
        }
    }

    closest_file
}

/// Find the `package.json` file for this package.
///
/// Can fail if the recursion limit is reached.
pub fn find_package_json() -> Result<PathBuf> {
    Ok(
        match find_closest_file_or_dir(
            &env::current_dir().wrap_err("failed to get current dir")?,
            "package.json",
        ) {
            Some(f) => f,
            _ => {
                user_error(PACKAGE_JSON_NOT_FOUND, exitcode::NOINPUT);
                unsafe { unreachable_unchecked() }
            }
        },
    )
}

/// Prints an user-facing error, and exits.
pub fn user_error(error: &str, exit_code: ExitCode) {
    eprintln!("{} {}", "error:".red().bold(), error.bold());
    process::exit(exit_code);
}
