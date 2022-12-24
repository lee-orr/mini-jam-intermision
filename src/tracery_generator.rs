use bevy::{prelude::Plugin, reflect::TypeUuid, utils::HashMap};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_turborand::{rng::{RandBorrowed, Rng, RandCompat}, RngComponent, DelegatedRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, TypeUuid)]
#[uuid = "ec95bab0-e021-4bd0-9cc1-b37d6b152ca0"]
pub struct TraceryGenerator {
    pub grammar: HashMap<String, Vec<String>>,
}

impl TraceryGenerator {
    pub fn generate_from<T: Into<String>>(&self, key: T, rng: &mut RngComponent) -> String {
        let mut rng = rng.as_rand();
        let text = if let Ok(mut grammar) = tracery::from_map(self.grammar.iter()) {
            if let Ok(output) = grammar.execute(&key.into(), &mut rng) {
                output
            } else {
                "failed".to_string()
            }
        } else {
            "none".to_string()
        };
        text
    }
}

pub struct TraceryPlugin;

impl Plugin for TraceryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(JsonAssetPlugin::<TraceryGenerator>::new(&["trace"]))
        .add_plugin(YamlAssetPlugin::<TraceryGenerator>::new(&["trace.yaml"]));
    }
}
