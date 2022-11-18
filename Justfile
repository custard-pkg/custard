alias b := build
alias r := run

housekeeping:
    cargo fmt
    cargo update
run:
    cargo run
build:
    cargo build
test:
    cargo nextest run
fmt:
    cargo fmt