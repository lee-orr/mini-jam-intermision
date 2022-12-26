use bevy::prelude::*;

use bevy_turborand::RngComponent;

use crate::{
    assets,
    card::{AvailableCards, Cards},
    game_state::AppState,
    story::*,
    tracery_generator::TraceryGenerator,
    ui::*,
};

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
    mut available_cards: ResMut<AvailableCards>,
    cards: Res<Cards>,
) {
    let (story, scenario) = if let Some(mut story) = story {
        let current_scenario = current_scenario.map(|s| s.into_inner());
        let scenario = story.generate_next_scenario(current_scenario);
        (Some(story.to_owned()), scenario)
    } else {
        let mut rng = RngComponent::new();
        if let Some(asset) = stories.get(&assets.story) {
            available_cards.cards = cards.available_cards.clone();
            let mut story = Story::generate(&mut rng, asset);
            let scenario = story.generate_next_scenario(None);
            (Some(story.to_owned()), scenario)
        } else {
            (None, None)
        }
    };

    bevy::log::info!("Setup Overworld UI");

     if let Some(story) = &story {
        commands.insert_resource(story.clone());
        if StoryPhase::Complete == story.phase {
            UiRoot::spawn(&mut commands, |parent| {
                MainText::new("The End").size(100.).spawn(parent, &assets);
                MenuButton::Primary.spawn("end", "Back To Menu", parent, &assets);
            });
        } else if let Some(scenario) = &scenario {
            commands.insert_resource(scenario.clone());
            UiRoot::spawn(&mut commands, |parent| {
                MainText::new(&scenario.initial_description)
                    .size(30.)
                    .alignment(JustifyContent::Center)
                    .spawn(parent, &assets);
                MenuButton::Primary.spawn("start_scenario", "Start Scenario", parent, &assets);
            });
        } else {
            UiRoot::spawn(&mut commands, |parent| {
                MainText::new("Error loading scenario....")
                    .size(100.)
                    .spawn(parent, &assets);
                MenuButton::Primary.spawn("end", "Back To Menu", parent, &assets);
            });
        }
    } else {
        UiRoot::spawn(&mut commands, |parent| {
            MainText::new("Story Error...")
                .size(100.)
                .spawn(parent, &assets);
            MenuButton::Primary.spawn("end", "Back To Menu", parent, &assets);
        });
    };
}

fn check_click(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    mut clicked: EventReader<ButtonClickEvent>,
) {
    for click in clicked.iter() {
        info!("I'm here for some reason {:?}", &click);
        let ButtonClickEvent(val, _) = click;
        if val == "start_scenario" {
            info!("start scenario clicked");
            let _ = app_state.set(AppState::Scene);
        } else if val == "end" {
            info!("end clicked");
            let _ = app_state.set(AppState::MainMenu);
            commands.remove_resource::<Story>();
        }
    }
}
