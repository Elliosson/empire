use crate::PositionToTileEntity;
use bevy::prelude::*;
use common::{Biome, ClientMap, ClientTile, Resources};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub fn map_system(
    mut commands: Commands,
    map: Res<ClientMap>,
    mut pos_to_tile_entity: ResMut<PositionToTileEntity>,
    mut sprite_query: Query<&mut Sprite>,
) {
    for tile in map.tiles.values() {
        let mut color = match tile.biome {
            Biome::Plain => Color::rgb(0.25, 0.75, 0.25),
            _ => Color::rgb(0.25, 0.25, 0.75),
        };

        if let Some(resource) = &tile.resource {
            color = match resource {
                Resources::Wood => Color::rgb(0.75, 0.75, 0.75),
                _ => Color::rgb(0.75, 0.0, 0.0),
            };
        }

        if tile.owner != "" {
            let hash = calculate_hash(&tile.owner);
            let player_color = Color::rgb(
                (hash % 100) as f32 / 100.,
                (hash / 100 % 100) as f32 / 100.,
                (hash / 100000 % 100) as f32 / 100.,
            );
            color = color * 0.1 + player_color * 0.9;

            for adjacent_tile in adjacent_tiles(tile, &map.tiles) {
                if adjacent_tile.owner != tile.owner {
                    color = Color::rgb(0.1, 0.1, 0.1);
                }
            }
        }

        if let Some(&entity) = pos_to_tile_entity.hash.get(&(tile.x, tile.y)) {
            if let Ok(mut sprite) = sprite_query.get_component_mut::<Sprite>(entity) {
                sprite.color = color;
            } else {
                println!("Bad ServerSate query");
            }
        } else {
            //create entity
            let new_entity = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        (tile.x * 10) as f32,
                        (tile.y * 10) as f32,
                        0.,
                    )),
                    ..default()
                })
                .id();

            pos_to_tile_entity.hash.insert((tile.x, tile.y), new_entity);
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn adjacent_tiles(tile: &ClientTile, map: &HashMap<(i32, i32), ClientTile>) -> Vec<ClientTile> {
    let mut result = Vec::new();

    for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if let Some(adjacent_tile) = map.get(&(tile.x + dx, tile.y + dy)) {
            result.push(adjacent_tile.clone())
        }
    }

    return result;
}
