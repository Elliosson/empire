use std::collections::HashMap;

// use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
// use specs_derive::*;
use serde_with::serde_as;

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

#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize, bevy::prelude::Resource)]
pub struct ClientMap {
    #[serde_as(as = "Vec<(_, _)>")]
    pub tiles: HashMap<(i32, i32), ClientTile>,
    pub dezoomed_map: Option<MapLevel>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MapMessage {
    pub map_json: String,
}

pub struct PlayerInfoMessage {
    pub json: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildingInfo {
    pub name: String,
    pub x: i32,
    pub y: i32,
}

#[derive(
    specs_derive::Component, bevy::prelude::Resource, Clone, Debug, Default, Serialize, Deserialize,
)]
pub struct PlayerInfo {
    pub gold: f32,
    pub resources: HashMap<Resources, f32>,
    pub buildings: Vec<BuildingInfo>,
    pub player_to_golds: HashMap<String, f32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MapLevel {
    pub level: i32,
    #[serde_as(as = "Vec<(_, _)>")]
    pub map: HashMap<(i32, i32), Color>,
}
