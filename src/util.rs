use std::path::{Path, PathBuf};
use std::{env, process};

use dialoguer::{theme::ColorfulTheme, Input};
use exitcode::ExitCode;
use eyre::Result;
use eyre::WrapErr;
use owo_colors::OwoColorize;
use rust_i18n::t;

use crate::consts::MAX_RECURSION_DEPTH;

/// Find the closest file/directory with the name you want, to
/// the `_current_dir` Path.
///
/// ```rust
/// let cargo_manifest = find_closest_file_or_dir(path, "Cargo.toml");
/// println!("{:?}", cargo_manifest.unwrap());
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
        if let Some(f) = find_closest_file_or_dir(
            &env::current_dir().wrap_err("failed to get current dir")?,
            "package.json",
        ) {
            f
        } else {
            user_error(t!("package-json-not-found"), exitcode::NOINPUT);
            unreachable!()
        },
    )
}

/// Prints an user-facing error, and exits.
pub fn user_error(error: String, exit_code: ExitCode) {
    eprintln!("{} {}", "error:".red().bold(), error.bold());
    process::exit(exit_code);
}

/// Uses `dialoguer` to ask for an input.
/// ```rust
/// let name = input("What is your name?", None).unwrap();
/// println!("Hello, {name}!");
/// ```
pub fn input(prompt: &str, default: Option<String>) -> Result<String> {
    let theme = ColorfulTheme::default();
    let mut input = Input::with_theme(&theme);
    input.with_prompt(prompt);
    input.default(default.unwrap_or_default());

    Ok(input.interact_text()?)
}

/// Gets the current working directory's name.
/// ```rust
/// let dir_name = get_current_dir_name().unwrap();
/// println!("Directory name: {dir_name}");
/// ```
pub fn get_current_dir_name() -> Result<String> {
    Ok(env::current_dir()?
        .file_name()
        // guaranteed to not fail, `current_dir` always returns a PathBuf
        // with components
        .unwrap()
        .to_string_lossy()
        .to_string())
}

/// Create an FNV `HashMap`, based on the Fowler-Noll-Vo hasher.
#[macro_export]
macro_rules! fnv_map {
    (@to_unit $($_:tt)*) => (());
    (@count $($tail:expr),*) => (
      <[()]>::len(&[$(fnv_map!(@to_unit $tail)),*])
    );

    {$($k: expr => $v: expr),* $(,)?} => {
      {
        let mut map = fnv::FnvHashMap::with_capacity_and_hasher(
            fnv_map!(@count $($k),*),
          Default::default()
        );

        $(
          map.insert($k, $v);
        )*

        map
      }
    };
  }
