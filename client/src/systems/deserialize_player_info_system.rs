use crate::{DataWrap, PlayerInfo};
use bevy::prelude::*;

pub fn deserialise_player_info_system(
    from_net_data: Res<DataWrap>,
    mut player_info: ResMut<PlayerInfo>,
) {
    let data_guard = from_net_data.protected_data.lock().unwrap();
    // println!("{}", data_guard.info_string);

    match serde_json::from_str(&data_guard.info_string) {
        Ok(info) => {
            let temp: PlayerInfo = info;
            *player_info = temp.clone();
        }
        Err(_) => println!("unable to deserialize json"),
    }
}
