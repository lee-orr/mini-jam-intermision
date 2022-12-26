use crate::assets;
use crate::game_state::AppState;
use crate::ui::*;
use bevy::prelude::*;

use crate::story::ScenarioState;

use crate::story::Scenario;

use super::SceneState;

pub struct ScenarioSuccessPlugin;

impl Plugin for ScenarioSuccessPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::Succeeded).with_system(display_success_menu),
        )
        .add_system_set(SystemSet::on_update(SceneState::Succeeded).with_system(check_click))
        .add_system_set(clear_ui_system_set(SceneState::Succeeded));
    }
}

pub(crate) fn display_success_menu(
    mut commands: Commands,
    assets: Res<assets::Assets>,
    scenario: Res<Scenario>,
) {
    UiRoot::spawn(&mut commands, |parent| {
        MainText::new("Success!").size(100.).spawn(parent, &assets);
        if let ScenarioState::Success(goal) = &scenario.state {
            MainText::new(goal).spawn(parent, &assets);
        }
        MenuButton::Primary.spawn("continue-to-overworld", "Continue", parent, &assets);
    });
}

pub(crate) fn check_click(
    mut app_state: ResMut<State<AppState>>,
    mut scene_state: ResMut<State<SceneState>>,
    mut clicked: EventReader<ButtonClickEvent>,
) {
    for click in clicked.iter() {
        let ButtonClickEvent(val, _) = click;
        if val == "continue-to-overworld" {
            let _ = scene_state.set(SceneState::None);
            let _ = app_state.set(AppState::Overworld);
        }
    }
}
