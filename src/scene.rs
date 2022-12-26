pub mod board;
mod intermission_phase;
mod player_turn;
mod scenario;
mod scenario_fail;
mod scenario_sucess;
mod setup_phase;

use bevy::prelude::*;
use board::*;
use intermission_phase::*;
use player_turn::PlayerTurnPlugin;
use scenario::*;
use setup_phase::*;

use crate::{game_state::AppState, ui::*};

use self::{scenario_fail::FailPhasePlugin, scenario_sucess::ScenarioSuccessPlugin};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(SceneState::None)
            .add_plugin(ScenarioPlugin)
            .add_plugin(BoardPlugin)
            .add_plugin(SetupPhasePlugin)
            .add_plugin(PlayerTurnPlugin)
            .add_plugin(IntermissionPhasePlugin)
            .add_plugin(ScenarioSuccessPlugin)
            .add_plugin(FailPhasePlugin)
            .add_system_set(SystemSet::on_enter(AppState::Scene).with_system(setup_scene))
            .add_system_set(SystemSet::on_exit(AppState::Scene).with_system(end_scene))
            .add_system_set(clear_ui_system_set(AppState::Scene));
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
    Intermission,
}

fn setup_scene(mut scene_state: ResMut<State<SceneState>>) {
    let _ = scene_state.set(SceneState::Setup);
}

fn end_scene(mut scene_state: ResMut<State<SceneState>>) {
    let _ = scene_state.set(SceneState::None);
}
