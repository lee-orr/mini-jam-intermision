use bevy::prelude::*;

use crate::{assets, game_state::AppState, ui::*};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(display_menu))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(check_click))
            .add_system_set(clear_ui_system_set(AppState::MainMenu));
    }
}

fn display_menu(mut commands: Commands, assets: Res<assets::Assets>) {
    UiRoot::spawn(&mut commands, |parent| {
        parent.spawn(main_text("Intermission", 100.0, &assets));
        MenuButton::Primary.spawn("start", "Start", parent, &assets);
    });
}
fn check_click(mut app_state: ResMut<State<AppState>>, mut clicked: EventReader<ButtonClickEvent>) {
    for click in clicked.iter() {
        let ButtonClickEvent(val) = click;
        if val == "start" {
            let _ = app_state.set(AppState::Overworld);
        }
    }
}
