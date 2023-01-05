use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod map;
pub use map::*;
mod component;
pub use component::*;
mod left_walker_system;
pub use left_walker_system::*;
mod network;
use network::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, process};
mod systems;

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.run_systems();

        let map = self.ecs.fetch::<Map>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

pub fn format_map_for_client(map: &Map) -> MapForClient {
    let mut client_map: MapForClient = MapForClient::default();

    for (i, tile) in map.tiles.iter().enumerate() {
        let (x, y) = idx_xy(i);
        client_map.tiles.push(TileForClient {
            biome: tile.biome.clone(),
            x,
            y,
        });
    }

    return client_map;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TileForClient {
    pub biome: Biome,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MapForClient {
    pub tiles: Vec<TileForClient>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MapMessage {
    pub map: Map,
    pub map_json: String,
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Sumerian").build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.insert(new_map());

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error creating Config: {}", err);
        println!("Usage: server url");
        process::exit(1);
    });

    println!("url: {}", config.url);
    let message_list: Arc<Mutex<Vec<(network::Message, String)>>> =
        Arc::new(Mutex::new(Vec::new()));

    //quickly set something for test
    let mut map_message = MapMessage::default();
    map_message.map = new_map();
    map_message.map_json = serde_json::to_string(&format_map_for_client(&map_message.map)).unwrap();

    let map_to_send: Arc<Mutex<MapMessage>> = Arc::new(Mutex::new(map_message));

    let player_info_to_send: Arc<Mutex<HashMap<String, String>>> =
        Arc::new(Mutex::new(HashMap::new()));

    gs.ecs.insert(message_list.clone());

    gs.ecs.insert(map_to_send.clone());
    gs.ecs.insert(player_info_to_send.clone());

    thread::spawn(move || {
        network::run(config, message_list, map_to_send, player_info_to_send);
    });

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }

    rltk::main_loop(context, gs)
}
