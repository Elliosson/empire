use bevy::prelude::Resource;
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
pub struct Tile {
    pub biome: Biome,
    pub x: i32,
    pub y: i32,
}

#[derive(Resource, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<Tile>,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Renderable {
    pub glyph: u8,
    pub render_order: i32,
}
