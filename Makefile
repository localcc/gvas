.PHONY: ready fmt format lint test clean
ready: format lint test
fmt: format
format: # Formats all bin and lib files of the current crate using rustfmt.
	cargo fmt --all
lint: # Run rustfmt and clippy lints.
	cargo fmt --check --all
	cargo clippy --all-targets --all-features -- -D warnings
test: # Run all tests.
	cargo build --all-targets --verbose --workspace
	cargo test --all-targets --verbose --workspace
	cargo build --all-targets --all-features --verbose --workspace
	cargo test --all-targets --all-features --verbose --workspace
clean:
	cargo clean
