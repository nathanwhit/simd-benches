#!/bin/bash -e

export RUSTFLAGS="-C target-feature=+simd128"
cargo build -p simd-benches --bin simd-benches --profile bench --features unstable --target wasm32-wasi
F=./target/wasm32-wasi/release/simd-benches.wasm

wasmer -V
wasmer run --enable-all $F
echo

wasmtime -V
wasmtime --wasm-features simd $F
echo

echo "node" "$(node -v)"
node --experimental-wasi-unstable-preview1 ./scripts/node-wasi.js $F
echo
