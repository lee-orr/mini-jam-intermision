use bevy::prelude::*;
use bevy_turborand::DelegatedRng;

use crate::story::Scenario;

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {}
}

#[derive(Debug, Clone, Resource)]
pub struct ScenarioMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
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
    Target,
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

        for _i in scenario.goals.iter() {
            random_place_tile(
                rng,
                &mut tiles,
                Some(TileType::Floor),
                Some(TileTag::Target),
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
