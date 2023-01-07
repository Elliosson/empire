extern crate specs;
use crate::{Connected, Player, PlayerInfo};
use specs::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
//serialize PlayerInfo in a json
pub struct PlayerInfoJsonSystem {}

//create a json to be send on the client
impl<'a> System<'a> for PlayerInfoJsonSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, PlayerInfo>,
        ReadStorage<'a, Connected>,
        ReadStorage<'a, Player>,
        WriteExpect<'a, Arc<Mutex<HashMap<String, String>>>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player_infos, connecteds, players, send_player_info) = data;

        let mut player_info_guard = send_player_info.lock().unwrap();
        player_info_guard.clear();

        //todo check in player is connected and find a way to handle local player
        for (_entity, player_info, _player, connected) in
            (&entities, &player_infos, &players, &connecteds).join()
        {
            let player_info_string = serde_json::to_string(&player_info).unwrap();

            let my_uiid = connected.uuid.clone();

            player_info_guard.insert(my_uiid, player_info_string);
            //test by writing in a real file
            //player_json(player_info_string).unwrap();
        }
    }
}
