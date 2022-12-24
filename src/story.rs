use bevy::prelude::Resource;
use bevy_turborand::rng::Rng;

use crate::tracery_generator::TraceryGenerator;

#[derive(Debug, Clone, Resource)]
pub struct Story {
    pub main_character: String,
    pub good_faction: String,
    pub bad_faction: String,
    pub evil_lord: String,
    pub phase: StoryPhase,
    pub scenarios: Vec<Scenario>,
}

#[derive(Clone, Debug, Copy)]
pub enum StoryPhase {
    Start,
    RoughAndTumble(u8, u8),
    EarlySuccesses(u8, u8),
    Fallback(u8, u8),
    ClimbToTheEnd(u8, u8),
    FinalConfrontation,
}

impl Default for StoryPhase {
    fn default() -> Self {
        Self::Start
    }
}

#[derive(Clone, Debug)]
pub enum Scenario {
    InProgress {
        description: String,
        goals: Vec<Goal>,
    },
    Succeeded {
        description: String,
    },
    Failed {
        description: String,
    },
}

#[derive(Clone, Debug)]
pub enum Goal {
    ReachLocation,
    BeatBaddies,
}

impl Story {
    pub fn generate(rng: &mut Rng, asset: &TraceryGenerator) -> Self {
        Self {
            phase: StoryPhase::Start,
            scenarios: vec![],
            main_character: asset.generate_from("main_character", rng),
            good_faction: asset.generate_from("good_faction", rng),
            bad_faction: asset.generate_from("bad_faction", rng),
            evil_lord: asset.generate_from("evil_lord", rng),
        }
    }

    fn process_text(&self, text: &str) -> String {
        let updated = text.replace("*main_character*", &self.main_character);
        let updated = updated.replace("*good_faction*", &self.good_faction);
        let updated = updated.replace("*bad_faction*", &self.main_character);

        updated.replace("*evil_lord*", &self.bad_faction)
    }

    pub fn introduce(&mut self, rng: &mut Rng, asset: &TraceryGenerator) -> String {
        let text = asset.generate_from("intro", rng);
        self.process_text(&text)
    }
}
