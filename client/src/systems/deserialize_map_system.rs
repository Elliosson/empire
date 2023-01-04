use crate::{Data, DataWrap, Map};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

pub fn deserialize_map_system(from_net_data: Res<DataWrap>, mut map: ResMut<Map>) {
    let data_guard = from_net_data.protected_data.lock().unwrap();

    match serde_json::from_str(&data_guard.map_string) {
        Ok(info) => {
            let temp: Map = info;
            *map = temp.clone();
        }
        Err(_) => println!("unable to deserialize json"),
    }
}
