use std::{collections::HashMap, fs::File, io::Read, hint};

use eyre::Result;
use rust_i18n::t;
use serde::Deserialize;

use crate::util::{find_package_json, user_error};

#[derive(Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub scripts: Option<HashMap<String, String>>,
}

impl PackageJson {
    pub fn from_package_json_file() -> Result<PackageJson> {
        let mut s = String::new();
        File::open(find_package_json()?)?.read_to_string(&mut s)?;
        
        match serde_json::from_str(&s) {
            Ok(package_json) => Ok(package_json),
            Err(e) => {
                user_error(t!("failed-to-read-package-json", error = &e.to_string()), exitcode::CONFIG);
                unsafe { hint::unreachable_unchecked() }
            }
        }
    }
}
