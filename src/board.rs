use bevy::prelude::*;

use bevy_sequential_actions::{
    Action, ActionCommands, ActionFinished, ActionsBundle, ActionsProxy, ModifyActions, StopReason,
};
use bevy_turborand::GlobalRng;
use smooth_bevy_cameras::LookTransform;

use crate::game_state::AppState;
use crate::scenario::{Actor, AnimateActionsEvents, ScenarioMap};
use crate::{scene::SceneState, story::*};

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
                SystemSet::on_enter(SceneState::Setup)
                    .with_system(generate_board)
                    .with_system(set_camera),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Scene)
                    .with_system(animate_actions)
                    .with_system(wait_system)
                    .with_system(continue_system),
            );
    }
}

#[derive(Component)]
struct Board;

#[derive(Default, Resource)]
struct BoardAssets {
    tile: Handle<Mesh>,
    wall: Handle<Mesh>,
    obstacle: Handle<Mesh>,
    monster: Handle<Mesh>,
    player: Handle<Mesh>,
    tile_mat: Handle<StandardMaterial>,
    monster_mat: Handle<StandardMaterial>,
    player_mat: Handle<StandardMaterial>,
    goal_mat: Handle<StandardMaterial>,
    start_point_mat: Handle<StandardMaterial>,
}

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
    commands.insert_resource(BoardAssets {
        tile,
        wall,
        obstacle,
        tile_mat,
        goal_mat,
        start_point_mat,
        monster,
        player,
        monster_mat,
        player_mat,
    });
}

fn clear_board(mut commands: Commands, query: Query<Entity, With<Board>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn generate_board(
    mut commands: Commands,
    current_scenario: Option<Res<Scenario>>,
    assets: Res<BoardAssets>,
    mut global_rng: ResMut<GlobalRng>,
    _scene_state: ResMut<State<SceneState>>,
) {
    if let Some(scenario) = current_scenario {
        let scenario_map = ScenarioMap::generate(global_rng.as_mut(), &scenario);

        let left = -1. * scenario_map.width as f32 / 2.;
        let top = -1. * scenario_map.height as f32 / 2.;

        commands
            .spawn((SpatialBundle::default(), Board, ActionsBundle::new()))
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
                    let pos = (tile.pos.0 as f32 + left, tile.pos.1 as f32 + top);

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
                            parent.spawn(PbrBundle {
                                mesh: assets.player.clone(),
                                material: assets.player_mat.clone(),
                                transform: Transform::from_xyz(pos.0, 0.5, pos.1),
                                ..Default::default()
                            });
                        }
                        crate::scenario::TileTag::Enemy => {
                            parent.spawn(PbrBundle {
                                mesh: assets.monster.clone(),
                                material: assets.monster_mat.clone(),
                                transform: Transform::from_xyz(pos.0, 0.5, pos.1),
                                ..Default::default()
                            });
                        }
                        _ => {}
                    }
                }
            });

        commands.insert_resource(scenario_map);
    }
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
                    actions.add(WaitAction {
                        duration: *s,
                        current: None,
                    });
                }
                AnimateActionsEvents::Continue(actor) => {
                    actions.add(ContinueAction(actor.clone()));
                }
            }
        }
    }
}

pub struct WaitAction {
    duration: f32,
    current: Option<f32>,
}

impl Action for WaitAction {
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {
        // Take current duration (if paused), or use full duration
        let duration = self.current.take().unwrap_or(self.duration);

        // Run the wait system on the agent
        world.entity_mut(agent).insert(Wait(duration));
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, reason: StopReason) {
        // Remove the wait component from the agent
        let wait = world.entity_mut(agent).remove::<Wait>();

        // Store current duration when paused
        if let StopReason::Paused = reason {
            self.current = Some(wait.unwrap().0);
        }
    }
}

#[derive(Component)]
struct Wait(f32);

fn wait_system(mut wait_q: Query<(&mut Wait, &mut ActionFinished)>, time: Res<Time>) {
    for (mut wait, mut finished) in wait_q.iter_mut() {
        wait.0 -= time.delta_seconds();

        // Confirm finished state every frame
        if wait.0 <= 0.0 {
            finished.confirm_and_reset();
        }
    }
}

pub struct ContinueAction(Actor);

impl Action for ContinueAction {
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {
        // Run the wait system on the agent
        world.entity_mut(agent).insert(Continue(self.0.clone()));
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, _reason: StopReason) {
        // Remove the wait component from the agent
        let _cont = world.entity_mut(agent).remove::<Continue>();
    }
}

#[derive(Component)]
struct Continue(Actor);

fn continue_system(
    mut wait_q: Query<(&mut Continue, &mut ActionFinished)>,
    mut scene_state: ResMut<State<SceneState>>,
) {
    for (_cont, mut finished) in wait_q.iter_mut() {
        finished.confirm_and_reset();
        let _ = scene_state.set(SceneState::PlayerTurn);
        // match cont.0 {
        //     Actor::Player => {let _ = scene_state.set(SceneState::PlayerTurn)},
        //     Actor::Enemy(_) => {},
        // }
    }
}
