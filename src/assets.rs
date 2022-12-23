use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::generated_story::StoryGenerator;

#[derive(AssetCollection, Resource)]
pub struct Assets {
    #[asset(path = "Xolonium-Regular.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "test_story.story")]
    pub story: Handle<StoryGenerator>,
}
