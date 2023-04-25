cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_example \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/release/client.wasm