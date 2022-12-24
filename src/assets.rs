use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::tracery_generator::TraceryGenerator;

#[derive(AssetCollection, Resource)]
pub struct Assets {
    #[asset(path = "Xolonium-Regular.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "test_story.trace.yaml")]
    pub story: Handle<TraceryGenerator>,
}
