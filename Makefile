# Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

# `nextest` is necessary for `make test` & `make unit-test` to work. Install the crate with:
# cargo install cargo-nextest --locked

export RUSTFLAGS=-Dwarnings

.PHONY: default check unit-test test doc all

ALL_FEATURES :=

default: check

check:
	cargo check --all --all-targets --features "${ALL_FEATURES}"
	cargo fmt -- --check
	cargo clippy --all-targets --features "${ALL_FEATURES}" -- -D clippy::all

unit-test:
	cargo nextest run --all --no-default-features

test: unit-test

doc:
	cargo doc --workspace --document-private-items --no-deps

all: check doc test

clean:
	cargo clean
	rm -rf target
