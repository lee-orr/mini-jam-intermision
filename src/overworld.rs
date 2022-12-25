use bevy::prelude::*;

use bevy_turborand::RngComponent;

use crate::{assets, game_state::AppState, story::*, tracery_generator::TraceryGenerator, ui::*};

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::Overworld).with_system(setup_overworld))
            .add_system_set(SystemSet::on_update(AppState::Overworld).with_system(check_click))
            .add_system_set(clear_ui_system_set(AppState::Overworld));
    }
}

fn setup_overworld(
    mut commands: Commands,
    assets: Res<assets::Assets>,
    stories: Res<Assets<TraceryGenerator>>,
    story: Option<ResMut<Story>>,
    current_scenario: Option<Res<Scenario>>,
) {
    let story_scenario: Option<(Story, Scenario)> = if let Some(mut story) = story {
        let current_scenario = current_scenario.map(|s| s.into_inner());
        story
            .generate_next_scenario(current_scenario)
            .map(|scenario| (story.to_owned(), scenario))
    } else {
        let mut rng = RngComponent::new();
        if let Some(asset) = stories.get(&assets.story) {
            let mut story = Story::generate(&mut rng, asset);
            story
                .generate_next_scenario(None)
                .map(|scenario| (story, scenario))
        } else {
            None
        }
    };

    if story_scenario.is_none() {
        return;
    }
    let (story, scenario) = story_scenario.unwrap();

    bevy::log::info!("Setup Overworld UI");
    UiRoot::spawn(&mut commands, |parent| {
        if StoryPhase::Complete == story.phase {
            MainText::new("The End").size(100.).spawn(parent, &assets);
            MenuButton::Primary.spawn("end", "Back To Menu", parent, &assets);
        } else {
            MainText::new(&scenario.initial_description)
                .size(30.)
                .alignment(JustifyContent::Center)
                .spawn(parent, &assets);
            MenuButton::Primary.spawn("start_scenario", "Start Scenario", parent, &assets);
        }
    });

    commands.insert_resource(story);
    commands.insert_resource(scenario);
}

fn check_click(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    mut clicked: EventReader<ButtonClickEvent>,
) {
    for click in clicked.iter() {
        let ButtonClickEvent(val, _) = click;
        if val == "start_scenario" {
            let _ = app_state.set(AppState::Scene);
        } else if val == "end" {
            let _ = app_state.set(AppState::MainMenu);
            commands.remove_resource::<Story>();
        }
    }
}
