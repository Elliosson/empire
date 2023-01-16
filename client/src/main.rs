//! Shows how to render simple primitive shapes with a single color.

use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
mod components;
mod network;
use common::{ClientMap, PlayerInfo};
pub use components::*;
mod systems;
pub use systems::*;
mod helper;
//add default and stuff?
#[derive(Resource)]
pub struct DataWrap {
    protected_data: Arc<Mutex<Data>>,
}
#[derive(Resource)]
pub struct ToSendWrap {
    to_send: Arc<Mutex<Vec<String>>>,
}

pub struct Data {
    pub characters: Vec<Point>,
    pub my_uid: String,
    pub map: Vec<(u32, i32, Point, Renderable)>,
    pub info_string: String,
    pub map_string: String,
}

fn main() {
    //Shared data between the network and the game system
    let data = Data {
        characters: vec![],
        my_uid: "".to_string(),
        map: vec![],
        info_string: "".to_string(),
        map_string: "".to_string(),
    };
    let data_wrap = DataWrap {
        protected_data: Arc::new(Mutex::new(data)),
    };

    let to_send: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    network::lauch_network(data_wrap.protected_data.clone(), to_send.clone());
    {
        let mut to_send_guard = to_send.lock().unwrap();
        to_send_guard.push("a5764857-ae35-34dc-8f25-a9c9e73aa898 map".to_string());
    }

    let to_send_wrap = ToSendWrap {
        to_send: to_send.clone(),
    };

    let map = ClientMap::default();
    let pos_to_entity = PositionToTileEntity::default();

    {
        //TODO make proper register system
        let mut to_send_guard = to_send.lock().unwrap();
        to_send_guard.push(format!("register {}", "test"));
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(map)
        .insert_resource(data_wrap)
        .insert_resource(pos_to_entity)
        .insert_resource(to_send_wrap)
        .insert_resource(UiState::default())
        .insert_resource(PlayerInfo::default())
        .insert_resource(MapClick::default())
        .add_startup_system(setup)
        .add_system(move_camera_system)
        .add_system(deserialize_map_system)
        .add_system(deserialise_player_info_system)
        .add_system(map_system)
        .add_system(mouse_input_system)
        .add_system(username_ui)
        .add_system(attack_ui)
        .add_system(building_ui)
        .add_system(cursor_position_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
