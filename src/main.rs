// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod game_state;
mod menu;
mod overworld;
mod scene;
mod story;
mod style;
mod tracery_generator;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::MaterialMesh2dBundle,
};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_egui::EguiPlugin;
use bevy_turborand::RngPlugin;
use game_state::AppState;
use menu::MenuPlugin;
use overworld::OverworldPlugin;
use scene::ScenePlugin;
use style::StylePlugin;
use tracery_generator::TraceryPlugin;

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
        .add_plugin(EguiPlugin)
        .add_plugin(StylePlugin)
        .add_plugin(RngPlugin::default())
        .add_plugin(TraceryPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(OverworldPlugin)
        .add_plugin(ScenePlugin)
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: colors.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
