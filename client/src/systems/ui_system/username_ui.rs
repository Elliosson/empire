use crate::{ToSendWrap, UiState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use common::PlayerInfo;

pub fn username_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    to_send: ResMut<ToSendWrap>,
    player_info: ResMut<PlayerInfo>,
) {
    egui::SidePanel::left("Left Panel")
        .exact_width(300.)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("world");

            ui.vertical(|ui| {
                ui.label("Your username: ");
                ui.text_edit_singleline(&mut ui_state.username);
                if ui.button("Confirm").clicked() {
                    {
                        //TODO make proper register system
                        let mut to_send_guard = to_send.to_send.lock().unwrap();
                        to_send_guard.push(format!("register {}", ui_state.username));
                    }
                }
            });

            ui.label(format!("Gold: {} ", player_info.gold));

            ui.add(
                egui::Slider::new(&mut ui_state.gold_percent, 0..=100).text("attack percentage"),
            );
            if ui.button("Increment").clicked() {
                ui_state.gold_percent += 1;
            }
            ui.label(format!("Resouces: {:?} ", player_info.resources));
            if ui.button("New Game").clicked() {
                let mut to_send_guard = to_send.to_send.lock().unwrap();
                to_send_guard.push(format!("new_game"));
            }
        });
}
