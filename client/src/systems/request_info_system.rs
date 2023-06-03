use crate::{DataWrap, LastRequestInfoTime, ToSendWrap};
use bevy::prelude::*;
use instant::Instant;
use std::time::Duration;

pub fn request_info_system(
    to_send: ResMut<ToSendWrap>,
    mut last_time: ResMut<LastRequestInfoTime>,
    net_data: ResMut<DataWrap>,
) {
    let data_guard = net_data.protected_data.lock().unwrap();

    let ask_info_interval = Duration::from_millis(100);

    if last_time.time.elapsed() > ask_info_interval {
        let mut to_send_guard = to_send.to_send.lock().unwrap();
        to_send_guard.push(format!("{} {} ", data_guard.my_uid, "map",));
        to_send_guard.push(format!("{} {} ", data_guard.my_uid, "player_info",));

        last_time.time = Instant::now();
    }
}
