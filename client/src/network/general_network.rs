use crate::Data;
use std::sync::{Arc, Mutex};

pub enum Message {
    Register,
    Play(String),
    Config,
    Map,
    Unknown,
}

pub fn handle_responce<F>(
    msg: String,
    data: Arc<Mutex<Data>>,
    message_sender: F,
) -> Option<(Message, String)>
where
    F: Fn(String, String) + Clone + 'static,
{
    println!("incoming message: {}", msg);
    let mut parts = msg.split_whitespace();

    let command = parts.next()?;

    let message = match command {
        "register" => {
            let mut data_guard = data.lock().unwrap();
            let id = parts.next()?.to_string();
            data_guard.my_uid = id.clone();
            message_sender(id, "play".to_string());

            Message::Register
        }
        "play" => {
            if parts.next()? == "ok" {
                let data_guard = data.lock().unwrap();
                let uid = data_guard.my_uid.clone();
                message_sender(uid.clone(), "config".to_string());
                message_sender(uid.clone(), "side".to_string());
                //start game
                Message::Play(uid.clone())
            } else {
                Message::Unknown
            }
        }
        "config" => {
            //lot of stuff
            Message::Config
        }
        "map" => {
            let mut data_guard = data.lock().unwrap();

            let map: String = parts.collect::<String>();

            data_guard.map_string = map;
            Message::Map
        }
        _ => Message::Unknown,
    };
    return Some((message, "done".to_string()));
}
