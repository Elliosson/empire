extern crate websocket;

use std::sync::mpsc::{channel, TryRecvError};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use super::general_network;
use crate::Data;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

const CONNECTION: &'static str = "ws://localhost:4321";

pub fn start_websocket(data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    println!("Connecting to {}", CONNECTION);

    let client = ClientBuilder::new(CONNECTION)
        .unwrap()
        .add_protocol("rust-websocket")
        .connect_insecure()
        .unwrap();

    println!("Successfully connected");

    let (mut receiver, mut sender) = client.split().unwrap();

    let (tx, rx): (Sender<OwnedMessage>, Receiver<OwnedMessage>) = channel();
    let (asker_tx, asker_rx): (Sender<String>, Receiver<String>) = channel();

    let tx_1 = tx.clone();

    let _to_send_loop = thread::spawn(move || loop {
        {
            let mut to_send_guard = to_send.lock().unwrap();

            for message in to_send_guard.drain(..) {
                println!("send: {}", message.clone());

                let _ = tx_1.send(OwnedMessage::Text(message));
            }
        }
        thread::sleep(Duration::from_millis(50));
    });

    let _send_loop = thread::spawn(move || {
        loop {
            // Send loop
            let message = match rx.recv() {
                Ok(m) => m,
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    let _ = sender.send_message(&message);
                    // If it's a close message, just send it and then return.
                    return;
                }
                _ => (),
            }
            // Send the message
            match sender.send_message(&message) {
                Ok(()) => (),
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    let _ = sender.send_message(&Message::close());
                    return;
                }
            }
        }
    });

    let tx_2 = tx.clone();
    let _receive_loop = thread::spawn(move || {
        // Receive loop
        for message in receiver.incoming_messages() {
            let message = match message {
                Ok(m) => m,
                Err(e) => {
                    println!("Receive Loop: {:?}", e);
                    let _ = tx_2.send(OwnedMessage::Close(None));
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    // Got a close message, so send a close message and return
                    let _ = tx_2.send(OwnedMessage::Close(None));
                    return;
                }
                OwnedMessage::Ping(data) => {
                    match tx_2.send(OwnedMessage::Pong(data)) {
                        // Send a pong in response
                        Ok(()) => (),
                        Err(e) => {
                            println!("Receive Loop: {:?}", e);
                            return;
                        }
                    }
                }
                // Say what we received
                OwnedMessage::Text(message) => {
                    let tx_3 = tx_2.clone();
                    let message_sender = move |uid, message| {
                        tx_3.send(OwnedMessage::Text(format!("{} {}", uid, message)))
                            .expect("Unable to send message to channel");
                    };
                    match general_network::handle_responce(
                        message.clone(),
                        data.clone(),
                        message_sender.clone(),
                    ) {
                        Some((general_network::Message::Play(uid), _)) => {
                            asker_tx
                                .send(uid)
                                .expect("Unable to send message to channel");
                        }
                        _ => {}
                    }
                }
                _ => {
                    println!("Receive Loop: {:?}", message);
                }
            }
        }
    });

    let tx_4 = tx.clone();
    let message_sender = move |uid, message| {
        tx_4.send(OwnedMessage::Text(format!("{} {}", uid, message)))
            .expect("Unable to send message to channel");
    };
    //thread that periodically ask basic stuff like player info and the map
    let _asker_loop = thread::spawn(move || {
        let mut maybe_uuid = None;

        loop {
            //update uuid
            match asker_rx.try_recv() {
                Ok(new_uuid) => {
                    println!("Change uuid.");
                    maybe_uuid = Some(new_uuid);
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => break,
            }

            if let Some(uuid) = &maybe_uuid {
                message_sender(uuid.clone(), "map".to_string());
                message_sender(uuid.clone(), "player_info".to_string());
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}
