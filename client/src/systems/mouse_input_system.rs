use crate::Point;
use crate::{MapClick, UiState};
use bevy::input::mouse::*;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn mouse_input_system(
    windows: Res<Windows>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut ui_state: ResMut<UiState>,
    mut map_click: ResMut<MapClick>,
    query_camera: Query<(&Camera, &Transform), With<Camera2d>>,
) {
    //get the camera pos
    //todo proprement avec id connue de la main camera
    let mut camera_pos_x = 0.;
    let mut camera_pos_y = 0.;

    for (_camera, transform) in query_camera.iter() {
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

                    if mouse_pos.x > 300. {
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

                        if !ui_state.attack_ui_open {
                            map_click.map_pos = Point {
                                x: x as f32,
                                y: y as f32,
                            };
                            map_click.screen_pos = Point {
                                x: mouse_pos.x as f32,
                                y: mouse_pos.y as f32,
                            };
                            ui_state.attack_ui_open = true;
                        } else {
                            ui_state.attack_ui_open = false;
                        }
                    }
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
