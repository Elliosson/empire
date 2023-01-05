use crate::{Biome, Map, PositionToTileEntity};
use bevy::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn map_system(
    mut commands: Commands,
    map: Res<Map>,
    mut pos_to_tile_entity: ResMut<PositionToTileEntity>,
    mut sprite_query: Query<&mut Sprite>,
) {
    for tile in map.tiles.iter() {
        let mut color = match tile.biome {
            Biome::Plain => Color::rgb(0.25, 0.75, 0.25),
            _ => Color::rgb(0.25, 0.25, 0.75),
        };

        if tile.owner != "" {
            let hash = calculate_hash(&tile.owner);
            color = Color::rgb(
                (hash % 100) as f32 / 100.,
                (hash / 100 % 100) as f32 / 100.,
                (hash / 100000 % 100) as f32 / 100.,
            );
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
