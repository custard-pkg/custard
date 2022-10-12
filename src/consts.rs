pub const SCRIPT_SIGNAL_EXIT_CODE: i32 = 420;
pub const MAX_RECURSION_DEPTH: usize = 8;
pub const NO_TEST_SPECIFIED: &str = "echo \"Error: no test specified\" && exit 1";
pub const PACKAGE_NAME_VALIDATION_REGEX: &str =
    "^(?:@[a-z0-9-*~][a-z0-9-*._~]*/)?[a-z0-9-~][a-z0-9-._~]*$";
