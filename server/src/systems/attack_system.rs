extern crate specs;
use crate::{Gold, Map, OnGoingAttack, Player, WantToAttack};
use specs::prelude::*;

pub struct AttackSystem {}

impl<'a> System<'a> for AttackSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantToAttack>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Gold>,
        WriteStorage<'a, OnGoingAttack>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut want_to_attacks, mut players, mut golds, mut ongoing_attacks, map) =
            data;

        for (_entity, player, gold, want_to_attack) in
            (&entities, &mut players, &mut golds, &mut want_to_attacks).join()
        {
            println!("attack {:?}", want_to_attack.pos);
            entities
                .build_entity()
                .with(
                    OnGoingAttack {
                        gold: gold.quantity,
                        last_turn_conquest: vec![want_to_attack.pos.clone()],
                        owner: player.name.clone(),
                        enemy: None,
                    },
                    &mut ongoing_attacks,
                )
                .build();
            gold.quantity = 0.;
        }

        want_to_attacks.clear();
    }
}
