extern crate specs;
use crate::{Gold, Player, PlayerInfo};
use specs::prelude::*;

pub struct PlayerInfoSystem {}

impl<'a> System<'a> for PlayerInfoSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Gold>,
        WriteStorage<'a, PlayerInfo>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut players, mut golds, mut player_infos) = data;

        for (_entity, _player, gold, player_info) in
            (&entities, &mut players, &mut golds, &mut player_infos).join()
        {
            player_info.gold = gold.quantity;
        }
    }
}
