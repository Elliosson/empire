//! Shows how to render simple primitive shapes with a single color.

use std::sync::{Arc, Mutex};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
mod components;
mod network;
pub use components::*;
pub struct Data {
    pub characters: Vec<Point>,
    pub my_uid: String,
    pub map: Vec<(u32, i32, Point, Renderable)>,
    pub info_string: String,
}

fn main() {
    //Shared data between the network and the game system
    let data = Data {
        characters: vec![],
        my_uid: "".to_string(),
        map: vec![],
        info_string: "".to_string(),
    };
    let protect_data: Arc<Mutex<Data>> = Arc::new(Mutex::new(data));
    let to_send: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    network::lauch_network(protect_data.clone(), to_send.clone());

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<&mut Transform, With<Camera2d>>,
) {
    if keyboard_input.pressed(KeyCode::Up) {
        for mut transform in transforms.iter_mut() {
            transform.translation.y += 10.;
        }
    }
    if keyboard_input.pressed(KeyCode::Down) {
        for mut transform in transforms.iter_mut() {
            transform.translation.y -= 10.;
        }
    }
    if keyboard_input.pressed(KeyCode::Right) {
        for mut transform in transforms.iter_mut() {
            transform.translation.x += 10.;
        }
    }
    if keyboard_input.pressed(KeyCode::Left) {
        for mut transform in transforms.iter_mut() {
            transform.translation.x -= 10.;
        }
    }
}
