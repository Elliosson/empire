extern crate specs;

use crate::{is_inside_map, City, Gold, Map, Player, WantNewCity};
use common::Building;
use specs::prelude::*;

pub struct NewCitySystem {}

impl<'a> System<'a> for NewCitySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantNewCity>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Gold>,
        WriteStorage<'a, City>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut want_new_cities, mut players, mut golds, mut cities, mut map) = data;

        for (_entity, player, gold, want_new_city) in
            (&entities, &mut players, &mut golds, &mut want_new_cities).join()
        {
            if is_inside_map(want_new_city.pos) && gold.quantity >= 100. {
                entities
                    .build_entity()
                    .with(
                        City {
                            name: "name".to_string(),
                            owner: player.name.clone(),
                        },
                        &mut cities,
                    )
                    .build();
                map.get_tile_mut(&want_new_city.pos).owner = player.name.clone();
                map.get_tile_mut(&want_new_city.pos).building = Some(Building::City);

                gold.quantity -= 100.;
            }
        }
        want_new_cities.clear();
    }
}
