use bevy::{app::AppExit, prelude::*};
use bevy_turborand::rng::Rng;

use crate::{
    assets, game_state::AppState, story::Story, tracery_generator::TraceryGenerator, ui::*,
};

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::Overworld).with_system(display_overworld))
            .add_system_set(SystemSet::on_update(AppState::Overworld).with_system(check_click))
            .add_system_set(clear_ui_system_set(AppState::Overworld));
    }
}

fn display_overworld(
    mut commands: Commands,
    assets: Res<assets::Assets>,
    stories: Res<Assets<TraceryGenerator>>,
) {
    let mut rng = Rng::new();
    let (text, story_exists) = if let Some(asset) = stories.get(&assets.story) {
        let mut story = Story::generate(&mut rng, asset);
        let intro = story.introduce(&mut rng, asset);
        commands.insert_resource(story);
        (intro, true)
    } else {
        ("no story".to_string(), false)
    };
    {
        let text = text;
        UiRoot::spawn(&mut commands, move |parent| {
            parent.spawn(main_text("Content:", 100.0, &assets));
            parent.spawn(main_text(text.clone(), 50.0, &assets));
            if story_exists {
                MenuButton::Primary.spawn("continue", "Continue", parent, &assets);
            } else {
                MenuButton::Primary.spawn("exit", "Exit", parent, &assets);
            }
        });
    }
}

fn check_click(
    _app_state: ResMut<State<AppState>>,
    mut clicked: EventReader<ButtonClickEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for click in clicked.iter() {
        let ButtonClickEvent(val) = click;
        if val == "exit" {
            exit.send(AppExit);
        } else if val == "continue" {
        }
    }
}
