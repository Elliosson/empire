use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Message {
    Register(String),
    Registered(Uuid, String),
    Play(Uuid),
    Map(Uuid, i32, i32, i32), //uuid, x, y, scale
    PlayerInfo(Uuid),
    Attack(Uuid, i32, i32, i32), // uuid, x, y, percent
}

impl Message {
    //the return String command contain : play, register or map etc
    pub fn from(msg: &str) -> Option<(Message, String)> {
        if msg.starts_with("register") {
            let mut parts = msg.split_whitespace();
            let _register = parts.next()?;
            let name = parts.next()?;
            Some((Message::Register(name.to_string()), "register".to_string()))
        } else {
            let mut parts = msg.split_whitespace();
            let id = match parts.next()?.parse() {
                Ok(id) => id,
                Err(_) => return None,
            };

            let command = parts.next()?;
            let msg = match command {
                "play" => Some(Message::Play(id)),
                "map" => {
                    let message: Vec<&str> = parts.collect();
                    let x: i32 = message[0].parse().unwrap();
                    let y: i32 = message[1].parse().unwrap();
                    let scale: i32 = message[2].parse().unwrap();
                    Some(Message::Map(id, x, y, scale))
                }
                "attack" => {
                    let message: Vec<&str> = parts.collect();
                    let x: i32 = message[0].parse().unwrap();
                    let y: i32 = message[1].parse().unwrap();
                    let percent: i32 = message[2].parse().unwrap();
                    //println!("attack {} {}", x, y);
                    Some(Message::Attack(id, x, y, percent))
                }
                "player_info" => Some(Message::PlayerInfo(id)),

                _ => None,
            };

            match msg {
                Some(msg) => Some((msg, command.to_string())),
                None => None,
            }
        }
    }
}
