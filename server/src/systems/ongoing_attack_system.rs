extern crate specs;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use crate::{idx_xy, Map, MapForClient, MapMessage, OnGoingAttack, Position, Tile, TileForClient};
use specs::prelude::*;

pub struct OngoingAttackSystem {}

impl<'a> System<'a> for OngoingAttackSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, OnGoingAttack>,
        WriteExpect<'a, Map>,
        WriteExpect<'a, Arc<Mutex<MapMessage>>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut ongoing_attacks, mut map, mut map_to_send) = data;

        let mut to_delete = HashSet::new();
        for (entity, ongoing_attack) in (&entities, &mut ongoing_attacks).join() {
            let mut new_conquest = Vec::new();
            for pos in ongoing_attack.last_turn_conquest.iter() {
                for target_pos in adjacent_positions(pos).iter() {
                    let tile: &mut Tile = map.get_tile_mut(target_pos);
                    if tile.owner != ongoing_attack.owner {
                        if ongoing_attack.gold >= 1. {
                            tile.owner = ongoing_attack.owner.clone();
                            ongoing_attack.gold -= 1.;
                            new_conquest.push(target_pos.clone());
                        } else {
                            to_delete.insert(entity);
                        }
                    }
                }
            }
            ongoing_attack.last_turn_conquest = new_conquest;
        }

        for entity in to_delete.drain() {
            entities.delete(entity).unwrap();
        }

        let mut map_to_send_guard = map_to_send.lock().unwrap();
        map_to_send_guard.map = map.clone();
        map_to_send_guard.map_json =
            serde_json::to_string(&format_map_for_client(&map_to_send_guard.map)).unwrap();
    }
}

pub fn adjacent_positions(pos: &Position) -> Vec<Position> {
    let x = pos.x;
    let y = pos.y;
    return vec![
        Position::new(x, y - 1),
        Position::new(x, y + 1),
        Position::new(x - 1, y),
        Position::new(x + 1, y),
    ];
}

pub fn format_map_for_client(map: &Map) -> MapForClient {
    let mut client_map: MapForClient = MapForClient::default();

    for (i, tile) in map.tiles.iter().enumerate() {
        let (x, y) = idx_xy(i);
        client_map.tiles.push(TileForClient {
            biome: tile.biome.clone(),
            x,
            y,
            owner: tile.owner.clone(),
        });
    }

    return client_map;
}
