use crate::{DataWrap, MapClick, ToSendWrap, UiState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContext,
};

pub fn attack_ui(
    mut egui_ctx: ResMut<EguiContext>,
    ui_state: ResMut<UiState>,
    to_send: ResMut<ToSendWrap>,
    map_click: ResMut<MapClick>,
    net_data: ResMut<DataWrap>,
    windows: Res<Windows>,
) {
    //check ui state to know if the window is activated.
    //set fixed size for the window
    //set the position to the click position.

    let data_guard = net_data.protected_data.lock().unwrap();

    let window = windows.get_primary().unwrap();

    if ui_state.attack_ui_open {
        egui::Area::new("attack area")
            .current_pos([
                map_click.screen_pos.x,
                window.height() - map_click.screen_pos.y,
            ])
            .show(egui_ctx.ctx_mut(), |ui| {
                if ui.button("Attack").clicked() {
                    {
                        let mut to_send_guard = to_send.to_send.lock().unwrap();
                        to_send_guard.push(format!(
                            "{} {} {} {} {}",
                            data_guard.my_uid,
                            "attack",
                            map_click.map_pos.x,
                            map_click.map_pos.y,
                            ui_state.gold_percent
                        ))
                    }
                }
            });
    }
}
