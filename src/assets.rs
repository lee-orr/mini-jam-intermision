use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_generative_grammars::tracery::TraceryGrammar;

use crate::card::Card;

#[derive(AssetCollection, Resource)]
pub struct Assets {
    #[asset(key = "font")]
    pub font: Handle<Font>,
    #[asset(key = "story")]
    pub story: Handle<TraceryGrammar>,
    #[asset(key = "cards", collection(typed))]
    pub cards: Vec<Handle<Card>>,
}
