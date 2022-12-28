// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod card;
mod game_state;
mod menu;
mod overworld;
mod scene;
mod story;
mod ui;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_generative_grammars::tracery::tracery_asset::TraceryAssetPlugin;
#[cfg(feature = "dev")]
use bevy_inspector_egui::*;
use bevy_mod_picking::PickingCameraBundle;
use bevy_sequential_actions::SequentialActionsPlugin;
use bevy_turborand::RngPlugin;

use card::CardPlugin;
use game_state::AppState;
use menu::MenuPlugin;
use overworld::OverworldPlugin;

use scene::{board::board_assets::BoardAssets, ScenePlugin};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};
use ui::UIPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            fit_canvas_to_parent: true,
            ..Default::default()
        },
        ..Default::default()
    }))
    .insert_resource(ClearColor(Color::hex("25215e").unwrap_or_default()))
    .add_plugin(EguiPlugin)
    .add_plugin(LookTransformPlugin)
    .add_plugin(OrbitCameraPlugin::default())
    .add_plugin(RngPlugin::default())
    .add_plugin(SequentialActionsPlugin)
    .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
    .add_plugin(TraceryAssetPlugin::new().with_yaml(&["trace.yaml"]));

    #[cfg(feature = "dev")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_plugin(UIPlugin)
        .add_plugin(CardPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(OverworldPlugin)
        .add_plugin(ScenePlugin)
        .add_state(AppState::Loading)
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::MainMenu)
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                    "dynamic_assets.assets",
                ])
                .with_collection::<assets::Assets>()
                .with_collection::<BoardAssets>(),
        )
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let eye = Vec3::new(0., 15., 0.);
    let target = Vec3::default();
    commands
        .spawn((Camera3dBundle::default(), PickingCameraBundle::default()))
        .insert(OrbitCameraBundle::new(
            OrbitCameraController {
                enabled: true,
                mouse_translate_sensitivity: Vec2::new(2., 2.),
                ..Default::default()
            },
            eye,
            target,
        ));
    commands.spawn(Camera2dBundle {
        camera: Camera {
            priority: 1,
            ..Default::default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        ..Default::default()
    });
}
