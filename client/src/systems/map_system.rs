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
    mut image_handles: Query<&mut Handle<Image>>,
    asset_server: Res<AssetServer>,
) {
    // if let Some(dezoomed_map) = &map.dezoomed_map {
    //     update_dezoomed_map_(commands, dezoomed_map, pos_to_tile_entity, image_handles)
    // } else {
    for tile in map.tiles.values() {
        let mut image = match tile.biome {
            Biome::Plain => asset_server.load("plain.png"),
            Biome::Mountain => asset_server.load("mountain.png"),
            _ => asset_server.load("plain.png"),
        };

        if let Some(resource) = &tile.resource {
            image = match resource {
                Resources::Wood => asset_server.load("forest.png"),
                _ => asset_server.load("plain.png"),
            };
        }

        if let Some(&entity) = pos_to_tile_entity.hash.get(&(tile.x, tile.y)) {
            if let Ok(mut image_handle) = image_handles.get_component_mut::<Handle<Image>>(entity) {
                *image_handle = image;
            } else {
                println!("Bad ServerSate query");
            }
        } else {
            //create entity
            let new_entity = commands
                .spawn(SpriteBundle {
                    texture: image,
                    transform: Transform::from_translation(Vec3::new(
                        (tile.x * 32) as f32,
                        (tile.y * 32) as f32,
                        0.,
                    )),
                    ..default()
                })
                .id();

            pos_to_tile_entity.hash.insert((tile.x, tile.y), new_entity);
        }
    }
    // }
}

fn _calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn _adjacent_tiles(tile: &ClientTile, map: &HashMap<(i32, i32), ClientTile>) -> Vec<ClientTile> {
    let mut result = Vec::new();

    for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if let Some(adjacent_tile) = map.get(&(tile.x + dx, tile.y + dy)) {
            result.push(adjacent_tile.clone())
        }
    }

    return result;
}

// pub fn update_dezoomed_map_(
//     mut commands: Commands,
//     map_level: &MapLevel,
//     mut pos_to_tile_entity: ResMut<PositionToTileEntity>,
//     mut image_handles: Query<&mut Handle<Image>>,
// ) {
//     let level: i32 = map_level.level;
//     let scale = 8_i32.pow((level - 1) as u32);
//     for ((x, y), net_color) in map_level.map.iter() {
//         let color = Color::rgb(net_color.r, net_color.g, net_color.b);

//         if let Some(&entity) = pos_to_tile_entity.dezoomed_map.get(&(*x, *y, scale)) {
//             // if let Ok(mut sprite) = sprite_query.get_component_mut::<Sprite>(entity) {
//             //     sprite.color = color;
//             // } else {
//             //     println!("Bad ServerSate query");
//             // }
//         } else {
//             //create entity
//             let new_entity = commands
//                 .spawn(SpriteBundle {
//                     sprite: Sprite {
//                         color,
//                         custom_size: Some(Vec2::new(10.0 * scale as f32, 10.0 * scale as f32)),
//                         ..default()
//                     },
//                     transform: Transform::from_translation(Vec3::new(
//                         (x * 32) as f32,
//                         (y * 32) as f32,
//                         -level as f32,
//                     )),
//                     ..default()
//                 })
//                 .id();

//             pos_to_tile_entity
//                 .dezoomed_map
//                 .insert((*x, *y, scale), new_entity);
//         }
//     }
// }
