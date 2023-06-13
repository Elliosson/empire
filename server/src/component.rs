use common::{Color, Resources};
use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::*;
use std::{collections::HashMap, time::Instant};

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash, Copy, Serialize, Deserialize, Default)]
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

#[derive(Component, Clone, Debug, Default)]
pub struct TerritoryArea {
    pub area: i32,
}

#[derive(Component, Clone, Debug)]
pub struct WantToAttack {
    pub pos: Position,
    pub gold_percent: i32,
}

#[derive(Component, Clone, Debug)]
pub struct OnGoingAttack {
    pub gold: f32,
    pub last_turn_conquest: Vec<Position>,
    pub owner: String,
    pub enemy: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum GamePhaseEnum {
    #[default]
    LocationSelection,
    Playing,
    GameOver,
}
#[derive(Component, Clone, Debug, Default, Serialize, Deserialize)]
pub struct GamePhase {
    pub phase: GamePhaseEnum,
}

#[derive(Component, Clone, Debug, Default, Serialize)]
pub struct ResourcesStorage {
    pub storage: HashMap<Resources, f32>,
}

#[derive(Component, Clone, Debug, Default, Serialize, Deserialize)]
pub struct BuildedTile {}

#[derive(Component, Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResourceExtractionBuilding {}

#[derive(Component, Clone, Debug, Default, Serialize, Deserialize)]
pub struct WantToBuild {
    pub name: String,
    pub pos: Position,
}

#[derive(Component, Clone, Debug, Default, Serialize, Deserialize)]
pub struct WantMap {
    pub pos: Position,
    pub scale: i32,
}

#[derive(Component, Clone, Debug)]
pub struct GoldGenerationTiming {
    pub last_time: Instant,
}

#[derive(Component, Clone, Debug)]
pub struct TerritoryStatTiming {
    pub last_time: Instant,
}

#[derive(Clone, Debug, Default)]
pub struct DezoomedMap {
    pub hash_map: HashMap<(i32, i32, i32), Color>,
}
