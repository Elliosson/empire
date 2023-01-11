use crate::helper::screen_coord_to_world_coord;
use crate::{MapClick, UiState};
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn cursor_position_system(
    windows: Res<Windows>,
    mut cursor_evr: EventReader<CursorMoved>,
    mut ui_state: ResMut<UiState>,
    map_click: ResMut<MapClick>,
    query_camera: Query<(&Camera, &Transform), With<Camera2d>>,
) {
    let mut camera_pos_x = 0.;
    let mut camera_pos_y = 0.;

    for (_camera, transform) in query_camera.iter() {
        let translation = &transform.translation;

        camera_pos_x = translation.x;
        camera_pos_y = translation.y;
    }

    for ev in cursor_evr.iter() {
        let (x, y) = screen_coord_to_world_coord(
            &windows,
            ev.position.x,
            ev.position.y,
            camera_pos_x,
            camera_pos_y,
        );

        if ui_state.attack_ui_open {
            if x > map_click.bevy_wolrd_pos.x + 50.
                || x < map_click.bevy_wolrd_pos.x - 50.
                || y > map_click.bevy_wolrd_pos.y + 50.
                || y < map_click.bevy_wolrd_pos.y - 50.
            {
                ui_state.attack_ui_open = false;
            }
        }
    }
}
