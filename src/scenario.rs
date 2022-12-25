use bevy::{prelude::*, utils::HashMap};
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::{
    card::{Card, Cards, Targetable},
    game_state::AppState,
    scene::SceneState,
    story::Scenario,
};

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CardPlayedEvent>()
            .add_event::<AnimateActionsEvents>()
            .init_resource::<ActorResources>()
            .insert_resource(CurrentTurnProcess::None)
            .add_system_set(SystemSet::on_enter(SceneState::Setup).with_system(setup_scenario))
            .add_system_set(
                SystemSet::on_enter(SceneState::EnemyTurn).with_system(choose_enemy_card),
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

#[derive(Debug, Clone)]
pub struct CardPlayedEvent {
    pub actor: Actor,
    pub card: String,
}

#[derive(Debug, Clone, Resource)]
pub struct ScenarioMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Actor {
    Player,
    Enemy(usize),
}

impl PartialOrd for Actor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Actor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Actor::Player, Actor::Player) => 0usize.cmp(&0),
            (Actor::Player, Actor::Enemy(_)) => 0usize.cmp(&1),
            (Actor::Enemy(_), Actor::Player) => 1usize.cmp(&0),
            (Actor::Enemy(a), Actor::Enemy(b)) => (*a).cmp(b),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActorPosition(pub usize, pub usize);

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

        let map = ScenarioMap::generate(global_rng.as_mut(), scenario.as_ref(), &resources);

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

