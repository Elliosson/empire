pub use bevy::prelude::*;

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
