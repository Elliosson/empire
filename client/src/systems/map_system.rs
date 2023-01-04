use crate::{Biome, Map};
use bevy::prelude::*;

pub fn map_system(mut commands: Commands, map: Res<Map>) {
    for tile in map.tiles.iter() {
        let color = match tile.biome {
            Biome::Plain => Color::rgb(0.25, 0.75, 0.25),
            _ => Color::rgb(0.25, 0.25, 0.75),
        };
        commands.spawn(SpriteBundle {
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
        });
    }
}
