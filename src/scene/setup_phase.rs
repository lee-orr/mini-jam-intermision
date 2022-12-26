use crate::{
    assets,
    card::*,
    scene::{scenario::*, SceneState},
    story::*,
    ui::*,
};
use bevy::prelude::*;

pub struct SetupPhasePlugin;

impl Plugin for SetupPhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::Setup).with_system(display_setup_phase_menu),
        )
        .add_system_set(
            SystemSet::on_update(SceneState::Setup)
                .with_system(card_select)
                .with_system(click_event),
        )
        .add_system_set(clear_ui_system_set(SceneState::Setup));
    }
}

#[derive(Component)]
struct SelectedCounter(usize, usize);

fn display_setup_phase_menu(
    mut commands: Commands,
    scenario: Res<Scenario>,
    assets: Res<assets::Assets>,
    cards: Res<AvailableCards>,
) {
    UiRoot::spawn(&mut commands, |parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::hex("25215e").unwrap_or_default()),
                ..Default::default()
            })
            .with_children(|parent| {
                if let ScenarioState::InProgress(count) = &scenario.state {
                    let current_goal = scenario.goals.get(*count);
                    let previous_goal = if let Some(prev) = count.checked_sub(1) {
                        scenario.goals.get(prev)
                    } else {
                        None
                    };

                    if let Some(current_goal) = current_goal {
                        if let Some(previous_goal) = previous_goal {
                            MainText::new(&previous_goal.success)
                                .size(20.)
                                .spawn(parent, &assets);
                        }
                        MainText::new(&current_goal.description)
                            .size(20.)
                            .spawn(parent, &assets);
                    }
                    MainText::new("Choose your cards:").spawn(parent, &assets);
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
                            for (_, card) in cards.cards.iter() {
                                CardUI::card(card).selectable().spawn(parent, &assets);
                            }
                        });
                    parent.spawn((NodeBundle::default(), SelectedCounter(0, 3)));
                }
            });
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
                MenuButton::Primary.spawn("setup-complete", "Complete Setup", parent, &assets);
            }
        });
    }
}

fn click_event(
    mut events: EventReader<ButtonClickEvent>,
    cards: Query<&CardUI>,
    mut scene_state: ResMut<State<SceneState>>,
    mut actor_resources: ResMut<ActorResources>,
) {
    for event in events.iter() {
        if event.0 == "setup-complete" {
            let selected = cards
                .iter()
                .filter_map(|card| {
                    if card.selected {
                        Some(card.card_id.clone())
                    } else {
                        None
                    }
                })
                .collect();

            actor_resources.resources.insert(
                Actor::Player,
                ActorResource {
                    hand: selected,
                    health: 5,
                    ..Default::default()
                },
            );
            let _ = scene_state.set(SceneState::PlayerTurn);
        }
    }
}
