use custard_util::{find_package_json, user_error};
use eyre::Result;
use fnv::FnvHashMap;
use node_semver::Version;
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use spdx::Expression;
use tokio::fs::read_to_string;

pub use validate_package_name::validate as validate_package_name;

rust_i18n::i18n!("../../locales");

pub const PACKAGE_NAME_VALIDATION_REGEX: &str =
    "^(?:@[a-z0-9-*~][a-z0-9-*._~]*/)?[a-z0-9-~][a-z0-9-._~]*$";

#[derive(Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    pub main: String,
    pub scripts: Option<FnvHashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<FnvHashMap<String, String>>,
}

impl Default for PackageJson {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: "1.0.0".parse().unwrap(),
            author: Default::default(),
            license: Default::default(),
            description: Default::default(),
            homepage: Default::default(),
            main: Default::default(),
            scripts: Default::default(),
            repository: Default::default(),
            keywords: Default::default(),
            os: Default::default(),
            cpu: Default::default(),
            bin: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub r#type: String,
    pub url: String,
}

impl PackageJson {
    pub async fn from_package_json_file() -> Result<Self> {
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

/// Validate a `SemVer` version.
#[allow(clippy::ptr_arg)]
pub fn validate_version(value: &String) -> Result<(), &'static str> {
    let version: Result<Version, _> = value.parse();

    match version {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid SemVer version"),
    }
}

/// Validate an SPDX identifier.
#[allow(clippy::ptr_arg)]
pub fn validate_spdx(value: &String) -> Result<(), &'static str> {
    match Expression::parse(value) {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid license identifier"),
    }
}
