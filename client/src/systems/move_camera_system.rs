use bevy::{input::mouse::MouseWheel, prelude::*};

pub fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut transforms: Query<&mut Transform, With<Camera2d>>,
) {
    if keyboard_input.pressed(KeyCode::Up) {
        for mut transform in transforms.iter_mut() {
            transform.translation.y += 20.;
        }
    }
    if keyboard_input.pressed(KeyCode::Down) {
        for mut transform in transforms.iter_mut() {
            transform.translation.y -= 20.;
        }
    }
    if keyboard_input.pressed(KeyCode::Right) {
        for mut transform in transforms.iter_mut() {
            transform.translation.x += 20.;
        }
    }
    if keyboard_input.pressed(KeyCode::Left) {
        for mut transform in transforms.iter_mut() {
            transform.translation.x -= 20.;
        }
    }

    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!(
                    "Scroll (line units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
            MouseScrollUnit::Pixel => {
                println!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );

                for mut transform in transforms.iter_mut() {
                    let mut new_scale = transform.scale - ev.y * 0.005;
                    if new_scale.x < 1. {
                        new_scale = Vec3::new(1., 1., 1.);
                    }
                    if new_scale.x > 20. {
                        new_scale = Vec3::new(20., 20., 20.);
                    }
                    transform.scale = new_scale;
                }
            }
        }
    }
}
