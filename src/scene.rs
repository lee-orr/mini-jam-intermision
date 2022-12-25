use bevy::prelude::*;

use crate::{board::*, game_state::AppState, scenario::*, setup_phase::*};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(SceneState::None)
            .add_plugin(ScenarioPlugin)
            .add_plugin(BoardPlugin)
            .add_plugin(SetupPhasePlugin)
            .add_system_set(SystemSet::on_enter(AppState::Scene).with_system(setup_scene))
            .add_system_set(SystemSet::on_exit(AppState::Scene).with_system(end_scene));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SceneState {
    None,
    Setup,
    RoundStart,
}

fn setup_scene(mut scene_state: ResMut<State<SceneState>>) {
    let _ = scene_state.set(SceneState::Setup);
}

fn end_scene(mut scene_state: ResMut<State<SceneState>>) {
    let _ = scene_state.set(SceneState::None);
}
