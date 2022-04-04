#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

pushd frontend
trunk build
popd

pushd server
cargo run --release -- --port 8080
popd
