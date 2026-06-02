#!/bin/bash
set -e

export PATH="$HOME/.cargo/bin:$PATH"
rustup target add wasm32-unknown-unknown
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
cargo binstall --no-confirm --targets x86_64-unknown-linux-musl trunk
trunk build --release --public-url=/
