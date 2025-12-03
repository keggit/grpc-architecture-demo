#!/bin/bash
# Build the WASM module without atomics to avoid SharedArrayBuffer requirements
cd native/hub
RUSTFLAGS='-C target-feature=-atomics' wasm-pack build --target web --out-dir ../../web/pkg --release
