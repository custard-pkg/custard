use std::{env, path::PathBuf};

use eyre::Result;
use rust_i18n::t;

use crate::util::{find_closest_file_or_dir, user_error};

pub fn scripts_field_not_found() {
    user_error(t!("scripts-field-not-found"), exitcode::CONFIG);
}

pub fn get_node_modules_bin_dir() -> Result<Option<PathBuf>> {
    let node_modules = find_closest_file_or_dir(&env::current_dir()?, "node_modules");

    if let Some(node_modules) = node_modules {
        Ok(Some(node_modules.join(".bin")))
    } else {
        Ok(None)
    }
}
