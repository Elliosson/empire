extern crate specs;
use crate::{Map, Player, TerritoryArea};
use specs::prelude::*;
use std::collections::HashMap;

pub struct TerritoryStatSystem {}

impl<'a> System<'a> for TerritoryStatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, TerritoryArea>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut players, mut territory_areas, map) = data;

        let mut name_to_area: HashMap<String, i32> = HashMap::new();

        for tile in map.tiles.iter() {
            *name_to_area.entry(tile.owner.clone()).or_insert(0) += 1;
        }

        for (_entity, player, territory_area) in
            (&entities, &mut players, &mut territory_areas).join()
        {
            territory_area.area = *name_to_area.get(&player.name).unwrap_or_else(|| &0);
        }
    }
}
