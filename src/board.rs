use bevy::prelude::*;

use bevy_mod_picking::{Highlighting, NoDeselect, PickableBundle, PickingEvent};
use bevy_sequential_actions::{
    Action, ActionCommands, ActionFinished, ActionsBundle, ActionsProxy, ModifyActions, StopReason,
};

use smooth_bevy_cameras::LookTransform;

use crate::game_state::AppState;
use crate::scenario::{
    Actor, ActorPosition, AnimateActionsEvents, CurrentTurnProcess, ScenarioMap, TargetSelection,
};
use crate::scene::SceneState;

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
                    .with_system(wait_system)
                    .with_system(continue_system)
                    .with_system(setup_selectable)
                    .with_system(process_selection_events)
                    .with_system(set_selection)
                    .with_system(move_system),
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
    selector: Handle<Mesh>,
    selector_mat: Handle<StandardMaterial>,
    selector_active: Handle<StandardMaterial>,
    selector_hover: Handle<StandardMaterial>,
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
    commands.insert_resource(BoardAssets {
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
    assets: Res<BoardAssets>,
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
                    actions.add(WaitAction {
                        duration: *s,
                        current: None,
                    });
                }
                AnimateActionsEvents::Continue(actor) => {
                    actions.add(ContinueAction(*actor));
                }
                AnimateActionsEvents::SelectTargets(target_selection) => {
                    bevy::log::info!("Animate Selecting Targets");
                    actions.add(SelectTargetsAction(target_selection.clone()));
                }
                AnimateActionsEvents::Move(actor, position) => {
                    bevy::log::info!("Move To Target");
                    actions.add(MoveAction { actor: actor.clone(), position: position.clone(), speed: 10.});
                },
            }
        }
    }
}

pub struct MoveAction {
    position: ActorPosition,
    actor: Actor,
    speed: f32
}

impl Action for MoveAction {
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {

        // Run the wait system on the agent
        world.entity_mut(agent).insert(Move(self.actor, self.position, self.speed));
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, reason: StopReason) {
        // Remove the wait component from the agent
        let wait = world.entity_mut(agent).remove::<Move>();
    }
}

#[derive(Component)]
struct Move(Actor, ActorPosition, f32);

fn move_system(mut move_q: Query<(&mut Move, &mut ActionFinished)>, mut moveable: Query<(&Actor, &mut ActorPosition, &mut Transform)>, time: Res<Time>) {
    for (target, mut finished) in move_q.iter_mut() {
        for (actor, mut pos, mut transform) in moveable.iter_mut() {
            if actor == &target.0 {
                let current_position = transform.translation;
                let target_position = Vec3::new(target.1.0 as f32, current_position.y,  target.1.1 as f32);
                let delta = target_position - transform.translation;
                let distance_to_move = time.delta_seconds() * target.2;

                if delta.length_squared() <= distance_to_move {
                    transform.translation = target_position;
                    pos.0 = target.1.0;
                    pos.1 = target.1.1;
                    finished.confirm_and_reset();
                } else {
                    let move_vector = delta.normalize() * distance_to_move;
                    transform.translation += move_vector;
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
        world.entity_mut(agent).insert(Continue(self.0));
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
    }
}

struct SelectTargetsAction(TargetSelection);

impl Action for SelectTargetsAction {
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {
        world
            .entity_mut(agent)
            .insert(SelectTargets(self.0.clone()));
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, _reason: StopReason) {
        let _cont = world.entity_mut(agent).remove::<SelectTargets>();
    }
}

#[derive(Component)]
struct SelectTargets(TargetSelection);

#[derive(Component)]
struct Selectable(usize, usize);

#[derive(Component)]
struct Selected;

fn setup_selectable(
    mut commands: Commands,
    board: Query<(Entity, &SelectTargets), Changed<SelectTargets>>,
    assets: Res<BoardAssets>,
) {
    if let Ok((board, select_targets)) = board.get_single() {
        bevy::log::info!("Displaying Selected Targets");
        commands.entity(board).add_children(|parent| {
            for pos in select_targets.0.valid_targets.iter() {
                parent.spawn((
                    Selectable(pos.0, pos.1),
                    PickableBundle::default(),
                    Highlighting {
                        initial: assets.selector_mat.clone(),
                        hovered: Some(assets.selector_hover.clone()),
                        pressed: Some(assets.selector_active.clone()),
                        selected: Some(assets.selector_active.clone()),
                    },
                    NoDeselect,
                    PbrBundle {
                        mesh: assets.selector.clone(),
                        material: assets.selector_mat.clone(),
                        transform: Transform::from_xyz(pos.0 as f32, 0., pos.1 as f32),
                        ..Default::default()
                    },
                ));
            }
        });
    }
}

fn process_selection_events(
    mut events: EventReader<PickingEvent>,
    mut commands: Commands,
    assets: Res<BoardAssets>,
    mut selectables: Query<
        (
            Entity,
            Option<&Selected>,
            &mut Highlighting<StandardMaterial>,
        ),
        With<Selectable>,
    >,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => {
                info!("Clicked...");
                if let Ok((entity, selected, mut highlighting)) = selectables.get_mut(*e) {
                    if selected.is_none() {
                        info!("Selected");
                        commands.entity(entity).insert(Selected);
                        highlighting.initial = assets.selector_active.clone();
                    } else {
                        info!("Deselected");
                        commands.entity(entity).remove::<Selected>();
                        highlighting.initial = assets.selector_mat.clone();
                    }
                }
            }
        }
    }
}

fn set_selection(
    mut commands: Commands,
    mut board: Query<(Entity, &SelectTargets, &mut ActionFinished)>,
    selectables: Query<(Entity, &Selectable), With<Selected>>,
) {
    if let Ok((_board, select_targets, mut action_finished)) = board.get_single_mut() {
        info!("Checking selections...");
        let select = &select_targets.0;
        let count = selectables.iter().count();
        if count >= select.num_targets_to_select || count >= select.valid_targets.len() {
            info!("Got all selections");
            let selected = selectables.iter().map(|(_, s)| (s.0, s.1)).collect();
            let process = CurrentTurnProcess::CardTargetsSelected(
                select.actor,
                select.card.clone(),
                selected,
                select.action_id,
            );
            commands.insert_resource(process);
            action_finished.confirm_and_reset();
            for (e, _) in selectables.iter() {
                commands.entity(e).despawn_recursive();
            }
        }
    }
}
