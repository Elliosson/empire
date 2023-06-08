extern crate specs;
use crate::{
    network, Connected, GamePhase, Gold, Player, Position, ResourcesStorage, TerritoryArea,
    WantMap, WantToAttack,
};
use common::PlayerInfo;

use specs::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct OnlinePlayerSystem {}

impl<'a> System<'a> for OnlinePlayerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, UuidPlayerHash>,
        WriteExpect<'a, NamePlayerHash>,
        WriteExpect<'a, Arc<Mutex<Vec<(network::Message, String)>>>>,
        WriteStorage<'a, Connected>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, WantToAttack>,
        WriteStorage<'a, Gold>,
        WriteStorage<'a, PlayerInfo>,
        WriteStorage<'a, GamePhase>,
        WriteStorage<'a, ResourcesStorage>,
        WriteStorage<'a, TerritoryArea>,
        WriteStorage<'a, WantMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut player_hash,
            mut pseudo_player_hash,
            message_mutex,
            mut connecteds,
            mut players,
            mut want_to_attacks,
            mut golds,
            mut player_infos,
            mut game_phases,
            mut resources_storages,
            mut territory_areas,
            mut want_maps,
        ) = data;

        let mut new_player_list = Vec::new();

        {
            //todo catch this error, can fail if invalide json, for now it's good because it's help debugging
            let mut message_list_guard = message_mutex.lock().unwrap();

            //todo hash map to get player entity

            for (net_mes, _command) in message_list_guard.iter() {
                // println!("message list: {:?}", net_mes);
                let mes = net_mes.clone();

                let uid;
                let player_entity: Option<&Entity>;
                match mes.clone() {
                    network::Message::Registered(uuid, name) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        match player_entity {
                            Some(_entity) => {
                                println!("ERROR: someone want to register with an already use uuid")
                            }
                            None => {
                                new_player_list.push((uid.clone(), name));
                            }
                        }
                    }
                    network::Message::Attack(uuid, x, y, percent) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        match player_entity {
                            Some(player_entity) => {
                                want_to_attacks
                                    .insert(
                                        *player_entity,
                                        WantToAttack {
                                            pos: Position::new(x, y),
                                            gold_percent: percent,
                                        },
                                    )
                                    .unwrap();
                            }
                            None => {
                                println!("Error: Trying to connect with an unknow uuid")
                            }
                        }
                    }
                    network::Message::Map(uuid, x, y, scale) => {
                        uid = uuid.to_string();
                        player_entity = player_hash.hash.get(&uid.clone());
                        match player_entity {
                            Some(player_entity) => {
                                want_maps
                                    .insert(
                                        *player_entity,
                                        WantMap {
                                            pos: Position { x, y },
                                            scale,
                                        },
                                    )
                                    .unwrap();
                            }
                            None => {
                                println!("Error: Trying to connect with an unknow uuid")
                            }
                        }
                    }

                    _ => {}
                }

                //todo read the hash map to asociate the uid with an entity
                //attention si c'est un register on va pas avoir l'uid en faite.
                //Donc traiter dans network les autre message et ne renvoyer que les message avec uid en premier
                //pour le register faire un truc, pour l'instant justen créer uine nouvelle entier q chaque uid inconue
            }

            message_list_guard.clear();
        }

        //create new player
        for (uid, pseudo) in new_player_list {
            //connect to this pseudo if he is already used
            let player_entity;
            if let Some(&entity) = pseudo_player_hash.hash.get(&pseudo) {
                //If he is still connected remove the old entity of uid_hash
                if let Some(connected) = connecteds.get(entity) {
                    player_hash.hash.remove(&connected.uuid);
                }

                player_entity = entity
            } else {
                let new_player = entities
                    .build_entity()
                    .with(
                        Player {
                            name: pseudo.clone(),
                        },
                        &mut players,
                    )
                    .with(Gold { quantity: 100. }, &mut golds)
                    .with(PlayerInfo::default(), &mut player_infos)
                    .with(GamePhase::default(), &mut game_phases)
                    .with(ResourcesStorage::default(), &mut resources_storages)
                    .with(TerritoryArea::default(), &mut territory_areas)
                    .build();

                player_entity = new_player;
            }
            connecteds
                .insert(player_entity, Connected { uuid: uid.clone() })
                .expect("Unable to insert");
            pseudo_player_hash
                .hash
                .insert(pseudo.clone(), player_entity);

            player_hash.hash.insert(uid.clone(), player_entity);
        }
    }
}

//link the uiid with the correct player entity
pub struct UuidPlayerHash {
    pub hash: HashMap<String, Entity>,
}

impl UuidPlayerHash {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UuidPlayerHash {
        UuidPlayerHash {
            hash: HashMap::new(),
        }
    }
}

//link the uiid with the correct player entity
pub struct NamePlayerHash {
    pub hash: HashMap<String, Entity>,
}

impl NamePlayerHash {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        NamePlayerHash {
            hash: HashMap::new(),
        }
    }
}
