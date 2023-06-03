use super::message::Message;
use crate::MapMessage;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[cfg(feature = "fly")]
const URL: &'static str = "0.0.0.0:8080";
#[cfg(not(feature = "fly"))]
const URL: &'static str = "0.0.0.0:4321";

pub struct Config {
    pub url: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        Ok(Config {
            url: URL.to_string(),
        })
    }
}

pub fn run(
    config: Config,
    message_list: Arc<Mutex<Vec<(Message, String)>>>,
    map_to_send: Arc<Mutex<MapMessage>>, //TODO formaliser le naming des structure partage
    player_info_to_send: Arc<Mutex<HashMap<String, String>>>,
) {
    ws::listen(config.url, move |out| {
        let message_list = message_list.clone();
        let map_to_send_clone = map_to_send.clone();
        let player_info_to_send_clone = player_info_to_send.clone();

        move |msg: ws::Message| {
            let message_list = message_list.clone();
            let mut message_guard = message_list.lock().unwrap();

            let msg = match msg.as_text() {
                Ok(msg) => msg,
                Err(_) => "",
            };
            // println!("message: {}", msg);

            //push the message in a struct that will be read by the game to do an action

            let response_to_send = match Message::from(msg) {
                //msg is a string in enter
                Some((message, command)) => {
                    // println!("message: {:?}", message);
                    let (response, message_to_system) = response(
                        message,
                        map_to_send_clone.clone(),
                        player_info_to_send_clone.clone(),
                    );
                    message_guard.push((message_to_system, command.clone()));
                    format!("{} {}", command, response)
                }
                None => {
                    // println!("None message");
                    "err".to_string()
                }
            };

            out.send(ws::Message::Text(response_to_send))
        }
    })
    .unwrap();
}

fn response(
    msg: Message,
    map_to_send: Arc<Mutex<MapMessage>>,
    player_info_to_send: Arc<Mutex<HashMap<String, String>>>,
) -> (String, Message) {
    let map_guard = map_to_send.lock().unwrap();
    let player_info_guard = player_info_to_send.lock().unwrap();

    match msg {
        //this is very tricky because since the uuid is created here we don't return the same message that the on we received
        Message::Register(name) => {
            println!("register");
            let uuid = Uuid::new_v4();
            (uuid.to_string(), Message::Registered(uuid, name))
        }
        Message::Map(_uuid) => (map_guard.map_json.clone(), msg),
        Message::PlayerInfo(uuid) => {
            if let Some(my_player_info) = player_info_guard.get(&uuid.to_string()) {
                (my_player_info.clone(), msg) // my_player_info is a string
            } else {
                ("nok".to_string(), msg)
            }
        }
        _ => ("ok".to_string(), msg),
    }
}
