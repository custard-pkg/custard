# Issues
## **src/consts.rs**
- Should `SCRIPT_SIGNAL_EXIT_CODE` be 2?
- Should we be using functions to return some `String`s, and have consts for others?

## **src/util.rs**
- We need a better name for the 1st argument of the `find_closest_file_or_dir` function than `_current_dir`
- `user_error` requires an `unreachable!` macro to be used if something is meant to be returned