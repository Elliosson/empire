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
use std::{process, time};
use systems::*;
mod systems;

pub const TICK_TIME: time::Duration = time::Duration::from_millis(50);

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, _: &mut Rltk) {}
}

fn common_tick(gs: &mut State) {
    let start = time::Instant::now();

    gs.run_systems();

    let end = time::Instant::now();

    let time_spend = end - start;

    if TICK_TIME > time_spend {
        let time_left = TICK_TIME - time_spend;
        thread::sleep(time_left);
    } else {
        println!("WARNING: tick is too slow ! : {:?}", time_spend);
    }
}

impl State {
    fn run_systems(&mut self) {
        let start = time::Instant::now();

        let mut online_player = OnlinePlayerSystem {};
        online_player.run_now(&self.ecs);
        println!("onl time : {:?}", time::Instant::now() - start);
        let mut attack = AttackSystem {};
        attack.run_now(&self.ecs);
        println!("att time: {:?}", time::Instant::now() - start);

        let mut ongoing_attack = OngoingAttackSystem {};
        ongoing_attack.run_now(&self.ecs);
        println!("ong att time: {:?}", time::Instant::now() - start);

        let mut gold_generation = GoldGenerationSystem {};
        gold_generation.run_now(&self.ecs);
        println!("gold gen time : {:?}", time::Instant::now() - start);

        let mut resources_generation = ResourceGenerationSystem {};
        resources_generation.run_now(&self.ecs);
        let mut build = BuildSystem {};
        build.run_now(&self.ecs);
        println!("build time : {:?}", time::Instant::now() - start);

        let mut territory_stat = TerritoryStatSystem {};
        territory_stat.run_now(&self.ecs);
        println!("territory stat time : {:?}", time::Instant::now() - start);

        let mut defeat = DefeatSystem {};
        defeat.run_now(&self.ecs);
        println!("defeat time : {:?}", time::Instant::now() - start);

        let mut player_info = PlayerInfoSystem {};
        player_info.run_now(&self.ecs);
        println!("pinfo time : {:?}", time::Instant::now() - start);

        let mut send_map = SendMapSystem {};
        send_map.run_now(&self.ecs);

        let mut player_info_json = PlayerInfoJsonSystem {};
        player_info_json.run_now(&self.ecs);
        println!("pjson time : {:?}", time::Instant::now() - start);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
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
    gs.ecs.register::<TerritoryArea>();
    gs.ecs.register::<WantMap>();

    let config = Config::new().unwrap_or_else(|err| {
        println!("Error creating Config: {}", err);
        println!("Usage: server url");
        process::exit(1);
    });

    println!("url: {}", config.url);
    let message_list: Arc<Mutex<Vec<(network::Message, String)>>> =
        Arc::new(Mutex::new(Vec::new()));

    let map_to_send: Arc<Mutex<HashMap<String, MapMessage>>> = Arc::new(Mutex::new(HashMap::new()));

    let player_info_to_send: Arc<Mutex<HashMap<String, String>>> =
        Arc::new(Mutex::new(HashMap::new()));

    gs.ecs.insert(message_list.clone());
    gs.ecs.insert(map_to_send.clone());
    gs.ecs.insert(player_info_to_send.clone());

    thread::spawn(move || {
        network::run(config, message_list, map_to_send, player_info_to_send);
    });

    loop {
        common_tick(&mut gs);
    }
}
