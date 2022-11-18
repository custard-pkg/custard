alias b := build

housekeeping:
    cargo fmt
    cargo update

build:
    cargo build

test:
    cargo nextest run

fmt:
    cargo fmt
