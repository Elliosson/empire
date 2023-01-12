extern crate specs;
use crate::{Map, Player, ResourcesStorage};
use common::Resources;
use specs::prelude::*;
use std::collections::HashMap;

pub struct ResourceGenerationSystem {}

impl<'a> System<'a> for ResourceGenerationSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, ResourcesStorage>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut players, mut resources_storages, map) = data;

        let mut name_to_incomes: HashMap<String, HashMap<Resources, f32>> = HashMap::new();

        for tile in map.tiles.iter() {
            if let Some(resource) = tile.resource.clone() {
                *name_to_incomes
                    .entry(tile.owner.clone())
                    .or_insert(HashMap::new())
                    .entry(resource)
                    .or_insert(0.) += 0.1;
            }
        }

        for (_entity, player, resource_storage) in
            (&entities, &mut players, &mut resources_storages).join()
        {
            if let Some(mut incomes) = name_to_incomes.remove(&player.name) {
                for (resource, income) in incomes.drain() {
                    *resource_storage.storage.entry(resource).or_insert(0.) += income;
                }
                println!("player: {}, resources: {:?}", player.name, resource_storage);
            }
        }
    }
}
