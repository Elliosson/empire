use bevy::prelude::*;

pub fn move_camera_system(
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
