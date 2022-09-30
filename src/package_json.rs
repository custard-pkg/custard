use std::{collections::HashMap, fs::File, io::Read};

use eyre::{Context, Result};
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
        serde_json::from_str(&s)
            .wrap_err("failed to read `package.json`, are all required fields filled?")
    }
}
