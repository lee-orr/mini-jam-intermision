use bevy::{prelude::*, reflect::TypeUuid};
use bevy_common_assets::yaml::YamlAssetPlugin;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, TypeUuid)]
#[uuid = "4a4c156e-fe23-44b3-be85-0c107d31cc54"]
pub struct Card {
    pub name: String,
}

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(YamlAssetPlugin::<Card>::new(&["card.yaml"]));
    }
}
