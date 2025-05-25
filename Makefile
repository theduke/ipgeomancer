
fmt:
	cargo fmt --all

lint-fmt:
	@echo "Checking formatting..."
	rustfmt --version
	cargo fmt --all --check


lint-clippy:
	@echo "Running Clippy..."
	cargo clippy --all-targets --all-features -- -D warnings
	@echo "Clippy checks passed."


lint: lint-fmt lint-clippy
	@echo "Linting completed successfully."


check:
	@echo "Running cargo check..."
	cargo check --all-targets --all-features
	@echo "Checks completed successfully."


test:
	@echo "Running tests..."
	cargo test --all-features
	@echo "Tests completed successfully."


ci: lint check test
	@echo "CI checks completed successfully."


setup-os-debian:
	@echo "Installing OS dependencies for Debian..."
	sudo apt-get update
	sudo apt-get install -y build-essential clang libssl-dev pkg-config
	@echo "Dependencies installed successfully."
