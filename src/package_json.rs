use eyre::Result;
use fnv::FnvHashMap;
use lazy_static::lazy_static;
use node_semver::Version;
use regex::Regex;
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;

use crate::consts::PACKAGE_NAME_VALIDATION_REGEX;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub r#type: String,
    pub url: String,
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

#[allow(clippy::ptr_arg)]
pub fn validate_package_name(name: &String) -> Result<(), &'static str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(PACKAGE_NAME_VALIDATION_REGEX).unwrap();
    }

    match RE.is_match(name) {
        true => Ok(()),
        false => Err("The package name is invalid!"),
    }
}

/// Validate a `SemVer` version.
#[allow(clippy::ptr_arg)]
pub fn validate_version(value: &String) -> Result<(), &'static str> {
    let version: Result<Version, _> = value.parse();

    match version {
        Ok(_) => Ok(()),
        _ => Err("Invalid SemVer version"),
    }
}
