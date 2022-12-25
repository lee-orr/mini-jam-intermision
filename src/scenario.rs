mod enemy_ai;
pub mod scenario_map;
mod scenario_utilities;
pub mod turn_types;

use bevy::{prelude::*, utils::HashMap};
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::{card::Cards, game_state::AppState, scene::SceneState, story::Scenario};

pub use scenario_map::*;
pub use turn_types::*;

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CardPlayedEvent>()
            .add_event::<AnimateActionsEvents>()
            .init_resource::<ActorResources>()
            .insert_resource(CurrentTurnProcess::None)
            .add_system_set(SystemSet::on_enter(SceneState::Setup).with_system(setup_scenario))
            .add_system_set(
                SystemSet::on_enter(SceneState::EnemyTurn).with_system(enemy_ai::choose_enemy_card),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Scene)
                    .with_system(current_turn_process_changed)
                    .with_system(process_card_events)
                    .with_system(next_turn_ready)
                    .with_system(process_card_action)
                    .with_system(apply_action_to_targets),
            );
    }
}

#[derive(Default, Debug, Clone)]
pub struct ActorResource {
    pub hand: Vec<String>,
    pub used: Vec<String>,
    pub discarded: Vec<String>,
    pub health: usize,
}

#[derive(Default, Debug, Clone, Resource)]
pub struct ActorResources {
    pub resources: HashMap<Actor, ActorResource>,
    pub turn_order: Vec<Actor>,
}

fn setup_scenario(
    mut commands: Commands,
    cards: Res<Cards>,
    mut global_rng: ResMut<GlobalRng>,
    current_scenario: Option<Res<Scenario>>,
) {
    if let Some(scenario) = current_scenario {
        let cards = cards.cards.iter().collect::<Vec<_>>();
        let num_cards = cards.len();

        let mut selected = Vec::with_capacity(3);

        for _i in 0..3 {
            let mut next = global_rng.usize(0..num_cards);
            while selected.contains(&next) {
                next = global_rng.usize(0..num_cards);
            }
            selected.push(next);
        }

        let selected = selected
            .iter()
            .filter_map(|i| cards.get(*i))
            .map(|(id, _)| id.to_string())
            .collect::<Vec<_>>();

        let resources = (0..global_rng.usize(3..5))
            .map(|i| {
                (
                    Actor::Enemy(i),
                    ActorResource {
                        hand: selected.clone(),
                        health: 2,
                        ..Default::default()
                    },
                )
            })
            .collect();

        let map =
            scenario_map::ScenarioMap::generate(global_rng.as_mut(), scenario.as_ref(), &resources);

        let mut turn_order = resources
            .iter()
            .map(|(actor, _)| *actor)
            .collect::<Vec<_>>();
        turn_order.push(Actor::Player);
        turn_order.sort();

        commands.insert_resource(ActorResources {
            resources,
            turn_order,
        });

        commands.insert_resource(map);
    }
}

fn process_card_events(
    mut events: EventReader<CardPlayedEvent>,
    mut resources: Option<ResMut<ActorResources>>,
    cards: Res<Cards>,
    mut scene_state: ResMut<State<SceneState>>,
    mut animate: EventWriter<AnimateActionsEvents>,
    mut commands: Commands,
) {
    for CardPlayedEvent { actor, card } in events.iter() {
        info!("Setting state to processing");
        let _ = scene_state.overwrite_set(SceneState::Processing);
        if let Some(resources) = resources.as_mut() {
            // Check Hand
            if let Some(actor_resources) = resources.resources.get_mut(actor) {
                if actor_resources.hand.contains(card) {
                    actor_resources.hand.retain(|c| c != card);
                    actor_resources.used.push(card.clone());

                    if actor_resources.hand.is_empty() {
                        actor_resources.hand = actor_resources.used.clone();
                        actor_resources.used = vec![];
                    }

                    if let Some(card) = cards.cards.get(card) {
                        commands.insert_resource(CurrentTurnProcess::CardActionTriggered(
                            *actor,
                            card.clone(),
                            0,
                        ));
                        bevy::log::info!("Triggered Card Action");
                        return;
                    }
                }
            }

            queue_next_turn(&mut commands, resources, actor, &mut animate);
        } else {
            commands.insert_resource(CurrentTurnProcess::Thinking(Actor::Player));
            animate.send(AnimateActionsEvents::Wait(0.2));
            animate.send(AnimateActionsEvents::Continue(Actor::Player));
        }
    }
}

fn next_turn_ready(
    current_turn_process: Res<CurrentTurnProcess>,
    mut commands: Commands,
    mut resources: ResMut<ActorResources>,
    mut animate: EventWriter<AnimateActionsEvents>,
) {
    if !current_turn_process.is_changed() {
        return;
    }

    if let CurrentTurnProcess::Done(actor) = *current_turn_process {
        queue_next_turn(&mut commands, &mut resources, &actor, &mut animate);
    }
}

