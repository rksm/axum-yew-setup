#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

pushd frontend
CARGO_TARGET_DIR=../target-trunk trunk build --release --public-url /assets/
popd

pushd server
cargo run --release -- --port 8080
popd
