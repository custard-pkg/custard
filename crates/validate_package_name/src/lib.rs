use lazy_static::lazy_static;
use regex::Regex;
use urlencoding::encode;

mod banned_names;
use banned_names::is_banned;

lazy_static! {
    static ref SCOPED_PACKAGE_REGEX: Regex = Regex::new("^(?:@([^/]+?)[/])?([^/]+?)$").unwrap();
}

const MAX_LEN: usize = 214;

/// Validates an `npm` package name.
///
/// ```rust
/// use validate_package_name::validate;
///
/// assert!(validate(&String::from("hello")).is_ok())
/// ```
///
/// # Errors
/// This function can fail if the package name is invalid.
pub fn validate(name: &String) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("package name must not be empty");
    }

    if name.starts_with('.') || name.starts_with('_') {
        return Err("package name cannot start with a period or underscore");
    }

    if name.trim() != name {
        return Err("package name cannot contain leading or trailing spaces");
    }

    if is_banned(name) {
        return Err("package name is blacklisted or is a core module name");
    }

    if name.len() > 214 {
        return Err("package name cannot contain more than 214 characters");
    }

    if &name.to_lowercase() != name {
        return Err("package name cannot contain capital letters");
    }

    if &encode(name).into_owned() != name {
        let name_match = SCOPED_PACKAGE_REGEX.captures(name);

        if let Some(matches) = name_match {
            // The regex returns **three** matches:
            //
            // - the full name ("@custard/hey")
            // - the user ("custard"),
            // - the package ("hi")

            if matches.len() != 3 || matches.get(1).is_none() || matches.get(2).is_none() {
                return Err("scoped package name is invalid");
            }

            let user = &matches[1];
            let pkg = &matches[2];

            if encode(user) == user && encode(pkg) == pkg {
                return Ok(());
            }
        }

        return Err("package name can only contain URL-friendly characters");
    }

    if name.len() > MAX_LEN {
        return Err("Package name cannot be longer than 214 characters");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::validate;

    fn assert_err(name: &str) {
        let is_valid = validate(&String::from(name));

        if is_valid.is_ok() {
            panic!("test failed: package name `{name}` passes when it shouldn't")
        }
    }

    fn assert_ok(name: &str) {
        let is_valid = validate(&String::from(name));

        if let Err(e) = is_valid {
            panic!("test failed: package name `{name}` fails with error: {e}")
        }
    }

    #[test]
    fn accept_valid() {
        assert_ok("some-package");
        assert_ok("discord.js");
        assert_ok("num3ric");
        assert_ok("under_scores")
    }

    #[test]
    fn reject_special_characters() {
        assert_err("crazy!");
        assert_err("@npm-zors/money!time.js")
    }

    #[test]
    fn accept_scoped() {
        assert_ok("@custard/hi")
    }

    #[test]
    fn reject_zero_len() {
        assert_err("")
    }

    #[test]
    fn reject_name_that_starts_with_period() {
        assert_err(".start-with-period")
    }

    #[test]
    fn reject_name_that_starts_with_underscore() {
        assert_err("_start-with-underscore")
    }

    #[test]
    fn reject_colons_in_name() {
        assert_err("contain:colons")
    }

    #[test]
    fn reject_leading_space() {
        assert_err(" leading-space")
    }

    #[test]
    fn reject_trailing_space() {
        assert_err("trailing-space ")
    }

    #[test]
    fn reject_non_url_friendly() {
        assert_err("s/l/a/s/h/e/s")
    }

    #[test]
    fn reject_blacklisted_name() {
        assert_err("favicon.ico");
        assert_err("node_modules")
    }

    #[test]
    fn reject_core_modules() {
        assert_err("http");
        assert_err("process")
    }

    #[test]
    fn accept_max_len() {
        assert_ok("ifyouwanttogetthesumoftwonumberswherethosetwonumbersarechosenbyfindingthelargestoftwooutofthreenumbersandsquaringthemwhichismultiplyingthembyitselfthenyoushouldinputthreenumbersintothisfunctionanditwilldothatforyou")
    }

    #[test]
    fn reject_mixed_case() {
        assert_err("hello-WORLD")
    }

    #[test]
    fn reject_name_over_max_len() {
        assert_err("ifyouwanttogetthesumoftwonumberswherethosetwonumbersarechosenbyfindingthelargestoftwooutofthreenumbersandsquaringthemwhichismultiplyingthembyitselfthenyoushouldinputthreenumbersintothisfunctionanditwilldothatforyoucool")
    }

    #[test]
    fn reject_empty() {
        assert_err("")
    }
}
