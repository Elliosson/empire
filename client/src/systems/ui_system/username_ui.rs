use crate::{PlayerInfo, ToSendWrap, UiState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn username_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut to_send: ResMut<ToSendWrap>,
    mut player_info: ResMut<PlayerInfo>,
) {
    egui::Window::new("Hello").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("world");

        ui.horizontal(|ui| {
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

        ui.add(egui::Slider::new(&mut ui_state.gold_percent, 0..=100).text("attack percentage"));
        if ui.button("Increment").clicked() {
            ui_state.gold_percent += 1;
        }
    });
}
