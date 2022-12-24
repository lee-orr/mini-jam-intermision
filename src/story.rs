use bevy::prelude::Resource;
use bevy_turborand::{rng::Rng, RngComponent};

use crate::tracery_generator::TraceryGenerator;

#[derive(Debug, Clone, Resource)]
pub struct Story {
    pub main_character: String,
    pub good_faction: String,
    pub bad_faction: String,
    pub evil_lord: String,
    pub phase: StoryPhase,
    pub scenarios: Vec<Scenario>,
    rng: RngComponent,
    asset: TraceryGenerator,
}

#[derive(Clone, Debug, Copy)]
pub enum StoryPhase {
    Start,
    // RoughAndTumble(u8, u8),
    // EarlySuccesses(u8, u8),
    // Fallback(u8, u8),
    // ClimbToTheEnd(u8, u8),
    FinalConfrontation,
}

impl Default for StoryPhase {
    fn default() -> Self {
        Self::Start
    }
}

impl StoryPhase {}

#[derive(Clone, Debug)]
pub struct Scenario {
    pub initial_description: String,
    pub state: ScenarioState,
    pub goals: Vec<Goal>
}

#[derive(Clone, Debug)]
pub enum ScenarioState {
    InProgress,
    Success,
    Failure
}

#[derive(Clone, Debug)]
pub enum Goal {
    ReachLocation(String, String, String),
}

impl Goal {
    pub fn parse(string: &str) -> Vec<Self> {
        string
            .split("|")
            .filter_map(|v| {
                let v = v.trim();
                let mut split = v.split(":");
                let goal_type = split.next();
                match goal_type {
                    Some("reach-location") => {
                        if let (Some(target_location), Some(success), Some(failure)) = (split.next(),split.next(), split.next()) {
                            Some(Self::ReachLocation(target_location.trim().to_string(), success.trim().to_string(), failure.trim().to_string()))
                        } else {
                            None
                        }
                    }
                    _ => None
                }
            }).collect()
    }
}

impl Scenario {
    pub fn parse(string: &str) -> Option<Scenario> {
        let mut split = string.split("@");
        if let (Some(initial_description), Some(goal_data)) = (split.next(), split.next()) {
            let goals = Goal::parse(goal_data);
            Some(Scenario {
                initial_description: initial_description.trim().to_string(),
                state: ScenarioState::InProgress,
                goals
            })
        } else {
            None
        }
    }
}

impl Story {
    pub fn generate(rng: &mut RngComponent, asset: &TraceryGenerator) -> Self {
        Self {
            phase: StoryPhase::Start,
            scenarios: vec![],
            main_character: asset.generate_from("main_character", rng),
            good_faction: asset.generate_from("good_faction", rng),
            bad_faction: asset.generate_from("bad_faction", rng),
            evil_lord: asset.generate_from("evil_lord", rng),
            rng: rng.clone(),
            asset: asset.clone(),
        }
    }

    fn process_text(&self, text: &str) -> String {
        let updated = text.replace("*main_character*", &self.main_character);
        let updated = updated.replace("*good_faction*", &self.good_faction);
        let updated = updated.replace("*bad_faction*", &self.bad_faction);

        updated.replace("*evil_lord*", &self.evil_lord)
    }

    pub fn introduce(&mut self) {
        let text = self.asset.generate_from("intro", &mut self.rng);
        let text = self.process_text(&text);
        if let Some(scenario) = Scenario::parse(&text) {
            self.scenarios.push(scenario);
        }
    }

    pub fn get_current_scenario(&self) -> Option<&Scenario> {
        self.scenarios.last()
    }
}
