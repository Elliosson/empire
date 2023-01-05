use crate::{DataWrap, ToSendWrap};
use bevy::input::mouse::*;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn mouse_input_system(
    windows: Res<Windows>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    to_send: ResMut<ToSendWrap>,
    net_data: ResMut<DataWrap>,
    query_camera: Query<(&Camera, &Transform), With<Camera2d>>,
) {
    let mut to_send_guard = to_send.to_send.lock().unwrap();
    let data_guard = net_data.protected_data.lock().unwrap();
    let uid = data_guard.my_uid.clone();

    //get the camera pos
    //todo proprement avec id connue de la main camera
    let mut camera_pos_x = 0.;
    let mut camera_pos_y = 0.;

    for (camera, transform) in query_camera.iter() {
        let translation = &transform.translation;

        camera_pos_x = translation.x;
        camera_pos_y = translation.y;
    }

    for event in mouse_button_input_events.iter() {
        if event.state == ButtonState::Released {
            match event.button {
                MouseButton::Left => {
                    let window = windows.get_primary().unwrap();
                    let mouse_pos = window.cursor_position().unwrap();

                    println!("event: {:?} position: {:?}", event, mouse_pos);

                    //pos is in pixel in the screen, need to be transform in equivalent in transform
                    //convert the click in tile pos

                    let coord = screen_coord_to_world_coord(
                        &windows,
                        camera_pos_x,
                        camera_pos_y,
                        mouse_pos.x,
                        mouse_pos.y,
                    );
                    let x = coord.0 as i32 / 10;
                    let y = coord.1 as i32 / 10;

                    to_send_guard.push(format!("{} {} {} {}", data_guard.my_uid, "attack", x, y))
                }
                _ => {}
            }
        }
    }
}

pub fn screen_coord_to_world_coord(
    windows: &Res<Windows>,
    cam_x: f32,
    cam_y: f32,
    screen_x: f32,
    screen_y: f32,
) -> (f32, f32) {
    let window = windows.get_primary().unwrap();
    let center_x = window.width() as f32 / 2.;
    let center_y = window.height() as f32 / 2.;

    let x = (screen_x - center_x) + cam_x;
    let y = (screen_y - center_y) + cam_y;
    println!("click to {} {}", x, y);

    return (x, y);
}
