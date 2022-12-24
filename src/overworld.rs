use bevy::prelude::*;
use bevy_egui::{
    egui::{Color32, Frame, RichText},
    *,
};
use bevy_turborand::RngComponent;

use crate::{
    assets,
    game_state::AppState,
    story::{Story, StoryPhase},
    tracery_generator::TraceryGenerator,
};

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::Overworld).with_system(setup_overworld))
            .add_system_set(
                SystemSet::on_update(AppState::Overworld).with_system(display_overworld),
            );
    }
}

fn setup_overworld(
    mut commands: Commands,
    assets: Res<assets::Assets>,
    stories: Res<Assets<TraceryGenerator>>,
    story: Option<ResMut<Story>>,
) {
    if let Some(mut story) = story {
        story.generate_next_scenario();
    } else {
        let mut rng = RngComponent::new();
        if let Some(asset) = stories.get(&assets.story) {
            let mut story = Story::generate(&mut rng, asset);
            story.generate_next_scenario();
            commands.insert_resource(story);
        }
    }
}

fn display_overworld(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    story: Option<ResMut<Story>>,
    mut app_state: ResMut<State<AppState>>,
) {
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
                    if let Some(story) = story {
                        if StoryPhase::Complete == story.phase {
                            if ui.button("The End").clicked() {
                                let _ = app_state.set(AppState::MainMenu);
                                commands.remove_resource::<Story>();
                            }
                        } else if let Some(scenario) = story.get_current_scenario() {
                            ui.label(RichText::from(&scenario.initial_description).size(30.));
                            if ui.button("Start Scenario").clicked() {
                                let _ = app_state.set(AppState::Scene);
                            }
                        } else {
                            ui.label("Couldn't load scenario");
                        }
                    }
                },
            );
        });
}
