# Issues

## General
- The code's kinda bad imo
- `rust-i18n` doesn't seem to be maintained, should we use a new crate?
## **src/consts.rs**
- Should `SCRIPT_SIGNAL_EXIT_CODE` be 2?
- ~~Should we be using functions to return some `String`s, and have consts for others?~~ We now use `rust-i18n`, making this irrelevant.

## **src/util.rs**
- We need a better name for the 1st argument of the `find_closest_file_or_dir` function than `_current_dir`
- `user_error` requires an `unreachable!` macro to be used if something is meant to be returned

## **src/run/mod.rs**
- Add `package.json` variables: https://docs.npmjs.com/cli/v8/using-npm/scripts#packagejson-vars