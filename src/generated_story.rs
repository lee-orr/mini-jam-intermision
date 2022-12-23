use bevy::{prelude::Plugin, reflect::TypeUuid, utils::HashMap};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_turborand::rng::{Rng, RandBorrowed};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, TypeUuid)]
#[uuid = "ec95bab0-e021-4bd0-9cc1-b37d6b152ca0"]
pub struct StoryGenerator {
    pub grammar: HashMap<String, Vec<String>>,
    pub default: String,
}

impl StoryGenerator {
    pub fn generate(&self, rng: &mut Rng) -> String{
        let mut rng = RandBorrowed::from(rng);
        let text = 
            if let Ok(mut grammar) = tracery::from_map(self.grammar.iter()) {
                if let Ok(output) = grammar.execute(&self.default, &mut rng) {
                    output
                } else {
                    "failed".to_string()
                }
            } else {
                "none".to_string()
            };
        text
    }
    pub fn generate_at(&self, key: &String, rng: &mut Rng) -> String{
        let mut rng = RandBorrowed::from(rng);
        let text = 
            if let Ok(mut grammar) = tracery::from_map(self.grammar.iter()) {
                if let Ok(output) = grammar.execute(key, &mut rng) {
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

pub struct StoryGeneratorPlugin;

impl Plugin for StoryGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(JsonAssetPlugin::<StoryGenerator>::new(&["story"]));
    }
}
