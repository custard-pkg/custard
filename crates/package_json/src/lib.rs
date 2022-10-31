use custard_util::{find_package_json, user_error};
use eyre::Result;
use fnv::FnvHashMap;
use node_semver::Version;
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use spdx::Expression;
use tokio::fs::read_to_string;
pub use validate_package_name::validate as validate_package_name;

rust_i18n::i18n!("../../locales");

pub const PACKAGE_NAME_VALIDATION_REGEX: &str =
    "^(?:@[a-z0-9-*~][a-z0-9-*._~]*/)?[a-z0-9-~][a-z0-9-._~]*$";

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: Version,
    pub author: Option<String>,
    pub license: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub main: Option<String>,
    pub scripts: Option<FnvHashMap<String, String>>,
    pub repository: Option<Repository>,
    pub keywords: Option<Vec<String>>,
    pub os: Option<Vec<String>>,
    pub cpu: Option<Vec<String>>,
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
    /// Create a `PackageJson` struct from the package's `package.json` file.
    ///
    /// # Errors
    /// This function can fail if:
    /// - there is no `package.json` in the package
    /// - the `package.json` file is invalid
    pub async fn from_package_json_file() -> Result<Self> {
        let s = read_to_string(find_package_json()?).await?;

        match serde_json::from_str(&s) {
            Ok(package_json) => Ok(package_json),
            Err(e) => {
                user_error(
                    t!("package-json.failed-to-read", error = &e.to_string()),
                    exitcode::CONFIG,
                );
                unreachable!()
            }
        }
    }
}

/// Validate a `SemVer` version.
///
/// # Errors
/// This function can error if the version is invalid.
#[allow(clippy::ptr_arg)]
pub fn validate_version(value: &String) -> Result<(), &'static str> {
    let version: Result<Version, _> = value.parse();

    match version {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid SemVer version"),
    }
}

/// Validate an SPDX identifier.
///
/// # Errors
/// This function can fail if the license is invalid.
#[allow(clippy::ptr_arg)]
pub fn validate_spdx(value: &String) -> Result<(), &'static str> {
    match Expression::parse(value) {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid license identifier"),
    }
}
