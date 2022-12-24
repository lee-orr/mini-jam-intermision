use bevy::prelude::*;
use bevy_egui::{
    egui::{Color32, Frame, RichText},
    *,
};
use bevy_turborand::RngComponent;

use crate::{assets, game_state::AppState, story::{Story, ScenarioState, Goal}, tracery_generator::TraceryGenerator};

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
) {
    let mut rng = RngComponent::new();
    if let Some(asset) = stories.get(&assets.story) {
        let mut story = Story::generate(&mut rng, asset);
        story.introduce();
        commands.insert_resource(story);
    }
}

fn display_overworld(mut egui_context: ResMut<EguiContext>, mut story: Option<ResMut<Story>>) {
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
                    if let Some(mut story) = story {
                        if let Some(scenario) = story.get_current_scenario() {
                            ui.label(RichText::from(&scenario.initial_description).size(30.));
                            if let Some(goal) = scenario.goals.iter().next() {
                                ui.label(match &goal {
                                    Goal::ReachLocation(t, _, _) => t,
                                    _ => "What?",
                                });
                            } else {
                                ui.label("No goals?");
                            }

                            match scenario.state {
                                ScenarioState::InProgress => {
                                    if ui.button("Succeed").clicked() {

                                    } else if ui.button("Fail").clicked() {

                                    }
                                },
                                _ => {}
                            };
                        } else {
                            ui.label("Couldn't load scenario");
                        }
                    }
                },
            );
        });
}
