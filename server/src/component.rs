use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        return Position { x, y };
    }
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct LeftMover {}

#[derive(Component, Clone, Debug)]
pub struct Player {
    pub name: String,
}

#[derive(Component, Clone, Debug)]
pub struct Connected {
    pub uuid: String,
}

#[derive(Component, Clone, Debug)]
pub struct Gold {
    pub quantity: f32,
}

#[derive(Component, Clone, Debug)]
pub struct WantToAttack {
    pub pos: Position,
}

#[derive(Component, Clone, Debug)]
pub struct OnGoingAttack {
    pub gold: f32,
    pub last_turn_conquest: Vec<Position>,
    pub owner: String,
    pub enemy: Option<String>,
}

#[derive(Component, Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub gold: f32,
}
