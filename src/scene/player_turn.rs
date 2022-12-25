use crate::{
    assets,
    card::*,
    scene::{SceneState, scenario::*},
    story::{Scenario, ScenarioState},
    ui::*,
};
use bevy::prelude::*;

pub struct PlayerTurnPlugin;

impl Plugin for PlayerTurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::PlayerTurn).with_system(display_playerturn_phase_menu),
        )
        .add_system_set(SystemSet::on_update(SceneState::PlayerTurn).with_system(click_event))
        .add_system_set(clear_ui_system_set(SceneState::PlayerTurn));
    }
}

fn display_playerturn_phase_menu(
    mut commands: Commands,
    assets: Res<assets::Assets>,
    cards: Res<Cards>,
    selected_cards: Res<ActorResources>,
    scenario: Res<Scenario>,
) {
    UiRoot::spawn(&mut commands, |parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: UiRect::bottom(Val::Px(0.)),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::hex("12102D").unwrap_or_default()),
                ..Default::default()
            })
            .with_children(|parent| {
                if let ScenarioState::InProgress(i) = &scenario.state {
                    if let Some(goal) = scenario.goals.get(*i) {
                        MainText::new(&goal.description)
                            .size(15.)
                            .spawn(parent, &assets);
                    }
                }
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            flex_wrap: FlexWrap::Wrap,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        for id in selected_cards
                            .resources
                            .get(&Actor::Player)
                            .unwrap()
                            .hand
                            .iter()
                        {
                            if let Some(card) = cards.cards.get(id) {
                                CardUI::card(card).spawn(parent, &assets);
                            }
                        }
                    });
            });
    });
}

fn click_event(mut events: EventReader<CardClickEvent>, mut sender: EventWriter<CardPlayedEvent>) {
    for event in events.iter() {
        let played = CardPlayedEvent {
            actor: Actor::Player,
            card: event.0.to_string(),
        };

        sender.send(played);
    }
}
