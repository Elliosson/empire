use crate::{DataWrap, LastRequestInfoTime, ToSendWrap};
use bevy::prelude::*;
use instant::Instant;
use std::time::Duration;

pub fn request_info_system(
    to_send: ResMut<ToSendWrap>,
    mut last_time: ResMut<LastRequestInfoTime>,
    net_data: ResMut<DataWrap>,
    cameras: Query<&Transform, With<Camera2d>>,
) {
    let data_guard = net_data.protected_data.lock().unwrap();

    let ask_info_interval = Duration::from_millis(100);

    let mut camera_pos_x = 0.;
    let mut camera_pos_y = 0.;
    let mut camera_scale = 1.;
    for camera in cameras.iter() {
        let translation = &camera.translation;

        camera_pos_x = translation.x;
        camera_pos_y = translation.y;
        camera_scale = camera.scale.y;
    }

    println!("camera scale {}", camera_scale);

    if last_time.time.elapsed() > ask_info_interval {
        let mut to_send_guard = to_send.to_send.lock().unwrap();
        to_send_guard.push(format!(
            "{} {} {} {} {}",
            data_guard.my_uid,
            "map",
            camera_pos_x as i32 / 10,
            camera_pos_y as i32 / 10,
            camera_scale as i32
        ));
        to_send_guard.push(format!("{} {} ", data_guard.my_uid, "player_info",));

        last_time.time = Instant::now();
    }
}
