//#[cfg(target_arch = "wasm32")]
use super::general_network;
use super::general_network::handle_responce;
use super::Data;
use std::sync::{Arc, Mutex};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

pub const ASK_DATA_INTERVAL: u32 = 100;

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
    let url = "wss://sumserver235235.fly.dev:443";
    // let url = "ws://localhost:8082";
    let ws = WebSocket::new(url)?;
    //let ws = WebSocket::new("ws://51.68.141.5:4321")?;

    console_log!("start websocket {:?}", url);

    let cloned_ws = ws.clone();
    //send message to the serveer
    let cb = Closure::wrap(Box::new(move || {
        let mut to_send_guard = to_send.lock().unwrap();

        for message in to_send_guard.drain(..) {
            cloned_ws
                .send_with_str(&message)
                .expect("Unable to send message");
        }
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
            Some((general_network::Message::Play(uid), _)) => {
                let message_sender2 = message_sender.clone();

                // this closure will regulary send request to the server
                let cb = Closure::wrap(Box::new(move || {
                    message_sender2(uid.clone(), "map".to_string());
                    message_sender2(uid.clone(), "player_info".to_string());
                    console_log!("ask map and player_info");
                }) as Box<dyn FnMut()>);
                let _interval_id = setInterval(&cb, ASK_DATA_INTERVAL);
                cb.forget();
            }
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
