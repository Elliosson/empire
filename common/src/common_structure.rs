use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum Biome {
    #[default]
    Plain,
    Forest,
    Desert,
    Mountain,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientTile {
    pub biome: Biome,
    pub x: i32,
    pub y: i32,
    pub owner: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct ClientMap {
    pub tiles: Vec<ClientTile>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MapMessage {
    pub map_json: String,
}

pub struct PlayerInfoMessage {
    pub json: String,
}

#[derive(Resource, Default, Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
    pub gold: f32,
}
