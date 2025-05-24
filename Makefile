
fmt:
	cargo fmt --all

lint-fmt:
	@echo "Checking formatting..."
	rustfmt --version
	cargo fmt --all --check


lint-clippy:
	@echo "Running Clippy..."
	cargo clippy --all-targets --all-features -- -D warnings
	cargo clippy --version
	cargo clippy --check -D warnings
	@echo "Clippy checks passed."


lint: lint-fmt lint-clippy
	@echo "Linting completed successfully."
