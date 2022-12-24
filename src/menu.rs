use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, Frame},
    EguiContext,
};

use crate::game_state::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(display_menu));
    }
}

fn display_menu(mut egui_context: ResMut<EguiContext>, mut app_state: ResMut<State<AppState>>) {
    let ctx = egui_context.ctx_mut();
    egui::CentralPanel::default()
        .frame(Frame {
            fill: Color32::TRANSPARENT,
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.with_layout(
                egui::Layout {
                    main_dir: egui::Direction::TopDown,
                    main_wrap: false,
                    main_align: egui::Align::Center,
                    main_justify: false,
                    cross_align: egui::Align::Center,
                    cross_justify: false,
                },
                |ui| {
                    ui.add(egui::Label::new(
                        egui::RichText::new("Intermission").size(100.),
                    ));
                    if ui.button("Start").clicked() {
                        let _ = app_state.set(AppState::Overworld);
                    }
                },
            );
        });
}
