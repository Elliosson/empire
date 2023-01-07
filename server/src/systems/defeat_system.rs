extern crate specs;
use crate::{GamePhase, GamePhaseEnum, Gold, Map, Player};
use specs::prelude::*;
use std::collections::HashSet;

pub struct DefeatSystem {}

impl<'a> System<'a> for DefeatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Gold>,
        WriteStorage<'a, GamePhase>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut players, mut golds, mut game_phases, map) = data;

        let mut player_with_territories = HashSet::new();

        for tile in map.tiles.iter() {
            player_with_territories.insert(tile.owner.clone());
        }

        for (_entity, player, gold, game_phase) in
            (&entities, &mut players, &mut golds, &mut game_phases).join()
        {
            if game_phase.phase == GamePhaseEnum::Playing {
                //if no gold anymore, then die
                if gold.quantity < 0. {
                    game_phase.phase = GamePhaseEnum::GameOver;
                }

                //if no territories, then die
                if !player_with_territories.contains(&player.name) {
                    game_phase.phase = GamePhaseEnum::GameOver;
                }
            }
        }
    }
}
