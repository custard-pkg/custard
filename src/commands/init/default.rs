use eyre::Result;
use slug::slugify;

use crate::consts::NO_TEST_SPECIFIED;
use crate::fnv_map;
use crate::package_json::PackageJson;
use crate::util::get_current_dir_name;

pub fn invoke() -> Result<PackageJson> {
    let package_json = PackageJson {
        name: slugify(get_current_dir_name()?),
        version: "1.0.0".parse().unwrap(),
        author: Some(String::new()),
        license: Some("MIT".into()),
        description: Some(String::new()),
        main: "index.js".into(),
        scripts: Some(fnv_map! {
            "test".into() => NO_TEST_SPECIFIED.into()
        }),
        repository: None,
        ..Default::default()
    };

    Ok(package_json)
}
