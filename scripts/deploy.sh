#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "Building the release binary..."
cargo build --release --target x86_64-unknown-linux-musl --bin api

printf "\nUploading the release binary...\n"
scp target/x86_64-unknown-linux-musl/release/api usync:/opt/api.ultrasploit.com/app

printf "\nDone!\n"
