mod attack_action;
pub mod board_assets;
mod continue_action;
mod move_action;
mod selection_actions;
mod set_turn_process_action;
mod stun_action;
mod wait_action;

use bevy::prelude::*;

use bevy_sequential_actions::{ActionsBundle, ActionsProxy, ModifyActions};

use smooth_bevy_cameras::LookTransform;

use super::scenario::{
    scenario_map::{self, *},
    types::{ActorResources, AdjustActorEvent},
    Actor, ActorPosition, AnimateActionsEvents, Goal, GoalStatus,
};
use crate::game_state::AppState;
use crate::scene::SceneState;

use selection_actions::*;
use set_turn_process_action::*;

use self::board_assets::BoardAssets;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::None)
                .with_system(clear_board)
                .with_system(reset_camera),
        )
        .add_system_set(
            SystemSet::on_enter(SceneState::Succeeded)
                .with_system(clear_board)
                .with_system(reset_camera),
        )
        .add_system_set(
            SystemSet::on_enter(SceneState::Failed)
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
                .with_system(attack_action::attack_system)
                .with_system(stun_action::stun_system)
                .with_system(set_turn_process_system)
                .with_system(draw_active_goal)
                .with_system(apply_changes_to_actors)
                .with_system(react_to_actor_events),
        );
    }
}

#[derive(Component)]
struct Board;

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

                    let goal_id = match tile.tag {
                        scenario_map::TileTag::Target(id) => Some(id),
                        _ => None,
                    };

                    match tile.tile_type {
                        scenario_map::TileType::Empty => {}
                        scenario_map::TileType::Floor => {
                            let mut tile = parent.spawn(PbrBundle {
                                mesh: assets.tile.clone(),
                                material: assets.tile_mat.clone(),
                                transform: Transform::from_xyz(pos.0, 0., pos.1),
                                ..Default::default()
                            });
                            if let Some(goal_id) = goal_id {
                                tile.insert(Goal {
                                    number: goal_id,
                                    status: if goal_id == 0 {
                                        GoalStatus::Active
                                    } else {
                                        GoalStatus::Hidden
                                    },
                                });
                            }
                        }
                        TileType::Obstacle => {
                            parent.spawn(PbrBundle {
                                mesh: assets.obstacle.clone(),
                                material: assets.obstacle_mat.clone(),
                                transform: Transform::from_xyz(pos.0, 0., pos.1),
                                ..Default::default()
                            });
                        }
                        TileType::Wall => {
                            parent.spawn(PbrBundle {
                                mesh: assets.wall.clone(),
                                material: assets.wall_mat.clone(),
                                transform: Transform::from_xyz(pos.0, 0., pos.1),
                                ..Default::default()
                            });
                        }
                    }

                    match tile.tag {
                        TileTag::Start => {
                            parent.spawn((
                                PbrBundle {
                                    mesh: assets.player.clone(),
                                    material: assets.player_mat.clone(),
                                    transform: Transform::from_xyz(pos.0, 0., pos.1),
                                    ..Default::default()
                                },
                                Actor::Player,
                                ActorPosition(tile.pos.0, tile.pos.1),
                            ));
                        }
                        scenario_map::TileTag::Enemy(actor) => {
                            parent.spawn((
                                PbrBundle {
                                    mesh: assets.monster.clone(),
                                    material: assets.monster_mat.clone(),
                                    transform: Transform::from_xyz(pos.0, 0., pos.1),
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
    bevy::log::info!("Preparing Camera for Game");
    for mut item in query.iter_mut() {
        bevy::log::info!("Setting camera pos");
        item.eye = eye;
        item.target = target;
    }
}

fn reset_camera(mut query: Query<&mut LookTransform>) {
    let eye = Vec3::new(0., 15., 0.);
    let target = Vec3::default();
    bevy::log::info!("Moving camera back");
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
                    actions.add(selection_actions::SelectTargetsAction(
                        target_selection.clone(),
                    ));
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
                }
                AnimateActionsEvents::Attack(actor, target, damage) => {
                    actions.add(attack_action::AttackAction {
                        actor: *actor,
                        target: *target,
                        damage: *damage,
                        duration: 0.5,
                    });
                }
                AnimateActionsEvents::Stun(actor, target, stun_duration) => {
                    actions.add(stun_action::StunAction {
                        actor: *actor,
                        target: *target,
                        stun_duration: *stun_duration,
                        duration: 0.5,
                    });
                }
            }
        }
    }
}

fn draw_active_goal(
    mut commands: Commands,
    goals: Query<(Entity, &Goal), Changed<Goal>>,
    assets: Res<BoardAssets>,
) {
    for (entity, goal) in goals.iter() {
        let material = match goal.status {
            GoalStatus::Hidden => &assets.tile_mat,
            GoalStatus::Active => &assets.goal_mat,
            GoalStatus::Completed => &assets.goal_succeeded_mat,
        }
        .clone();
        commands.entity(entity).insert(material);
    }
}

fn apply_changes_to_actors(
    mut commands: Commands,
    mut query: Query<(&Actor, Entity, &mut Transform)>,
    resources: Res<ActorResources>,
    assets: Res<BoardAssets>,
) {
    if !resources.is_changed() {
        return;
    }
    for (actor, entity, mut transform) in query.iter_mut() {
        if !resources.turn_order.contains(actor) {
            commands
                .entity(entity)
                .insert(assets.dead_character_mat.clone())
                .remove::<Actor>();
            transform.scale = Vec3::new(transform.scale.x, 0.2, transform.scale.z);
            transform.translation =
                Vec3::new(transform.translation.x, 0.1, transform.translation.z);
        } else if let Some(res) = resources.resources.get(actor) {
            if res.stun_duration > 0 {
                transform.scale = Vec3::new(transform.scale.x, 0.2, transform.scale.z);
                transform.translation =
                    Vec3::new(transform.translation.x, 0.1, transform.translation.z);
            } else {
                transform.scale = Vec3::new(transform.scale.x, 1., transform.scale.z);
                transform.translation =
                    Vec3::new(transform.translation.x, 0.5, transform.translation.z);
            }
        }
    }
}

fn react_to_actor_events(
    _commands: Commands,
    _query: Query<(&Actor, &Transform)>,
    _events: EventReader<AdjustActorEvent>,
) {
}
