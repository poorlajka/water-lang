set shell := ["bash", "-euxo", "pipefail", "-c"]

default: fmt clippy check test

fmt:
    cargo fmt
    cargo fmt -- --check

clippy:
    cargo clippy --all-targets -- -D warnings

check:
    cargo check --all-targets

test:
    cargo test --all
