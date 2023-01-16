use crate::{ToSendWrap, UiState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use common::PlayerInfo;

pub fn building_ui(
    mut egui_ctx: ResMut<EguiContext>,
    _ui_state: ResMut<UiState>,
    to_send: ResMut<ToSendWrap>,
    player_info: ResMut<PlayerInfo>,
) {
    egui::Window::new("Buildings").show(egui_ctx.ctx_mut(), |ui| {
        for building_info in player_info.buildings.iter() {
            ui.label(format!(
                "pos: {} {}, building: {}",
                building_info.x, building_info.y, building_info.name
            ));
        }

        if ui.button("Build converter").clicked() {
            {
                //TODO build on a specific position
                let mut to_send_guard = to_send.to_send.lock().unwrap();
                to_send_guard.push(format!("build extract_res_building 0 0"));
            }
        }
    });
}
