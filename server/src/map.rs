use crate::Position;
use common::Biome;
use rltk::{Rltk, RGB};
use serde::{Deserialize, Serialize};

pub const MAPWIDTH: i32 = 80;
pub const MAPHEIGHT: i32 = 50;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tile {
    pub biome: Biome,
    pub owner: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<Tile>,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y * MAPWIDTH + x) as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    let x = (idx % MAPWIDTH as usize) as i32;
    let y = (idx / MAPWIDTH as usize) as i32;

    return (x, y);
}

pub fn new_map() -> Map {
    let mut map = Map::default();
    map.tiles = vec![Tile::default(); MAPWIDTH as usize * 50];

    // Make the boundaries mountain
    for x in 0..MAPWIDTH {
        map.tiles[xy_idx(x, 0)].biome = Biome::Mountain;
        map.tiles[xy_idx(x, MAPHEIGHT - 1)].biome = Biome::Mountain;
    }
    for y in 0..MAPHEIGHT {
        map.tiles[xy_idx(0, y)].biome = Biome::Mountain;
        map.tiles[xy_idx(MAPWIDTH - 1, y)].biome = Biome::Mountain;
    }

    // Now we'll randomly splat a bunch of desert.
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, MAPWIDTH - 1);
        let y = rng.roll_dice(1, MAPHEIGHT - 1);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map.tiles[idx].biome = Biome::Desert;
        }
    }

    map
}

pub fn draw_map(map: &Map, ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.tiles.iter() {
        // Render a tile depending upon the tile type
        match tile.biome {
            Biome::Plain => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            Biome::Mountain => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
            _ => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('-'),
                );
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

impl Map {
    pub fn get_tile_mut(&mut self, position: &Position) -> &mut Tile {
        if position.x < 0 || position.y < 0 {
            //TODO handle this stuff.
            return &mut self.tiles[0];
        }
        let idx = xy_idx(position.x, position.y);
        if idx >= self.tiles.len() {
            println!("Error: request tile out of bound, handle this case");
            //TODO handle when out of bound stuff are asked.
            return &mut self.tiles[0];
        }
        return &mut self.tiles[idx];
    }
    pub fn get_tile(&self, position: &Position) -> &Tile {
        if position.x < 0 || position.y < 0 {
            //TODO handle this stuff.
            return &self.tiles[0];
        }
        let idx = xy_idx(position.x, position.y);
        if idx >= self.tiles.len() {
            println!("Error: request tile out of bound, handle this case");
            //TODO handle when out of bound stuff are asked.
            return &self.tiles[0];
        }
        return &self.tiles[idx];
    }
}

pub fn adjacent_positions(pos: &Position) -> Vec<Position> {
    let x = pos.x;
    let y = pos.y;

    let mut result = vec![
        Position::new(x, y - 1),
        Position::new(x, y + 1),
        Position::new(x - 1, y),
        Position::new(x + 1, y),
    ];
    result.retain(|&x| is_inside_map(x));
    return result;
}

pub fn is_inside_map(pos: Position) -> bool {
    if pos.x < 0 || pos.x >= MAPWIDTH || pos.y < 0 || pos.y >= MAPHEIGHT {
        return false;
    }
    return true;
}
