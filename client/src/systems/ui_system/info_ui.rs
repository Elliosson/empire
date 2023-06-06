use crate::RightClickedTile;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use common::{ClientMap, PlayerInfo};

pub fn info_ui(
    mut egui_ctx: ResMut<EguiContext>,
    player_info: ResMut<PlayerInfo>,
    right_click: ResMut<RightClickedTile>,
    map: Res<ClientMap>,
) {
    egui::Window::new("Info").show(egui_ctx.ctx_mut(), |ui| {
        let mut owner = "".to_string();
        for tile in map.tiles.values() {
            if tile.x == right_click.pos.x as i32 && tile.y == right_click.pos.y as i32 {
                owner = tile.owner.clone();
                break;
            }
        }

        let gold = *player_info
            .player_to_golds
            .get(&owner)
            .unwrap_or_else(|| &0.);

        ui.label(format!(
            "tile: {} {}, owner: {}, owner's gold: {}",
            right_click.pos.x, right_click.pos.y, owner, gold
        ));
    });
}
