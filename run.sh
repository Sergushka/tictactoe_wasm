#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.
set -eux

wasm-pack build --out-dir build --release --target web
cp ./index.html ./build/
npx serve ./build/