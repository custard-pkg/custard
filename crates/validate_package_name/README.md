# validate_package_name

This crate validates an npm package name.

## Example

```rust
use validate_package_name::validate;

assert!(validate(&String::from("hello")).is_ok())
assert!(validate(&String::from("hi!")).is_err())
```

## License

Apache License 2.0