#[derive(Debug, Clone, Default)]
pub struct Tile {
    pub pos: (usize, usize),
    pub tile_type: TileType,
    pub entity: Option<Entity>,
    pub tag: TileTag,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TileType {
    Empty,
    Floor,
    Obstacle,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileTag {
    None,
    Start,
    Target(usize),
    Enemy(Actor),
}

impl Default for TileType {
    fn default() -> Self {
        Self::Empty
    }
}

impl Default for TileTag {
    fn default() -> Self {
        Self::None
    }
}

impl ScenarioMap {
    pub fn generate<T: DelegatedRng>(
        rng: &mut T,
        scenario: &Scenario,
        resources: &HashMap<Actor, ActorResource>,
    ) -> ScenarioMap {
        let width = rng.usize(10..=20);
        let height = rng.usize(10..=20);

        let width_tiles = 0usize..width;

        let mut tiles: Vec<Tile> = width_tiles
            .into_iter()
            .flat_map(|w| {
                let height_tiles = 0usize..height;
                height_tiles
                    .into_iter()
                    .map(|h| Tile {
                        pos: (w, h),
                        ..Default::default()
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        random_place_tile(rng, &mut tiles, Some(TileType::Floor), Some(TileTag::Start));

        for (i, _) in scenario.goals.iter().enumerate() {
            random_place_tile(
                rng,
                &mut tiles,
                Some(TileType::Floor),
                Some(TileTag::Target(i)),
            );
        }

        for (actor, _resource) in resources {
            random_place_tile(
                rng,
                &mut tiles,
                Some(TileType::Floor),
                Some(TileTag::Enemy(*actor)),
            );
        }

        for tile in tiles.iter_mut() {
            if tile.tile_type != TileType::Empty || tile.tag != TileTag::None {
                continue;
            }
            let probability = rng.f32_normalized();
            tile.tile_type = match probability {
                p if p < 0.7 => TileType::Floor,
                p if p < 0.8 => TileType::Obstacle,
                p if p < 0.9 => TileType::Wall,
                _ => TileType::Empty,
            };
        }

        ScenarioMap {
            width,
            height,
            tiles,
        }
    }
}

fn random_place_tile<T: DelegatedRng>(
    rng: &mut T,
    tiles: &mut Vec<Tile>,
    tile_type: Option<TileType>,
    tag: Option<TileTag>,
) {
    let index = rng.usize(0..tiles.len());
    if let Some(mut tile) = tiles.get_mut(index) {
        if let Some(tile_type) = tile_type {
            tile.tile_type = tile_type;
        }
        if let Some(tag) = tag {
            tile.tag = tag;
        }
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

                    if actor_resources.hand.len() == 0 {
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
    map: Option<Res<ScenarioMap>>,
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

                let valid_targets =
                    propose_valid_targets(actor, &targetable, &positions, &map, resources.as_ref());

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
                        let targets = select_target(global_rng.as_mut(), &target_selection);
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

pub fn propose_valid_targets(
    actor: &Actor,
    targetable: &Targetable,
    positions: &[(Actor, ActorPosition)],
    map: &ScenarioMap,
    _resources: &ActorResources,
) -> Vec<(usize, usize)> {
    let my_position = positions
        .iter()
        .find_map(|(a, p)| if a == actor { Some(*p) } else { None });
    match targetable {
        Targetable::Path { max_distance } => {
            if let Some(my_position) = my_position {
                let positions = map
                    .tiles
                    .iter()
                    .filter_map(|t| {
                        if t.tile_type == TileType::Floor {
                            Some(t.pos)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                positions_within_n(&(my_position.0, my_position.1), &positions, *max_distance)
            } else {
                vec![]
            }
        }
    }
}

fn positions_within_n(
    position: &(usize, usize),
    positions: &[(usize, usize)],
    distance: usize,
) -> Vec<(usize, usize)> {
    let mut checked = vec![];
    let mut to_check = vec![*position];
    let mut unchecked = positions
        .iter()
        .filter_map(|p| if p != position { Some(*p) } else { None })
        .collect::<Vec<_>>();

    for _ in 0..distance {
        let mut remove = vec![];
        let mut next_check = vec![];
        for pos in to_check.iter() {
            for (i, p) in unchecked.iter().enumerate() {
                if p.0.abs_diff(pos.0) <= 1 && p.1.abs_diff(pos.1) <= 1 {
                    remove.push(i);
                    if !checked.contains(p) {
                        next_check.push(*p);
                    }
                }
            }
        }
        unchecked = unchecked
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if remove.contains(&i) { None } else { Some(*v) })
            .collect();
        checked.append(&mut to_check);
        to_check.append(&mut next_check);
    }

    checked.append(&mut to_check);
    checked
}

pub fn select_target<T: DelegatedRng>(
    rng: &mut T,
    selection: &TargetSelection,
) -> Vec<(usize, usize)> {
    let mut selected = Vec::with_capacity(selection.num_targets_to_select);
    let valid_target_len = selection.valid_targets.len();
    let range = 0..valid_target_len;

    while selected.len() < selection.num_targets_to_select && selected.len() < valid_target_len {
        let mut next = rng.usize(range.clone());
        while selected.contains(&next) {
            next = rng.usize(range.clone());
        }
        selected.push(next);
    }

    selected
        .iter()
        .filter_map(|i| selection.valid_targets.get(*i).copied())
        .collect()
}

fn choose_enemy_card(
    mut commands: Commands,
    mut events: EventWriter<CardPlayedEvent>,
    mut global_rng: ResMut<GlobalRng>,
    current_turn_process: Option<Res<CurrentTurnProcess>>,
    resources: Option<Res<ActorResources>>,
    _cards: Res<Cards>,
) {
    info!("Choosing enemy card...");
    if let (Some(process), Some(resources)) = (current_turn_process, resources) {
        info!("Process can continue");
        match *process {
            CurrentTurnProcess::Thinking(actor) => {
                if let Some(res) = resources.resources.get(&actor) {
                    let hand = &res.hand;
                    let range = 0..hand.len();
                    let selected = global_rng.usize(range);
                    if let Some(selected) = hand.get(selected) {
                        info!("Playing a card {:?}", selected);
                        events.send(CardPlayedEvent {
                            actor,
                            card: selected.clone(),
                        });
                        return;
                    }
                }
                info!("Couldn't play anything, skipping turn");
                commands.insert_resource(CurrentTurnProcess::Done(actor));
            }
            _ => {
                info!("Not thinking - why?");
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TargetSelection {
    pub actor: Actor,
    pub card: Card,
    pub valid_targets: Vec<(usize, usize)>,
    pub num_targets_to_select: usize,
    pub action_id: usize,
}

#[derive(Debug, Clone)]
pub enum AnimateActionsEvents {
    Wait(f32),
    Continue(Actor),
    SelectTargets(TargetSelection),
    Move(Actor, ActorPosition),
    SetTurnProcess(CurrentTurnProcess),
}

#[derive(Debug, Clone, Resource)]
pub enum CurrentTurnProcess {
    None,
    Thinking(Actor),
    CardActionTriggered(Actor, Card, usize),
    CardTargetsSelected(Actor, Card, Vec<(usize, usize)>, usize),
    Done(Actor),
}

fn current_turn_process_changed(p: Option<Res<CurrentTurnProcess>>) {
    if let Some(p) = p {
        if p.is_changed() {
            info!("Process Changed: {:?}", p);
        }
    }
}
