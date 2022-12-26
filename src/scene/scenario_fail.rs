use crate::assets;
use crate::card::AvailableCards;
use crate::card::CardSelectedEvent;
use crate::card::CardUI;
use crate::card::Cards;
use crate::game_state::AppState;
use crate::ui::*;
use bevy::prelude::*;

use crate::story::ScenarioState;

use crate::story::Scenario;

use super::SceneState;

pub struct FailPhasePlugin;

impl Plugin for FailPhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::Failed).with_system(display_failure_men),
        )
        .add_system_set(
            SystemSet::on_update(SceneState::Failed)
                .with_system(card_select)
                .with_system(click_event),
        )
        .add_system_set(clear_ui_system_set(SceneState::Failed));
    }
}

#[derive(Component)]
struct SelectedCounter(usize, usize);

pub(crate) fn display_failure_men(
    mut commands: Commands,
    assets: Res<assets::Assets>,
    scenario: Res<Scenario>,
    cards: Res<Cards>,
    available_cards: Res<AvailableCards>,
) {
    UiRoot::spawn(&mut commands, |parent| {
        MainText::new("Mission Failed...")
            .size(100.)
            .spawn(parent, &assets);
        if let ScenarioState::Failure(goal) = &scenario.state {
            MainText::new(goal).spawn(parent, &assets);
        }

        let available_cards = {
            let current_cards = available_cards.cards.keys().collect::<Vec<_>>();
            let remaining_cards = cards
                .cards
                .iter()
                .filter(|(key, _)| !current_cards.contains(key))
                .collect::<Vec<_>>();
            let min = remaining_cards
                .iter()
                .fold(usize::MAX, |v, (_, card)| v.min(card.tier));
            remaining_cards
                .iter()
                .filter_map(|(_, card)| {
                    if card.tier > min {
                        None
                    } else {
                        Some((**card).clone())
                    }
                })
                .collect::<Vec<_>>()
        };

        if available_cards.is_empty() {
            MainText::new("You have nothing to learn, and everything to prove.");
            MenuButton::Primary.spawn("continue-story", "Continue Story", parent, &assets);
            return;
        }

        MainText::new(
            "And yet - we all grow through our failures.\nChoose a card to add to your deck:",
        );

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
                for card in available_cards.iter() {
                    CardUI::card(card).selectable().spawn(parent, &assets);
                }
            });
        parent.spawn((NodeBundle::default(), SelectedCounter(0, 1)));
    });
}

fn card_select(
    mut events: EventReader<CardSelectedEvent>,
    mut commands: Commands,
    mut counters: Query<(Entity, &mut SelectedCounter)>,
    assets: Res<assets::Assets>,
) {
    let mut added = 0;
    let mut removed = 0;

    for event in events.iter() {
        if event.0 {
            added += 1;
        } else {
            removed += 1;
        }
    }

    for (entity, mut counter) in counters.iter_mut() {
        counter.0 += added;
        counter.0 = counter.0.checked_sub(removed).unwrap_or_default();

        let selected = counter.0;
        let total = counter.1;

        commands.entity(entity).despawn_descendants();
        commands.entity(entity).add_children(|parent| {
            MainText::new(format!("Selected {selected}/{total} Cards"))
                .size(10.)
                .spawn(parent, &assets);

            if selected == total {
                MenuButton::Primary.spawn(
                    "continue-story-cards",
                    "Continue Story",
                    parent,
                    &assets,
                );
            }
        });
    }
}

fn click_event(
    mut events: EventReader<ButtonClickEvent>,
    cards: Query<&CardUI>,
    card_collection: Res<Cards>,
    mut available_cards: ResMut<AvailableCards>,
    mut app_state: ResMut<State<AppState>>,
    mut scene_state: ResMut<State<SceneState>>,
) {
    for event in events.iter() {
        if event.0 == "continue-story-cards" {
            for card in cards.iter().filter_map(|card| {
                if card.selected {
                    let id = &card.card_id;
                    card_collection.cards.get(id).cloned()
                } else {
                    None
                }
            }) {
                available_cards.cards.insert(card.id.clone(), card);
            }
            let _ = scene_state.set(SceneState::None);
            let _ = app_state.set(AppState::Overworld);
        } else if event.0 == "continue-story" {
            let _ = scene_state.set(SceneState::None);
            let _ = app_state.set(AppState::Overworld);
        }
    }
}
