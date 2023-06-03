Sumerian is a multiplayer strategie game coded in Rust.

It put the emphasis on trade, diplomatie, logistic and tactic.

How to launch:

1-Run the server
Open a terminal, go in the server folder, and type:
cargo run --release 0.0.0.0:4321

2-Run the client
In another terminal, go in the client folder, and type:
cargo run


To deploy with fly.io
1-Server
fly deploy
2-Client
cd client/
cargo build --release --features fly --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_client --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/client.wasm
fly deploy