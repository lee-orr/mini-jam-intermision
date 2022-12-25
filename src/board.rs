
mod selection_actions;
mod set_turn_process_action;
mod board_assets;
mod continue_action;
mod move_action;
mod wait_action;

use bevy::prelude::*;

use bevy_sequential_actions::{
    ActionsBundle, ActionsProxy, ModifyActions, 
};

use smooth_bevy_cameras::LookTransform;

use crate::game_state::AppState;
use crate::scenario::{
    Actor, ActorPosition, AnimateActionsEvents, CurrentTurnProcess, ScenarioMap, 
};
use crate::scene::SceneState;

use selection_actions::*;
use set_turn_process_action::*;
use board_assets::*;
use continue_action::*;

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
                SystemSet::on_update(SceneState::Setup)
                    .with_system(generate_board)
                    .with_system(set_camera),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Scene)
                    .with_system(animate_actions)
                    .with_system(wait_action::wait_system)
                    .with_system(continue_action::continue_system)
                    .with_system(setup_selectable)
                    .with_system(process_selection_events)
                    .with_system(set_selection)
                    .with_system(move_action::move_system)
                    .with_system(set_turn_process_system),
            );
    }
}

#[derive(Component)]
struct Board;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile = meshes.add(shape::Box::new(1., 0.2, 1.).into());
    let wall = meshes.add(shape::Box::new(1., 2., 1.).into());
    let obstacle = meshes.add(shape::Box::new(1., 0.6, 1.).into());
    let monster = meshes.add(shape::Box::new(0.3, 1.8, 0.2).into());
    let player = meshes.add(shape::Capsule::default().into());

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
    let player_mat = materials.add(StandardMaterial {
        base_color: Color::BLUE,
        ..Default::default()
    });
    let selector_mat = materials.add(StandardMaterial {
        base_color: Color::Rgba {
            red: 0.9,
            green: 0.8,
            blue: 0.2,
            alpha: 0.3,
        },
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    let selector_hover = materials.add(StandardMaterial {
        base_color: Color::Rgba {
            red: 0.7,
            green: 0.7,
            blue: 0.2,
            alpha: 0.5,
        },
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    let selector_active = materials.add(StandardMaterial {
        base_color: Color::Rgba {
            red: 0.2,
            green: 0.8,
            blue: 0.5,
            alpha: 0.5,
        },
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    commands.insert_resource(board_assets::BoardAssets {
        tile,
        selector: wall.clone(),
        wall,
        obstacle,
        tile_mat,
        goal_mat,
        start_point_mat,
        monster,
        player,
        monster_mat,
        player_mat,
        selector_mat,
        selector_active,
        selector_hover,
    });
}

fn clear_board(mut commands: Commands, query: Query<Entity, With<Board>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn generate_board(
    mut commands: Commands,
    assets: Res<board_assets::BoardAssets>,
    scenario_map: Option<Res<ScenarioMap>>,
) {
    if let Some(scenario_map) = scenario_map {
        if !scenario_map.is_changed() {
            return;
        }

        let left = -1. * scenario_map.width as f32 / 2.;
        let top = -1. * scenario_map.height as f32 / 2.;

        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(left, 0., top),
                    ..Default::default()
                },
                Board,
                ActionsBundle::new(),
            ))
            .with_children(|parent| {
                parent.spawn(DirectionalLightBundle {
                    transform: Transform::from_rotation(Quat::from_euler(
                        EulerRot::XYZ,
                        2.8,
                        4.1,
                        0.,
                    )),
                    ..Default::default()
                });

                for tile in scenario_map.tiles.iter() {
                    let pos = (tile.pos.0 as f32, tile.pos.1 as f32);

                    let floor_material = match tile.tag {
                        crate::scenario::TileTag::Start => assets.start_point_mat.clone(),
                        crate::scenario::TileTag::Target(_) => assets.goal_mat.clone(),
                        _ => assets.tile_mat.clone(),
                    };

                    match tile.tile_type {
                        crate::scenario::TileType::Empty => {}
                        crate::scenario::TileType::Floor => {
                            parent.spawn(PbrBundle {
                                mesh: assets.tile.clone(),
                                material: floor_material,
                                transform: Transform::from_xyz(pos.0, -0.1, pos.1),
                                ..Default::default()
                            });
                        }
                        crate::scenario::TileType::Obstacle => {
                            parent.spawn(PbrBundle {
                                mesh: assets.obstacle.clone(),
                                material: floor_material,
                                transform: Transform::from_xyz(pos.0, -0.1, pos.1),
                                ..Default::default()
                            });
                        }
                        crate::scenario::TileType::Wall => {
                            parent.spawn(PbrBundle {
                                mesh: assets.wall.clone(),
                                material: floor_material,
                                transform: Transform::from_xyz(pos.0, -0.1, pos.1),
                                ..Default::default()
                            });
                        }
                    }

                    match tile.tag {
                        crate::scenario::TileTag::Start => {
                            parent.spawn((
                                PbrBundle {
                                    mesh: assets.player.clone(),
                                    material: assets.player_mat.clone(),
                                    transform: Transform::from_xyz(pos.0, 0.5, pos.1),
                                    ..Default::default()
                                },
                                Actor::Player,
                                ActorPosition(tile.pos.0, tile.pos.1),
                            ));
                        }
                        crate::scenario::TileTag::Enemy(actor) => {
                            parent.spawn((
                                PbrBundle {
                                    mesh: assets.monster.clone(),
                                    material: assets.monster_mat.clone(),
                                    transform: Transform::from_xyz(pos.0, 0.5, pos.1),
                                    ..Default::default()
                                },
                                actor,
                                ActorPosition(tile.pos.0, tile.pos.1),
                            ));
                        }
                        _ => {}
                    }
                }
            });
    }
}

fn set_camera(mut query: Query<&mut LookTransform>, new_board: Query<Entity, Added<Board>>) {
    if new_board.is_empty() {
        return;
    }
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

fn animate_actions(
    mut commands: Commands,
    board: Query<Entity, With<Board>>,
    mut events: EventReader<AnimateActionsEvents>,
) {
    if let Ok(agent) = board.get_single() {
        let mut actions = commands.actions(agent);
        for event in events.iter() {
            match event {
                AnimateActionsEvents::Wait(s) => {
                    actions.add(wait_action::WaitAction {
                        duration: *s,
                        current: None,
                    });
                }
                AnimateActionsEvents::Continue(actor) => {
                    actions.add(continue_action::ContinueAction(*actor));
                }
                AnimateActionsEvents::SelectTargets(target_selection) => {
                    bevy::log::info!("Animate Selecting Targets");
                    actions.add(selection_actions::SelectTargetsAction(target_selection.clone()));
                }
                AnimateActionsEvents::Move(actor, position) => {
                    bevy::log::info!("Move To Target");
                    actions.add(move_action::MoveAction {
                        actor: *actor,
                        position: *position,
                        speed: 10.,
                    });
                }
                AnimateActionsEvents::SetTurnProcess(p) => {
                    actions.add(set_turn_process_action::SetTurnProcessAction(p.clone()));
                },
            }
        }
    }
}


