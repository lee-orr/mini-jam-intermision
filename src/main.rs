// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod game_state;
mod ui;

use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use game_state::AppState;
use ui::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StylePlugin)
        .add_state(AppState::Loading)
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::MainMenu)
                .with_collection::<assets::Assets>(),
        )
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(display_menu))
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(check_click))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(display_in_game))
        .add_system_set(clear_ui_system_set(AppState::MainMenu))
        .add_system_set(clear_ui_system_set(AppState::InGame))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn display_menu(mut commands: Commands, assets: Res<assets::Assets>) {
    UiRoot::spawn(&mut commands, |parent| {
        parent.spawn(main_text("Intermission", 100.0, &assets));
        MenuButton::Primary.spawn("start", "Start", parent, &assets);
    });
}

fn display_in_game(mut commands: Commands, assets: Res<assets::Assets>) {
    UiRoot::spawn(&mut commands, |parent| {
        parent.spawn(main_text("In Game!", 100.0, &assets));
    });
}

fn check_click(mut app_state: ResMut<State<AppState>>, mut clicked: EventReader<ButtonClickEvent>) {
    for click in clicked.iter() {
        let ButtonClickEvent(val) = click;
        if val == "start" {
            let _ = app_state.set(AppState::InGame);
        }
    }
}
