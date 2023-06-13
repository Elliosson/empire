extern crate specs;
use crate::{Map, Player, TerritoryArea, TerritoryStatTiming};
use specs::prelude::*;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub struct TerritoryStatSystem {}

impl<'a> System<'a> for TerritoryStatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, TerritoryArea>,
        WriteExpect<'a, Map>,
        WriteExpect<'a, TerritoryStatTiming>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut players, mut territory_areas, map, mut territory_stat_timing) = data;

        if territory_stat_timing.last_time.elapsed() > Duration::from_secs(1) {
            territory_stat_timing.last_time = Instant::now();
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
}
