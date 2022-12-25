use crate::assets;
use crate::game_state::AppState;
use crate::ui::*;
use bevy::prelude::*;

use crate::story::ScenarioState;

use crate::story::Scenario;

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
        MenuButton::Primary.spawn("continue", "Continue", parent, &assets);
    });
}

pub(crate) fn display_failure_men(
    mut commands: Commands,
    assets: Res<assets::Assets>,
    scenario: Res<Scenario>,
) {
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

pub(crate) fn check_click(
    mut app_state: ResMut<State<AppState>>,
    mut clicked: EventReader<ButtonClickEvent>,
) {
    for click in clicked.iter() {
        let ButtonClickEvent(val, _) = click;
        if val == "continue" {
            let _ = app_state.set(AppState::Overworld);
        }
    }
}
