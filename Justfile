alias b := build

housekeeping:
    cargo fmt
    cargo update

build:
    cargo build

build-release:
    cargo build --release

test:
    cargo nextest run

fmt:
    cargo fmt
