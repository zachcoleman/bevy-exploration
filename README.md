# bevy-exploration
Exploring the Rust game engine Bevy through a simple tower defense demo. Features unique to this demo include:
- A custom world camera (RTS-style)
- Implementation of a static quad tree
- Limited flocking algorithm for enemies 

The static quad tree is on in the current implementation deployed to the webpage.

## Using Demo / Controls

### Camera
The camera has a cursor (a white sphere). Movement consists of:
- `WASD` for xz-translation (forwards/backwards/sideways)
- `QE` for rotation
- _Scrolling_ for zoom
- `Space/L-Shift` for y-translation (vertical)

### Spawn Towers
Pressing `t` with a selected hexagon will spawn a tower.

### Spawn Enemies
Pressing `x` will spawn a group of 10 enemies around the cursor.


## Web Development 

### Run Locally with `wasm-server-runner`

```sh
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner
cargo run --target wasm32-unknown-unknown -r 
```

### Deploy To GitHub Pages

#### Install `wasm-bindgen` and `wasm` target
```
cargo install -f wasm-bindgen-cli
rustup target install wasm32-unknown-unknown
```

#### Build and Deploy Necessary Files to `website/`
```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./website/ --target web ./target/wasm32-unknown-unknown/release/bevy-exploration.wasm
rm -rf ./website/assets
cp -R ./assets ./website
```
