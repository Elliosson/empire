extern crate specs;
use crate::{BuildedTile, Map, ResourceExtractionBuilding, WantToBuild};
use specs::prelude::*;

pub struct BuildSystem {}

impl<'a> System<'a> for BuildSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantToBuild>,
        WriteStorage<'a, ResourceExtractionBuilding>,
        WriteStorage<'a, BuildedTile>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut want_to_builds, mut res_extract_buildings, mut builded_tiles, mut map) =
            data;

        for (_entity, want_to_build) in (&entities, &mut want_to_builds).join() {
            let tile_entity = map.get_tile_entity_or_create(&want_to_build.pos, &entities);
            if want_to_build.name == "ressoure_extractor" {
                //TODO add some sort of payment
                res_extract_buildings
                    .insert(tile_entity, ResourceExtractionBuilding {})
                    .unwrap();
                builded_tiles.insert(tile_entity, BuildedTile {}).unwrap();
            }
        }
    }
}
