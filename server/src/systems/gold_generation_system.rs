extern crate specs;
use crate::{Gold, Map, Player, TerritoryArea};
use specs::prelude::*;
use std::collections::HashMap;

pub struct GoldGenerationSystem {}

impl<'a> System<'a> for GoldGenerationSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Gold>,
        ReadStorage<'a, TerritoryArea>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut players, mut golds, territory_areas, map) = data;

        let mut name_to_income: HashMap<String, f32> = HashMap::new();

        for tile in map.tiles.iter() {
            *name_to_income.entry(tile.owner.clone()).or_insert(0.) += 0.001;
        }

        for (_entity, player, gold, area) in
            (&entities, &mut players, &mut golds, &territory_areas).join()
        {
            gold.quantity += name_to_income.get(&player.name).unwrap_or_else(|| &0.);
            let max_gold = (area.area * 10 + 100) as f32;
            if gold.quantity > max_gold {
                gold.quantity = max_gold;
            }
            // println!("player: {}, gold: {}", player.name, gold.quantity);
        }
    }
}
