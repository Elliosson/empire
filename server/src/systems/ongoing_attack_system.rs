extern crate specs;
use crate::map::adjacent_positions;
use crate::{idx_xy, Gold, Map, MapMessage, OnGoingAttack, Player, Tile};
pub use common::ClientMap;
use common::ClientTile;
use specs::prelude::*;
use std::collections::HashMap;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

pub struct OngoingAttackSystem {}

impl<'a> System<'a> for OngoingAttackSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, OnGoingAttack>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Gold>,
        WriteExpect<'a, Map>,
        WriteExpect<'a, Arc<Mutex<MapMessage>>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut ongoing_attacks, players, mut golds, mut map, map_to_send) = data;

        let mut to_delete = HashSet::new();
        let mut defence_cost = HashMap::new();
        for (entity, ongoing_attack) in (&entities, &mut ongoing_attacks).join() {
            let mut new_conquest = Vec::new();
            for pos in ongoing_attack.last_turn_conquest.iter() {
                for target_pos in adjacent_positions(pos).iter() {
                    let tile: &mut Tile = map.get_tile_mut(target_pos);
                    if tile.owner == ongoing_attack.enemy.clone().unwrap() {
                        if tile.owner == "" {
                            if ongoing_attack.gold >= 1. {
                                tile.owner = ongoing_attack.owner.clone();
                                ongoing_attack.gold -= 1.;
                                new_conquest.push(target_pos.clone());
                            }
                        } else if tile.owner != ongoing_attack.owner {
                            if ongoing_attack.gold >= 2. {
                                ongoing_attack.gold -= 2.;
                                *defence_cost.entry(tile.owner.clone()).or_insert(0) += 1;
                                tile.owner = ongoing_attack.owner.clone();
                                new_conquest.push(target_pos.clone());
                            }
                        }
                    }
                }
            }
            ongoing_attack.last_turn_conquest = new_conquest;
            if ongoing_attack.last_turn_conquest.is_empty() {
                to_delete.insert(entity);
            }
        }

        //pay defence cost
        for (_entity, player, gold) in (&entities, &players, &mut golds).join() {
            gold.quantity -= *defence_cost.entry(player.name.clone()).or_default() as f32;
        }

        for entity in to_delete.drain() {
            entities.delete(entity).unwrap();
        }

        let mut map_to_send_guard = map_to_send.lock().unwrap();
        map_to_send_guard.map_json = serde_json::to_string(&format_map_for_client(&map)).unwrap();
    }
}

pub fn format_map_for_client(map: &Map) -> ClientMap {
    let mut client_map: ClientMap = ClientMap::default();

    for (i, tile) in map.tiles.iter().enumerate() {
        let (x, y) = idx_xy(i);
        client_map.tiles.push(ClientTile {
            biome: tile.biome.clone(),
            x,
            y,
            owner: tile.owner.clone(),
            resource: tile.resource.clone(),
        });
    }

    return client_map;
}
