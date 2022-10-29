use custard_util::{fnv_map, get_current_dir_name};
use eyre::Result;
use package_json::PackageJson;
use slug::slugify;

use crate::consts::NO_TEST_SPECIFIED;

pub fn invoke() -> Result<PackageJson> {
    let package_json = PackageJson {
        name: slugify(get_current_dir_name()?),
        version: "1.0.0".parse().unwrap(),
        author: Some(String::new()),
        license: Some("MIT".into()),
        description: Some(String::new()),
        main: Some("index.js".into()),
        scripts: Some(fnv_map! {
            "test".into() => NO_TEST_SPECIFIED.into()
        }),
        repository: None,
        ..Default::default()
    };

    Ok(package_json)
}
