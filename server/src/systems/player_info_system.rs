extern crate specs;
use std::collections::HashMap;

use crate::{Gold, Player, PlayerInfo, ResourcesStorage};
use specs::prelude::*;

pub struct PlayerInfoSystem {}

impl<'a> System<'a> for PlayerInfoSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Gold>,
        WriteStorage<'a, ResourcesStorage>,
        WriteStorage<'a, PlayerInfo>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut players, mut golds, mut resources_storages, mut player_infos) = data;

        let mut player_to_golds: HashMap<String, f32> = HashMap::new();
        for (_entity, player, gold) in (&entities, &mut players, &mut golds).join() {
            player_to_golds.insert(player.name.clone(), gold.quantity);
        }

        for (_entity, _player, gold, resources_storage, player_info) in (
            &entities,
            &mut players,
            &mut golds,
            &mut resources_storages,
            &mut player_infos,
        )
            .join()
        {
            player_info.gold = gold.quantity;
            player_info.resources = resources_storage.storage.clone();
            player_info.player_to_golds = player_to_golds.clone();
        }
    }
}
