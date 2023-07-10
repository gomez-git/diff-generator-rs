install:
	@cargo install --path .

uninstall:
	@cargo uninstall

check-lint:
	@cargo clippy -- -D warnings

test:
	@cargo test

check-format:
	@cargo fmt --check

ci: check-lint check-format test

format:
	@cargo fmt
