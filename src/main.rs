// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod game_state;
mod menu;
mod overworld;
mod story;
mod tracery_generator;
mod ui;

use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_turborand::RngPlugin;
use game_state::AppState;
use menu::MenuPlugin;
use overworld::OverworldPlugin;
use tracery_generator::TraceryPlugin;
use ui::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(RngPlugin::default())
        .add_plugin(StylePlugin)
        .add_plugin(TraceryPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(OverworldPlugin)
        .insert_resource(ClearColor(Color::hex("25215e").unwrap_or_default()))
        .add_state(AppState::Loading)
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::MainMenu)
                .with_collection::<assets::Assets>(),
        )
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}
