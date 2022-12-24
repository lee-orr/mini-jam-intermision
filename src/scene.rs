use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, Frame},
    EguiContext,
};

use crate::{
    game_state::AppState,
    story::{Scenario, Story},
};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(SceneState::None)
            .add_system_set(SystemSet::on_update(AppState::Scene).with_system(display_scene))
            .add_system_set(SystemSet::on_enter(AppState::Scene).with_system(setup_scene))
            .add_system_set(SystemSet::on_exit(AppState::Scene).with_system(end_scene));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SceneState {
    None,
    Setup,
}

fn setup_scene(mut scene_state: ResMut<State<SceneState>>) {
    let _ = scene_state.set(SceneState::Setup);
}

fn end_scene(mut scene_state: ResMut<State<SceneState>>) {
    let _ = scene_state.set(SceneState::None);
}

fn display_scene(
    mut ctx: ResMut<EguiContext>,
    story: Option<ResMut<Story>>,
    mut app_state: ResMut<State<AppState>>,
) {
    let ctx = ctx.ctx_mut();
    if let Some(mut story) = story {
        if let Some(scenario) = story.scenarios.last_mut() {
            let scenario: &mut Scenario = scenario;
            egui::CentralPanel::default()
                .frame(Frame {
                    fill: Color32::TRANSPARENT,
                    ..Default::default()
                })
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        match &scenario.state {
                            crate::story::ScenarioState::InProgress(count) => {
                                let current_goal = scenario.goals.get(*count);
                                let previous_goal = if let Some(prev) = count.checked_sub(1) {
                                    scenario.goals.get(prev)
                                } else {
                                    None
                                };

                                if let Some(current_goal) = current_goal {
                                    if let Some(previous_goal) = previous_goal {
                                        ui.label(&previous_goal.success);
                                    }
                                    ui.label(&current_goal.description);
                                    ui.horizontal(|ui| {
                                        if ui.button("Succeed").clicked() {
                                            scenario.succeed();
                                        }
                                        if ui.button("Fail").clicked() {
                                            scenario.fail();
                                        }
                                    });
                                } else {
                                    ui.label("No goal?");
                                }
                            }
                            crate::story::ScenarioState::Success(text) => {
                                ui.label(text);
                                if ui.button("Continue...").clicked() {
                                    let _ = app_state.set(AppState::Overworld);
                                }
                            }
                            crate::story::ScenarioState::Failure(text) => {
                                ui.label(text);
                                if ui.button("Continue...").clicked() {
                                    let _ = app_state.set(AppState::Overworld);
                                }
                            }
                        };
                    });
                });
            return;
        }
    }
    egui::CentralPanel::default()
        .frame(Frame {
            fill: Color32::TRANSPARENT,
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.label("No story available");
        });
}
