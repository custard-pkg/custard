use eyre::Result;
use fnv::FnvHashMap;
use rust_i18n::t;
use serde::Deserialize;
use tokio::fs::read_to_string;

use crate::util::{find_package_json, user_error};

#[derive(Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub scripts: Option<FnvHashMap<String, String>>,
}

impl PackageJson {
    pub async fn from_package_json_file() -> Result<PackageJson> {
        let s = read_to_string(find_package_json()?).await?;

        match serde_json::from_str(&s) {
            Ok(package_json) => Ok(package_json),
            Err(e) => {
                user_error(
                    t!("failed-to-read-package-json", error = &e.to_string()),
                    exitcode::CONFIG,
                );
                unreachable!()
            }
        }
    }
}
