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
}

impl Message {
    //the return String command contain : play, register or map etc
    pub fn from(msg: &str) -> Option<(Message, String)> {
        println!("message: {}", msg);
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
                "positions" => Some(Message::Positions(id)),
                "map" => Some(Message::Map(id)),
                "up" => Some(Message::UP(id)),
                "down" => Some(Message::DOWN(id)),
                "right" => Some(Message::RIGHT(id)),
                "left" => Some(Message::LEFT(id)),
                "player_info" => Some(Message::PlayerInfo(id)),
                "exit" => Some(Message::Exit(id)),
                "pickup" => Some(Message::PickUp(id)),
                "destroy" => Some(Message::Destroy(id)),
                "action" => {
                    let message: Vec<&str> = parts.collect();
                    let name: String = message[0].parse().unwrap();
                    Some(Message::Action(id, name))
                }
                "switch_item" => {
                    let message: Vec<&str> = parts.collect();
                    let idx1: u32 = message[0].parse().unwrap();
                    let idx2: u32 = message[1].parse().unwrap();
                    Some(Message::SwitchItem(id, idx1, idx2))
                }
                "build" => {
                    let build: Vec<&str> = parts.collect();
                    let x: i32 = build[0].parse().unwrap();
                    let y: i32 = build[1].parse().unwrap();
                    let name: String = build[2].parse().unwrap();
                    Some(Message::Build(id, x, y, name))
                }
                "interact" => {
                    let interact: Vec<&str> = parts.collect();
                    let name: String = interact[0].parse().unwrap();
                    let ent_id: u32 = interact[1].parse().unwrap();
                    let gen: i32 = interact[2].parse().unwrap();

                    Some(Message::Interact(id, name, ent_id, gen))
                }
                "consume" => {
                    let interact: Vec<&str> = parts.collect();
                    let ent_id: u32 = interact[0].parse().unwrap();
                    let gen: i32 = interact[1].parse().unwrap();

                    Some(Message::Consume(id, ent_id, gen))
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
