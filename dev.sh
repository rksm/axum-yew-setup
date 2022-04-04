#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

(trap 'kill 0' SIGINT; \
 bash -c 'cd frontend; trunk serve' & \
 bash -c 'cd server; cargo watch -- cargo run -- --port 8081')
