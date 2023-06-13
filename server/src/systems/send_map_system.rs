extern crate specs;
use std::{
    cmp::{max, min},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{xy_idx, Connected, DezoomedMap, Map, Player, Position, WantMap, MAPHEIGHT, MAPWIDTH};
use common::{ClientMap, ClientTile, MapLevel, MapMessage};
use specs::prelude::*;

pub struct SendMapSystem {}

impl<'a> System<'a> for SendMapSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantMap>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Connected>,
        WriteExpect<'a, Map>,
        WriteExpect<'a, Arc<Mutex<HashMap<String, MapMessage>>>>,
        WriteExpect<'a, DezoomedMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut want_maps, players, connecteds, map, map_to_send, dezoomed_map) = data;

        let mut map_send_guard = map_to_send.lock().unwrap();

        for (_entity, _player, want_map, connected) in
            (&entities, &players, &mut want_maps, &connecteds).join()
        {
            let my_uiid = connected.uuid.clone();

            let map_message = MapMessage {
                map_json: serde_json::to_string(&format_map_for_client(
                    &map,
                    &dezoomed_map,
                    &want_map.pos,
                    want_map.scale,
                ))
                .unwrap(),
            };

            map_send_guard.insert(my_uiid, map_message);
        }

        want_maps.clear();
    }
}

pub fn format_map_for_client(
    map: &Map,
    dezoomed_map: &DezoomedMap,
    pos: &Position,
    scale: i32,
) -> ClientMap {
    let mut client_map: ClientMap = ClientMap::default();

    println!("scale {}", scale);

    if scale == 1 {
        //200*200 around pos.
        for x in max(pos.x - 100, 0)..min(pos.x + 100, MAPWIDTH) {
            for y in max(pos.y - 100, 0)..min(pos.y + 100, MAPHEIGHT) {
                let idx = xy_idx(x, y);
                let tile = &map.tiles[idx];
                client_map.tiles.insert(
                    (x, y),
                    ClientTile {
                        biome: tile.biome.clone(),
                        x,
                        y,
                        owner: tile.owner.clone(),
                        resource: tile.resource.clone(),
                    },
                );
            }
        }
    } else {
        let mut map_level: MapLevel = MapLevel::default();
        map_level.level = 2;
        for x in 0..(MAPWIDTH / 8) {
            for y in 0..(MAPHEIGHT / 8) {
                let color = dezoomed_map.hash_map.get(&(x, y, 2)).unwrap();
                map_level.map.insert((x * 8, y * 8), color.clone());
            }
        }
        client_map.dezoomed_map = Some(map_level);
    }

    return client_map;
}
