use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{card::Card, tracery_generator::TraceryGenerator};

#[derive(AssetCollection, Resource)]
pub struct Assets {
    #[asset(key = "font")]
    pub font: Handle<Font>,
    #[asset(key = "story")]
    pub story: Handle<TraceryGenerator>,
    #[asset(key = "cards", collection(typed))]
    pub cards: Vec<Handle<Card>>,
}
