#!/usr/bin/env bash

pushd server
cargo doc \
      -p axum \
      -p axum-extra \
      -p hyper \
      -p tokio \
      -p tower \
      -p tower-http \
      -p tracing \
      -p tracing-subscriber \
      --open --no-deps
popd

pushd frontend
cargo doc \
      -p gloo-net \
      -p gloo-timers \
      -p gloo-utils \
      -p wasm-bindgen-futures \
      -p wasm-logger \
      -p yew \
      -p yew-router \
      --open --no-deps
popd
