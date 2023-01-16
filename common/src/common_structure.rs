use std::collections::HashMap;

// use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
// use specs_derive::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum Biome {
    #[default]
    Plain,
    Forest,
    Desert,
    Mountain,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub enum Resources {
    #[default]
    Wood,
    Stone,
    Iron,
    Wheat,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientTile {
    pub biome: Biome,
    pub x: i32,
    pub y: i32,
    pub owner: String,
    pub resource: Option<Resources>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, bevy::prelude::Resource)]
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

#[derive(
    specs_derive::Component, bevy::prelude::Resource, Clone, Debug, Default, Serialize, Deserialize,
)]
pub struct PlayerInfo {
    pub gold: f32,
    pub resources: HashMap<Resources, f32>,
}
