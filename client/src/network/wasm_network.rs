//#[cfg(target_arch = "wasm32")]
use super::general_network;
use super::general_network::handle_responce;
use super::Data;
use std::sync::{Arc, Mutex};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

pub const ASK_DATA_INTERVAL: u32 = 100;

#[cfg(feature = "fly")]
const CONNECTION: &'static str = "wss://sumerserver.fly.dev:443";
#[cfg(not(feature = "fly"))]
const CONNECTION: &'static str = "ws://localhost:4321";

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

//#[cfg(target_arch = "wasm32")]
pub fn start_websocket(
    data: Arc<Mutex<Data>>,
    to_send: Arc<Mutex<Vec<String>>>,
) -> Result<WebSocket, JsValue> {
    // Connect to the game server
    let ws = WebSocket::new(CONNECTION)?;

    console_log!("start websocket {:?}", CONNECTION);

    let cloned_ws = ws.clone();
    //send message to the serveer
    let cb = Closure::wrap(Box::new(move || {
        let mut to_send_guard = to_send.lock().unwrap();

        //list stuff if idx, at first failure, break and keep index.
        //remove all the messages before idx of to_send_guard, keep the others.
        let mut idx = 0;
        for message in to_send_guard.iter() {
            console_log!("trying to send: {:?}", message);
            match cloned_ws.send_with_str(&message) {
                Ok(()) => console_log!("message send"),
                Err(_) => {
                    console_log!("failed to send the message");
                    break;
                }
            }
            idx += 1;
        }
        to_send_guard.drain(0..idx);
    }) as Box<dyn FnMut()>);
    let _interval_id = setInterval(&cb, ASK_DATA_INTERVAL);
    cb.forget();

    let cloned_ws = ws.clone();

    // create callback
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        // handle message

        let response = e
            .data()
            .as_string()
            .expect("Can't convert received data to a string");

        console_log!("message event, received data: {:?}", response);

        let cloned2_ws = cloned_ws.clone();
        let message_sender =
            move |uid, message| match cloned2_ws.send_with_str(&format!("{} {}", uid, message)) {
                Ok(_) => console_log!("message successfully sent: {:?}", message),
                Err(err) => console_log!("error sending message: {:?}", err),
            };
        match handle_responce(response.clone(), data.clone(), message_sender.clone()) {
            Some((general_network::Message::Play(_uid), _)) => {}
            _ => {}
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        console_log!("socket opened");
        match cloned_ws.send_with_str("open com") {
            Ok(_) => console_log!("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(ws)
}
