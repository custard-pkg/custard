use std::{env, path::PathBuf};

use eyre::Result;
use rust_i18n::t;

use crate::util::user_error;

pub fn scripts_field_not_found() {
    user_error(t!("scripts-field-not-found"), exitcode::CONFIG);
}

pub fn get_node_modules_bin_dir() -> Result<PathBuf> {
    Ok(env::current_dir()?.join("node_modules").join(".bin"))
}
