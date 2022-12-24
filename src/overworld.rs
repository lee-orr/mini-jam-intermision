use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{Color32, Frame},
    *,
};
use bevy_turborand::rng::Rng;

use crate::{
    assets, game_state::AppState, story::Story, tracery_generator::TraceryGenerator, ui::*,
};

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<OverworldUI>()
            .add_system_set(SystemSet::on_enter(AppState::Overworld).with_system(setup_overworld))
            .add_system_set(SystemSet::on_update(AppState::Overworld).with_system(display_overworld));
    }
}

#[derive(Default, Resource, Debug, Clone)]
struct OverworldUI {
    main_text: Option<String>,
}

fn setup_overworld(
    mut commands: Commands,
    assets: Res<assets::Assets>,
    stories: Res<Assets<TraceryGenerator>>,
) {
    let mut rng = Rng::new();
    let text = if let Some(asset) = stories.get(&assets.story) {
        let mut story = Story::generate(&mut rng, asset);
        let intro = story.introduce(&mut rng, asset);
        commands.insert_resource(story);
        Some(intro)
    } else {
        None
    };
    commands.insert_resource(OverworldUI { main_text: text });
}

fn display_overworld(mut egui_context: ResMut<EguiContext>, overworld: Res<OverworldUI>) {
    let ctx = egui_context.ctx_mut();
    egui::CentralPanel::default()
        .frame(Frame {
            fill: Color32::TRANSPARENT,
            ..Default::default()
        })
        .show(&ctx, |ui| {
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
                    if let Some(text) = &overworld.main_text {
                        ui.add(egui::Label::new(
                            egui::RichText::new(text).size(50.),
                        ));
                    }
                },
            );
        });
}
