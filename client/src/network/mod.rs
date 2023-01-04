use crate::Data;
use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
mod desktop_network;
mod general_network;
#[cfg(target_arch = "wasm32")]
mod wasm_network;

#[cfg(target_arch = "wasm32")]
pub fn lauch_network(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    wasm_network::start_websocket(protect_data, to_send).expect("Unable to start websocket");
}

#[cfg(not(target_arch = "wasm32"))]
pub fn lauch_network(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    desktop_network::start_websocket(protect_data, to_send);
}
