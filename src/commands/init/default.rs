use eyre::Result;

use crate::consts::NO_TEST_SPECIFIED;
use crate::fnv_map;
use crate::package_json::PackageJson;
use crate::util::get_current_dir_name;

pub fn invoke() -> Result<PackageJson> {
    let package_json = PackageJson {
        name: get_current_dir_name()?,
        version: "1.0.0".parse().unwrap(),
        author: String::new(),
        license: "MIT".into(),
        description: String::new(),
        main: "index.js".into(),
        scripts: Some(fnv_map! {
            "test".into() => NO_TEST_SPECIFIED.into()
        }),
        repository: None,
    };

    Ok(package_json)
}
