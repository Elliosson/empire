use crate::DataWrap;
use bevy::prelude::*;
use common::ClientMap;

pub fn deserialize_map_system(from_net_data: Res<DataWrap>, mut map: ResMut<ClientMap>) {
    let data_guard = from_net_data.protected_data.lock().unwrap();

    match serde_json::from_str(&data_guard.map_string) {
        Ok(info) => {
            let temp: ClientMap = info;
            *map = temp.clone();
        }
        Err(_) => println!(
            "unable to deserialize  map json : {}",
            data_guard.map_string
        ),
    }
}
