use bevy::prelude::Resource;
use bevy_turborand::RngComponent;

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

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StoryPhase {
    Setup,
    Start,
    // RoughAndTumble(u8, u8),
    // EarlySuccesses(u8, u8),
    // Fallback(u8, u8),
    // ClimbToTheEnd(u8, u8),
    FinalConfrontation,
    Complete,
}

impl Default for StoryPhase {
    fn default() -> Self {
        Self::Setup
    }
}

impl StoryPhase {}

#[derive(Clone, Debug)]
pub struct Scenario {
    pub initial_description: String,
    pub state: ScenarioState,
    pub goals: Vec<Goal>,
}

#[derive(Clone, Debug)]
pub enum ScenarioState {
    InProgress(usize),
    Success(String),
    Failure(String),
}

#[derive(Default, Clone, Debug)]
pub struct Goal {
    pub description: String,
    pub success: String,
    pub failure: String,
    pub goal_type: GoalType,
}

#[derive(Clone, Debug)]
pub enum GoalType {
    ReachLocation(String),
}

impl Default for GoalType {
    fn default() -> Self {
        Self::ReachLocation("".to_string())
    }
}

impl Goal {
    pub fn parse(string: &str) -> Vec<Self> {
        bevy::log::info!("Parsing goal - {string}");
        string
            .split('|')
            .filter_map(|v| {
                let v = v.trim();
                bevy::log::info!("Parsing goal section - {v}");
                let mut split = v.split(':');
                let description = split.next();
                let success = split.next();
                let failure = split.next();
                let goal_type = split.next();
                if let (Some(description), Some(success), Some(failure), Some(goal_type)) =
                    (description, success, failure, goal_type)
                {
                    let goal_type = match goal_type {
                        "reach-location" => {
                            if let Some(target_location) = split.next() {
                                let goal = target_location.trim().to_string();
                                Some(GoalType::ReachLocation(goal))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    goal_type.map(|goal_type| Goal {
                        description: description.to_string(),
                        success: success.to_string(),
                        failure: failure.to_string(),
                        goal_type,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Scenario {
    pub fn parse(string: &str) -> Option<Scenario> {
        let mut split = string.split('@');
        bevy::log::info!("Parsing Scenario: {:?}", &split);
        if let (Some(initial_description), Some(goal_data)) = (split.next(), split.next()) {
            let goals = Goal::parse(goal_data);
            Some(Scenario {
                initial_description: initial_description.trim().to_string(),
                state: ScenarioState::InProgress(0),
                goals,
            })
        } else {
            None
        }
    }

    pub fn succeed(&mut self) -> &ScenarioState {
        bevy::log::info!("Succeeding");
        if let ScenarioState::InProgress(count) = self.state {
            let next = count + 1;
            bevy::log::info!(
                "Currently at {count}, next is {next}, len is {}",
                self.goals.len()
            );
            if self.goals.len() > next {
                bevy::log::info!("On to the next one");
                self.state = ScenarioState::InProgress(next);
            } else {
                bevy::log::info!("Scenario completed!");
                let text = if let Some(goal) = self.goals.last() {
                    goal.success.clone()
                } else {
                    "Scenario Succeeded".to_string()
                };
                self.state = ScenarioState::Success(text);
            }
        }
        &self.state
    }

    pub fn fail(&mut self) -> &ScenarioState {
        let text = if let Some(goal) = self.goals.last() {
            goal.failure.clone()
        } else {
            "Scenario Failed".to_string()
        };
        self.state = ScenarioState::Failure(text);
        &self.state
    }
}

impl Story {
    pub fn generate(rng: &mut RngComponent, asset: &TraceryGenerator) -> Self {
        Self {
            phase: StoryPhase::Setup,
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

    fn generate_scenario(&mut self) -> bool {
        let key = match self.phase {
            StoryPhase::Setup => "intro",
            StoryPhase::Start => "intro",
            StoryPhase::FinalConfrontation => "confrontation",
            StoryPhase::Complete => "complete",
        };
        bevy::log::info!("Generating Scenario with key {key}");
        let text = self.asset.generate_from(key, &mut self.rng);
        let text = self.process_text(&text);
        bevy::log::info!("Scenario Text {text}");
        if let Some(scenario) = Scenario::parse(&text) {
            self.scenarios.push(scenario);
            true
        } else {
            false
        }
    }

    pub fn get_current_scenario(&self) -> Option<&Scenario> {
        self.scenarios.last()
    }

    pub fn generate_next_scenario(&mut self) {
        match self.phase {
            StoryPhase::Setup => {
                self.phase = StoryPhase::Start;
            }
            StoryPhase::Start => {
                self.phase = StoryPhase::FinalConfrontation;
            }
            StoryPhase::FinalConfrontation => {
                self.phase = StoryPhase::Complete;
            }
            StoryPhase::Complete => {}
        }
        self.generate_scenario();
    }
}
