#!/usr/bin/env bash

set -Eeuxo pipefail

# If rustup is not installed, install it
if ! command -v rustup &> /dev/null; then
    echo "rustup is not installed. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "rustup is already installed."
fi

# Ensure latest stable Rust toolchain is installed
rustup install stable
rustup component add rustfmt clippy

# Fetch Rust dependencies

cargo fetch --locked

# Download test data

mkdir -p data/test/db

# echo "Downloading test data..."

# echo "Downloading RIPE database dump..."
# curl -L https://ftp.ripe.net/ripe/dbase/ripe.db.gz | gunzip -c > data/test/db/ripe.db

# echo "Done!"
