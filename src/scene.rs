use bevy::prelude::*;

use crate::{
    board::*, game_state::AppState, player_turn::PlayerTurnPlugin, scenario::*, setup_phase::*, ui::*, assets, story::{Scenario, ScenarioState},
};

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
            .add_system_set(SystemSet::on_update(SceneState::Succeeded).with_system(check_click))
            .add_system_set(SystemSet::on_update(SceneState::Failed).with_system(check_click))
            .add_system_set(SystemSet::on_enter(SceneState::Succeeded).with_system(display_success_menu))
            .add_system_set(SystemSet::on_enter(SceneState::Failed).with_system(display_failure_men));
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

fn display_success_menu(mut commands: Commands, assets: Res<assets::Assets>, scenario: Res<Scenario>) {
    UiRoot::spawn(&mut commands, |parent| {
        MainText::new("Success!")
            .size(100.)
            .spawn(parent, &assets);
            if let ScenarioState::Success(goal) = &scenario.state {
                MainText::new(goal).spawn(parent, &assets);
            }
        MenuButton::Primary.spawn("continue", "Continue", parent, &assets);
    });
}

fn display_failure_men(mut commands: Commands, assets: Res<assets::Assets>, scenario: Res<Scenario>) {
    UiRoot::spawn(&mut commands, |parent| {
        MainText::new("Mission Failed...")
            .size(100.)
            .spawn(parent, &assets);
        if let ScenarioState::Failure(goal) = &scenario.state {
            MainText::new(goal).spawn(parent, &assets);
        }
        MenuButton::Primary.spawn("continue", "Continue Story", parent, &assets);
    });
}

fn check_click(mut app_state: ResMut<State<AppState>>, mut clicked: EventReader<ButtonClickEvent>) {
    for click in clicked.iter() {
        let ButtonClickEvent(val, _) = click;
        if val == "continue" {
            let _ = app_state.set(AppState::Overworld);
        }
    }
}