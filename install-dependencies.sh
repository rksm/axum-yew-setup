#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'


pushd frontend;
cargo add \
      anyhow \
      console_error_panic_hook \
      env_logger \
      log \
      gloo-net \
      gloo-timers \
      gloo-utils \
      serde +derive \
      serde_json \
      wasm-bindgen-futures \
      wasm-logger \
      yew \
      yew-router
popd

pushd server;
cargo add \
      anyhow \
      axum \
      axum-extra +spa \
      base64 \
      clap +derive \
      log \
      hyper +full \
      serde +derive \
      serde_json \
      tokio +full \
      tower \
      tower-http +full \
      tracing \
      tracing-subscriber
popd
