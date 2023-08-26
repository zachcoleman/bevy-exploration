# bevy-exploration
Exploring the Rust game engine Bevy

## Deploy To GitHub Pages

### Install `wasm-bindgen` and `wasm` target
```
cargo install -f wasm-bindgen-cli
rustup target install wasm32-unknown-unknown
```

### Build and Deploy Necessary Files to `out/`
```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy-exploration.wasm
```