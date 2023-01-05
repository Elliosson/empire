use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Message {
    Register(String),
    Registered(Uuid, String),
    Play(Uuid),
    Positions(Uuid),
    UP(Uuid),
    DOWN(Uuid),
    RIGHT(Uuid),
    LEFT(Uuid),
    Exit(Uuid),
    Map(Uuid),
    PickUp(Uuid),
    Interact(Uuid, String, u32, i32), //x, y, name, id, gen
    Consume(Uuid, u32, i32),          //id, gen
    PlayerInfo(Uuid),
    Build(Uuid, i32, i32, String),
    SwitchItem(Uuid, u32, u32),
    Destroy(Uuid),
    Action(Uuid, String),
    Attack(Uuid, i32, i32),
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
                "map" => Some(Message::Map(id)),
                "attack" => {
                    let message: Vec<&str> = parts.collect();
                    let x: i32 = message[0].parse().unwrap();
                    let y: i32 = message[1].parse().unwrap();
                    println!("attack {} {}", x, y);
                    Some(Message::Attack(id, x, y))
                }

                _ => None,
            };

            match msg {
                Some(msg) => Some((msg, command.to_string())),
                None => None,
            }
        }
    }
}
