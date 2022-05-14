#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'


pushd frontend;
cargo add \
      anyhow \
      console_error_panic_hook \
      gloo-net \
      gloo-timers \
      gloo-utils \
      serde_json \
      tracing \
      tracing-wasm \
      wasm-bindgen-futures \
      yew \
      yew-router
cargo add serde --features derive
popd

pushd server;
cargo add \
      anyhow \
      axum \
      base64 \
      log \
      serde_json \
      tower \
      tracing \
      tracing-subscriber
cargo add axum-extra --features spa
cargo add clap --features derive
cargo add hyper --features full
cargo add serde --features derive
cargo add tokio --features full
cargo add tower-http --features full
popd
