# Issues

## General

- The code's kinda bad imo

## **src/consts.rs**

- Should `SCRIPT_SIGNAL_EXIT_CODE` be 2?

## **src/util.rs**

- `user_error` requires an `unreachable!` macro to be used if something is meant to be returned

## **src/commands/run/mod.rs**

- `npm run` sets the `NODE` environment variable to the node executable with which npm is executed. Custard doesn't.
- Add `package.json` variables: <https://docs.npmjs.com/cli/v8/using-npm/scripts#packagejson-vars>

## **src/commands/init/mod.rs**

- `npm` guesses a ton of stuff for the `package.json`, we need to do that too
  - [x] use `git config --get remote.origin.url` to get the current Git repository URL
- `npm` is additive, `custard` isn't