fn queue_next_turn(
    commands: &mut Commands,
    resources: &mut ResMut<ActorResources>,
    actor: &Actor,
    animate: &mut EventWriter<AnimateActionsEvents>,
) {
    if let Some(current_turn) =
        resources.turn_order.iter().enumerate().find_map(
            |(id, a)| {
                if a == actor {
                    Some(id)
                } else {
                    None
                }
            },
        )
    {
        let next_turn = current_turn + 1;
        let next_turn = if next_turn >= resources.turn_order.len() {
            0
        } else {
            next_turn
        };
        if let Some(next_actor) = resources.turn_order.get(next_turn) {
            commands.insert_resource(CurrentTurnProcess::Thinking(*next_actor));
            animate.send(AnimateActionsEvents::Wait(0.2));
            animate.send(AnimateActionsEvents::Continue(*next_actor));
        }
    } else {
        commands.insert_resource(CurrentTurnProcess::Thinking(Actor::Player));
        animate.send(AnimateActionsEvents::Wait(0.2));
        animate.send(AnimateActionsEvents::Continue(Actor::Player));
    }
}

fn process_card_action(
    current_turn_process: Res<CurrentTurnProcess>,
    mut commands: Commands,
    resources: Option<ResMut<ActorResources>>,
    map: Option<Res<scenario_map::ScenarioMap>>,
    position_query: Query<(&Actor, &ActorPosition)>,
    mut animate: EventWriter<AnimateActionsEvents>,
    mut global_rng: ResMut<GlobalRng>,
) {
    if !current_turn_process.is_changed() {
        return;
    }

    if let CurrentTurnProcess::CardActionTriggered(actor, card, action_index) =
        &*current_turn_process
    {
        bevy::log::info!("Processing Card Action");
        if let (Some(resources), Some(map)) = (resources, map) {
            if let Some(current_action) = card.actions.get(*action_index) {
                let targetable = current_action.target();

                let positions = position_query
                    .iter()
                    .map(|(a, p)| (*a, *p))
                    .collect::<Vec<_>>();

                let valid_targets = scenario_utilities::propose_valid_targets(
                    actor,
                    &targetable,
                    &positions,
                    &map,
                    resources.as_ref(),
                );

                if valid_targets.is_empty() {
                    commands.insert_resource(CurrentTurnProcess::CardTargetsSelected(
                        *actor,
                        card.clone(),
                        vec![],
                        *action_index,
                    ));
                    return;
                }

                let target_selection = TargetSelection {
                    actor: *actor,
                    card: card.clone(),
                    valid_targets,
                    num_targets_to_select: targetable.num_targets(),
                    action_id: *action_index,
                };

                match actor {
                    Actor::Player => {
                        bevy::log::info!("Setting Up Valid Player Targets");
                        animate.send(AnimateActionsEvents::SelectTargets(target_selection));
                    }
                    Actor::Enemy(_) => {
                        bevy::log::info!("Selecting Enemy Targets");
                        let targets =
                            enemy_ai::select_target(global_rng.as_mut(), &target_selection);
                        info!("Enemy Targets Selected");
                        let TargetSelection {
                            actor,
                            card,
                            valid_targets: _,
                            num_targets_to_select: _,
                            action_id,
                        } = target_selection;
                        commands.insert_resource(CurrentTurnProcess::CardTargetsSelected(
                            actor, card, targets, action_id,
                        ));
                    }
                }
            } else {
                commands.insert_resource(CurrentTurnProcess::Done(*actor));
            }
        }
    }
}

fn apply_action_to_targets(
    current_turn_process: Res<CurrentTurnProcess>,
    mut animate: EventWriter<AnimateActionsEvents>,
) {
    if !current_turn_process.is_changed() {
        return;
    }

    if let CurrentTurnProcess::CardTargetsSelected(actor, card, targets, action_index) =
        &*current_turn_process
    {
        if let Some(action) = card.actions.get(*action_index) {
            match action {
                crate::card::CardAction::Move(_) => {
                    if let Some(position) = targets.first() {
                        animate.send(AnimateActionsEvents::Move(
                            *actor,
                            ActorPosition(position.0, position.1),
                        ));
                    }
                }
            }
        }

        let next_action = action_index + 1;

        if card.actions.len() <= next_action {
            info!("Turn Complete - schedule done");
            animate.send(AnimateActionsEvents::SetTurnProcess(
                CurrentTurnProcess::Done(*actor),
            ));
        } else {
            info!("Turn Continues - schedule next action");
            animate.send(AnimateActionsEvents::SetTurnProcess(
                CurrentTurnProcess::CardActionTriggered(*actor, card.clone(), next_action),
            ));
        }
    }
}

fn current_turn_process_changed(p: Option<Res<CurrentTurnProcess>>) {
    if let Some(p) = p {
        if p.is_changed() {
            info!("Process Changed: {:?}", p);
        }
    }
}
