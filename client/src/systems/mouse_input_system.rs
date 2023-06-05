use crate::helper::screen_coord_to_world_coord;
use crate::{MapClick, UiState};
use crate::{Point, RightClickedTile};
use bevy::input::mouse::*;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn mouse_input_system(
    windows: Res<Windows>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut ui_state: ResMut<UiState>,
    mut map_click: ResMut<MapClick>,
    mut right_click: ResMut<RightClickedTile>,
    query_camera: Query<(&Camera, &Transform), With<Camera2d>>,
) {
    //get the camera pos
    //todo proprement avec id connue de la main camera
    let mut camera_pos_x = 0.;
    let mut camera_pos_y = 0.;
    let mut camera_scale = 1.;

    for (_camera, transform) in query_camera.iter() {
        let translation = &transform.translation;

        camera_pos_x = translation.x;
        camera_pos_y = translation.y;
        camera_scale = transform.scale.x;
    }

    for event in mouse_button_input_events.iter() {
        if event.state == ButtonState::Released {
            match event.button {
                MouseButton::Left => {
                    if !ui_state.attack_ui_open {
                        *map_click =
                            get_map_click(&windows, camera_pos_x, camera_pos_y, camera_scale);
                        ui_state.attack_ui_open = true;
                    }
                }

                MouseButton::Right => {
                    ui_state.attack_ui_open = false;
                    right_click.pos =
                        get_map_click(&windows, camera_pos_x, camera_pos_y, camera_scale).map_pos;
                }
                _ => {}
            }
        }
    }
}

fn get_map_click(
    windows: &Res<Windows>,
    camera_pos_x: f32,
    camera_pos_y: f32,
    camera_scale: f32,
) -> MapClick {
    let mut map_click = MapClick::default();

    let window = windows.get_primary().unwrap();
    let mouse_pos = window.cursor_position().unwrap();

    //ignore click in the side panel area.
    if mouse_pos.x > 300. {
        //pos is in pixel in the screen, need to be transform in equivalent in transform
        //convert the click in tile pos

        let (world_x, world_y) = screen_coord_to_world_coord(
            windows,
            camera_pos_x,
            camera_pos_y,
            mouse_pos.x,
            mouse_pos.y,
            camera_scale,
        );
        let x = world_x as i32 / 10;
        let y = world_y as i32 / 10;

        map_click.map_pos = Point {
            x: x as f32,
            y: y as f32,
        };
        map_click.screen_pos = Point {
            x: mouse_pos.x as f32,
            y: mouse_pos.y as f32,
        };
        map_click.bevy_wolrd_pos = Point {
            x: world_x,
            y: world_y,
        };
    }

    return map_click;
}
