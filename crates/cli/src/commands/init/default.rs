use std::env;

use custard_util::{fnv_map, get_current_dir_name};
use eyre::Result;
use package_json::{PackageJson, Repository};
use slug::slugify;

use crate::consts::NO_TEST_SPECIFIED;
use crate::init::find_origin_remote;

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
        repository: find_origin_remote(&env::current_dir()?)?.map(|url| Repository {
            r#type: "git".into(),
            url,
        }),
        ..Default::default()
    };

    Ok(package_json)
}
