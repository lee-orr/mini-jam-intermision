// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod game_state;
mod assets;

use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingStateAppExt, LoadingState};
use game_state::AppState;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Loading)
        .add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::MainMenu).with_collection::<assets::Assets>())
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<assets::Assets>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.icon.clone(),
        ..Default::default()
    });
}
