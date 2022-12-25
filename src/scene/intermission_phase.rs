use crate::{
    assets,
    card::*,
    scene::{scenario::*, SceneState},
    story::*,
    ui::*,
};
use bevy::prelude::*;

pub struct IntermissionPhasePlugin;

impl Plugin for IntermissionPhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::Intermission)
                .with_system(display_intermission_phase_menu),
        )
        .add_system_set(
            SystemSet::on_update(SceneState::Intermission)
                .with_system(card_select)
                .with_system(click_event),
        )
        .add_system_set(clear_ui_system_set(SceneState::Intermission));
    }
}

#[derive(Component)]
struct SelectedCounter(usize, usize);

fn display_intermission_phase_menu(
    mut commands: Commands,
    scenario: Res<Scenario>,
    assets: Res<assets::Assets>,
    actor_resources: Res<ActorResources>,
    cards: Res<Cards>,
) {
    info!("Displaying Intermission");
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
                    MainText::new("Intermission!")
                        .size(50.)
                        .spawn(parent, &assets);
                    let current_goal = scenario.goals.get(*count);
                    let previous_goal = if let Some(prev) = count.checked_sub(1) {
                        scenario.goals.get(prev)
                    } else {
                        None
                    };

                    if let Some(_current_goal) = current_goal {
                        if let Some(previous_goal) = previous_goal {
                            MainText::new(&previous_goal.success)
                                .size(20.)
                                .spawn(parent, &assets);
                        }
                    }
                    MainText::new("Choose one card to add to your hand:").spawn(parent, &assets);
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
                            let used = if let Some(player_resource) =
                                actor_resources.resources.get(&Actor::Player)
                            {
                                let mut hand = player_resource.hand.clone();
                                let mut used = player_resource.used.clone();
                                hand.append(&mut used);
                                hand
                            } else {
                                vec![]
                            };
                            for (id, card) in cards.cards.iter() {
                                if used.contains(id) {
                                    continue;
                                }
                                CardUI::card(card).selectable().spawn(parent, &assets);
                            }
                        });
                    parent.spawn((NodeBundle::default(), SelectedCounter(0, 1)));
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
                MenuButton::Primary.spawn(
                    "intermission-complete",
                    "Continue Mission",
                    parent,
                    &assets,
                );
            }
        });
    }
}

fn click_event(
    mut commands: Commands,
    mut events: EventReader<ButtonClickEvent>,
    cards: Query<&CardUI>,
    mut scene_state: ResMut<State<SceneState>>,
    mut actor_resources: ResMut<ActorResources>,
    mut animate: EventWriter<AnimateActionsEvents>,
) {
    for event in events.iter() {
        if event.0 == "intermission-complete" {
            let mut selected = cards
                .iter()
                .filter_map(|card| {
                    if card.selected {
                        Some(card.card_id.clone())
                    } else {
                        None
                    }
                })
                .collect();

            if let Some(player_resource) = actor_resources.resources.get_mut(&Actor::Player) {
                player_resource.hand.append(&mut player_resource.used);
                player_resource.hand.append(&mut selected);
            }
            let _ = scene_state.set(SceneState::Processing);
            commands.insert_resource(CurrentTurnProcess::Thinking(Actor::Player));
            animate.send(AnimateActionsEvents::Wait(0.2));
            animate.send(AnimateActionsEvents::Continue(Actor::Player));
        }
    }
}
