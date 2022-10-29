# Issues

## General

- The code's kinda bad imo
- `rust-i18n` doesn't seem to be maintained, should we use a new crate?

## **src/consts.rs**

- Should `SCRIPT_SIGNAL_EXIT_CODE` be 2?
- ~~Should we be using functions to return some `String`s, and have consts for others?~~ We now use `rust-i18n`, making this irrelevant.

## **src/util.rs**

- `user_error` requires an `unreachable!` macro to be used if something is meant to be returned

## **src/commands/run/mod.rs**

- `npm run` sets the `NODE` environment variable to the node executable with which npm is executed. Custard doesn't.
- Add `package.json` variables: <https://docs.npmjs.com/cli/v8/using-npm/scripts#packagejson-vars>

## **src/commands/init/mod.rs**

- `npm` guesses a ton of stuff for the `package.json`, we need to do that too
  - use `git config --get remote.origin.url` to get the current Git repository URL
- `npm` is additive, `custard` isn't
