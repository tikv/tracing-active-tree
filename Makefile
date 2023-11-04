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
