use bevy::{prelude::*, utils::HashMap};
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::{card::Cards, game_state::AppState, scene::SceneState, story::Scenario};

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CardPlayedEvent>()
            .add_event::<AnimateActionsEvents>()
            .init_resource::<ActorResources>()
            .add_system_set(
                SystemSet::on_enter(SceneState::Setup)
                    .with_system(setup_scenario),
            )
            .add_system_set(SystemSet::on_update(AppState::Scene).with_system(process_card_events));
    }
}

#[derive(Debug, Clone)]
pub struct CardPlayedEvent {
    pub actor: Actor,
    pub card: String,
    pub targets: Vec<(usize, usize, usize)>,
}

#[derive(Debug, Clone, Resource)]
pub struct ScenarioMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Actor {
    Player,
    Enemy(usize),
}

#[derive(Default, Debug, Clone)]
pub struct ActorResource {
    pub hand: Vec<String>,
    pub used: Vec<String>,
    pub discarded: Vec<String>,
    pub health: usize,
    pub position: Option<(usize, usize)>
}

#[derive(Default, Debug, Clone, Resource)]
pub struct ActorResources {
    pub resources: HashMap<Actor, ActorResource>,
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

        let resoures = (0..global_rng.usize(3..5))
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

        let map = ScenarioMap::generate(global_rng.as_mut(), scenario.as_ref());

        commands.insert_resource(ActorResources {
            resources: resoures,
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
    Enemy,
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
    pub fn generate<T: DelegatedRng>(rng: &mut T, scenario: &Scenario) -> ScenarioMap {
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

        for _i in 0..rng.usize(3..5) {
            random_place_tile(rng, &mut tiles, Some(TileType::Floor), Some(TileTag::Enemy));
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
    _map: Option<ResMut<ScenarioMap>>,
    mut resources: Option<ResMut<ActorResources>>,
    _cards: Res<Cards>,
    mut scene_state: ResMut<State<SceneState>>,
    mut animate: EventWriter<AnimateActionsEvents>,
) {
    let mut is_processing = false;
    for CardPlayedEvent {
        actor,
        card,
        targets: _,
    } in events.iter()
    {
        is_processing = true;
        if let Some(resources) = resources.as_mut() {
            if let Some(actor_resources) = resources.resources.get_mut(actor) {
                if actor_resources.hand.contains(card) {
                    actor_resources.hand.retain(|c| c != card);
                    actor_resources.used.push(card.clone());
                }
            }
        }
    }

    if is_processing {
        let _ = scene_state.set(SceneState::Processing);
        animate.send(AnimateActionsEvents::Wait(1.));
        animate.send(AnimateActionsEvents::Continue);
    }
}

#[derive(Debug, Clone)]
pub enum AnimateActionsEvents {
    Wait(f32),
    Continue,
}
