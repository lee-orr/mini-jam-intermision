mod board;
mod player_turn;
mod scenario;
mod scenario_end_screens;
mod setup_phase;

use bevy::prelude::*;
use board::*;
use player_turn::PlayerTurnPlugin;
use scenario::*;
use setup_phase::*;

use crate::{game_state::AppState, ui::*};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(SceneState::None)
            .add_plugin(ScenarioPlugin)
            .add_plugin(BoardPlugin)
            .add_plugin(SetupPhasePlugin)
            .add_plugin(PlayerTurnPlugin)
            .add_system_set(SystemSet::on_enter(AppState::Scene).with_system(setup_scene))
            .add_system_set(SystemSet::on_exit(AppState::Scene).with_system(end_scene))
            .add_system_set(clear_ui_system_set(AppState::Scene))
            .add_system_set(
                SystemSet::on_update(SceneState::Succeeded)
                    .with_system(scenario_end_screens::check_click),
            )
            .add_system_set(
                SystemSet::on_update(SceneState::Failed)
                    .with_system(scenario_end_screens::check_click),
            )
            .add_system_set(
                SystemSet::on_enter(SceneState::Succeeded)
                    .with_system(scenario_end_screens::display_success_menu),
            )
            .add_system_set(
                SystemSet::on_enter(SceneState::Failed)
                    .with_system(scenario_end_screens::display_failure_men),
            );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SceneState {
    None,
    Setup,
    PlayerTurn,
    EnemyTurn,
    Processing,
    Succeeded,
    Failed,
}

fn setup_scene(mut scene_state: ResMut<State<SceneState>>) {
    let _ = scene_state.set(SceneState::Setup);
}

fn end_scene(mut scene_state: ResMut<State<SceneState>>) {
    let _ = scene_state.set(SceneState::None);
}
