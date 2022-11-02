use eyre::Result;
use fnv::FnvHashMap;
use package_json::PackageJson;
use serde_value::Value;
use serde_value_flatten::to_flatten_maptree;

const SEPERATOR: &str = "_";
const PREFIX: &str = "npm_";

pub fn get(package_json: &PackageJson) -> Result<FnvHashMap<String, String>> {
    let tree = to_flatten_maptree(SEPERATOR, Some(PREFIX), package_json)?;
    let mut result = FnvHashMap::default();

    for (key, value) in tree.iter() {
        if let Value::Option(option) = value {
            if let Some(value) = option {
                if let Box::new(Value::Map(map)) = value {

                }
            }
        }
    }

    Ok(result)
}

fn value_to_string(value: &Value) -> String {
    if let Value::String(s) = value {
        s.clone()
    } else {
        serde_json::to_string(value).unwrap()
    }
}
