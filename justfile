set shell := ["bash", "-euxo", "pipefail", "-c"]

ci: fmt fmt-check clippy check test

fmt:
    cargo +nightly fmt

fmt-check:
    cargo +nightly fmt -- --check

clippy:
    cargo +stable clippy --all-targets -- -D warnings

check:
    cargo +stable check --workspace

test:
    cargo +stable test --workspace

run *args:
    cargo +stable run -p flow -- {{args}}

