use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct Assets {
    #[asset(path = "icon.png")]
    pub icon: Handle<Image>,
    #[asset(path = "Xolonium-Regular.ttf")]
    pub font: Handle<Font>,
}
