use eyre::Result;
use fnv::FnvHashMap;
use node_semver::Version;
use rust_i18n::t;
use serde::{Serialize, Deserialize};
use tokio::fs::read_to_string;

use crate::util::{find_package_json, user_error};

#[derive(Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: Version,
    pub author: String,
    pub license: String,
    pub description: String,
    pub main: String,
    pub scripts: Option<FnvHashMap<String, String>>,
    pub repository: Option<Repository>
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub r#type: String,
    pub url: String
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
