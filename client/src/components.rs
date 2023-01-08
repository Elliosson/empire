use bevy::prelude::*;
use bevy::{prelude::Resource, utils::HashMap};

#[derive(Debug, Clone, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Renderable {
    pub glyph: u8,
    pub render_order: i32,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct PositionToTileEntity {
    pub hash: HashMap<(i32, i32), Entity>,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct UiState {
    pub username: String,
    pub gold_percent: i32,
    pub attack_ui_open: bool,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct MapClick {
    pub screen_pos: Point,
    pub map_pos: Point,
}
