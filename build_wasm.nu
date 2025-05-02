cargo build --release --target wasm32-unknown-unknown --config 'build.rustflags=["--cfg", "getrandom_backend=\"wasm_js\""]'
wasm-bindgen --out-dir www --target web target/wasm32-unknown-unknown/release/minesweeper.wasm
