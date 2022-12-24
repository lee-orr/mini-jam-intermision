use bevy::prelude::*;
use bevy_turborand::DelegatedRng;
use bevy_turborand::GlobalRng;
use smooth_bevy_cameras::LookTransform;

use crate::{scene::SceneState, story::Story};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(startup)
            .add_system_set(
                SystemSet::on_enter(SceneState::None)
                    .with_system(clear_board)
                    .with_system(reset_camera),
            )
            .add_system_set(
                SystemSet::on_enter(SceneState::Playing)
                    .with_system(generate_board)
                    .with_system(set_camera),
            );
    }
}

#[derive(Component)]
struct Board;

#[derive(Default, Resource)]
struct BoardAssets {
    tile: Handle<Mesh>,
    monster: Handle<Mesh>,
    tile_mat: Handle<StandardMaterial>,
    monster_mat: Handle<StandardMaterial>,
    goal_mat: Handle<StandardMaterial>,
    start_point_mat: Handle<StandardMaterial>,
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile = meshes.add(shape::Box::new(1., 0.2, 1.).into());
    let monster = meshes.add(shape::Box::new(0.3, 1.8, 0.2).into());
    let tile_mat = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..Default::default()
    });
    let goal_mat = materials.add(StandardMaterial {
        base_color: Color::GOLD,
        ..Default::default()
    });
    let start_point_mat = materials.add(StandardMaterial {
        base_color: Color::GREEN,
        ..Default::default()
    });
    let monster_mat = materials.add(StandardMaterial {
        base_color: Color::PURPLE,
        ..Default::default()
    });
    commands.insert_resource(BoardAssets {
        tile,
        tile_mat,
        goal_mat,
        start_point_mat,
        monster,
        monster_mat,
    });
}

fn clear_board(mut commands: Commands, query: Query<Entity, With<Board>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn generate_board(
    mut commands: Commands,
    _story: Option<ResMut<Story>>,
    assets: Res<BoardAssets>,
    mut global_rng: ResMut<GlobalRng>,
    mut scene_state: ResMut<State<SceneState>>,
) {
    let width = global_rng.usize(10..=20);
    let height = global_rng.usize(10..=20);

    let width_tiles = 0usize..width;

    let tiles: Vec<(usize, usize)> = width_tiles
        .into_iter()
        .flat_map(|w| {
            let height_tiles = 0usize..height;
            height_tiles
                .into_iter()
                .map(|h| (w, h))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let left = -1. * width as f32 / 2.;
    let top = -1. * height as f32 / 2.;

    let result = scene_state.set(SceneState::Playing);
    bevy::log::info!("Setting to playing: {:?}", result);

    commands
        .spawn((SpatialBundle::default(), Board))
        .with_children(|parent| {
            parent.spawn(DirectionalLightBundle {
                transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 2.8, 4.1, 0.)),
                ..Default::default()
            });

            let start = global_rng.usize(0..tiles.len());
            let end = {
                let mut end = global_rng.usize(0..tiles.len());
                while end == start {
                    end = global_rng.usize(0..tiles.len());
                }
                end
            };

            let monsters = 0..global_rng.usize(3..5);
            let monsters = monsters
                .map(|_m| global_rng.usize(0..tiles.len()))
                .collect::<Vec<usize>>();

            for (index, tile) in tiles.iter().enumerate() {
                let (material, is_occupied) = if index == start {
                    (assets.start_point_mat.clone(), true)
                } else if index == end {
                    (assets.goal_mat.clone(), true)
                } else if monsters.contains(&index) {
                    (assets.tile_mat.clone(), true)
                } else {
                    (assets.tile_mat.clone(), global_rng.chance(0.9))
                };
                if is_occupied {
                    parent.spawn(PbrBundle {
                        mesh: assets.tile.clone(),
                        material,
                        transform: Transform::from_xyz(
                            tile.0 as f32 + left,
                            -0.1,
                            tile.1 as f32 + top,
                        ),
                        ..Default::default()
                    });
                    if monsters.contains(&index) {
                        parent.spawn(PbrBundle {
                            mesh: assets.monster.clone(),
                            material: assets.monster_mat.clone(),
                            transform: Transform::from_xyz(
                                tile.0 as f32 + left,
                                1.,
                                tile.1 as f32 + top,
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
        });
}

fn set_camera(mut query: Query<&mut LookTransform>) {
    let eye = Vec3::new(0., 10., 0.);
    let target = Vec3::default();
    bevy::log::info!("Ready to play");
    for mut item in query.iter_mut() {
        bevy::log::info!("Setting camera pos");
        item.eye = eye;
        item.target = target;
    }
}

fn reset_camera(mut query: Query<&mut LookTransform>) {
    let eye = Vec3::new(0., 15., 0.);
    let target = Vec3::default();
    bevy::log::info!("Ready to play");
    for mut item in query.iter_mut() {
        bevy::log::info!("Setting camera pos");
        item.eye = eye;
        item.target = target;
    }
}
