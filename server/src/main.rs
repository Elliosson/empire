use common::{MapMessage, PlayerInfo};
use rltk::{GameState, Rltk};
use specs::prelude::*;

mod map;
pub use map::*;
mod component;
pub use component::*;
mod network;
use network::Config;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, process};
use systems::*;
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
        let mut online_player = OnlinePlayerSystem {};
        online_player.run_now(&self.ecs);
        let mut attack = AttackSystem {};
        attack.run_now(&self.ecs);
        let mut ongoing_attack = OngoingAttackSystem {};
        ongoing_attack.run_now(&self.ecs);
        let mut gold_generation = GoldGenerationSystem {};
        gold_generation.run_now(&self.ecs);
        let mut resources_generation = ResourceGenerationSystem {};
        resources_generation.run_now(&self.ecs);
        let mut build = BuildSystem {};
        build.run_now(&self.ecs);
        let mut defeat = DefeatSystem {};
        defeat.run_now(&self.ecs);
        let mut player_info = PlayerInfoSystem {};
        player_info.run_now(&self.ecs);
        let mut player_info_json = PlayerInfoJsonSystem {};
        player_info_json.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Sumerian").build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.insert(new_map());
    gs.ecs.insert(UuidPlayerHash::new());
    gs.ecs.insert(NamePlayerHash::new());

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<OnGoingAttack>();
    gs.ecs.register::<WantToAttack>();
    gs.ecs.register::<Gold>();
    gs.ecs.register::<Connected>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<PlayerInfo>();
    gs.ecs.register::<GamePhase>();
    gs.ecs.register::<ResourcesStorage>();
    gs.ecs.register::<BuildedTile>();
    gs.ecs.register::<ResourceExtractionBuilding>();
    gs.ecs.register::<WantToBuild>();

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error creating Config: {}", err);
        println!("Usage: server url");
        process::exit(1);
    });

    println!("url: {}", config.url);
    let message_list: Arc<Mutex<Vec<(network::Message, String)>>> =
        Arc::new(Mutex::new(Vec::new()));

    let map_message = MapMessage::default();

    let map_to_send: Arc<Mutex<MapMessage>> = Arc::new(Mutex::new(map_message));

    let player_info_to_send: Arc<Mutex<HashMap<String, String>>> =
        Arc::new(Mutex::new(HashMap::new()));

    gs.ecs.insert(message_list.clone());
    gs.ecs.insert(map_to_send.clone());
    gs.ecs.insert(player_info_to_send.clone());

    thread::spawn(move || {
        network::run(config, message_list, map_to_send, player_info_to_send);
    });

    rltk::main_loop(context, gs)
}
