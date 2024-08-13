#!/bin/bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "glulands" ./target/wasm32-unknown-unknown/release/glulands.wasm
tar zcvf glulands_game.tgz out
