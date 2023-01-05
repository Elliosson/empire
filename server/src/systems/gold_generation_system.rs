extern crate specs;
use crate::{Gold, Map, Player};
use specs::prelude::*;
use std::collections::HashMap;

pub struct GoldGenerationSystem {}

impl<'a> System<'a> for GoldGenerationSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Gold>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut players, mut golds, map) = data;

        let mut name_to_income: HashMap<String, f32> = HashMap::new();

        for tile in map.tiles.iter() {
            *name_to_income.entry(tile.owner.clone()).or_insert(0.) += 1.;
        }

        for (_entity, player, gold) in (&entities, &mut players, &mut golds).join() {
            gold.quantity += name_to_income.get(&player.name).unwrap_or_else(|| &0.);
            println!("player: {}, gold: {}", player.name, gold.quantity);
        }
    }
}
