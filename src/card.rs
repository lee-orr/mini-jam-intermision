use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};
use bevy_common_assets::yaml::YamlAssetPlugin;
use serde::{Deserialize, Serialize};

use crate::{
    assets::{self},
    game_state::AppState,
};

#[derive(Debug, Clone, Serialize, Deserialize, TypeUuid)]
#[uuid = "4a4c156e-fe23-44b3-be85-0c107d31cc54"]
pub struct Card {
    pub id: String,
    pub name: String,
    pub actions: Vec<CardAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardAction {
    Move(usize),
}

#[derive(Debug, Clone)]
pub enum Targetable {
    Path { max_distance: usize },
}

impl CardAction {
    pub fn describe(&self) -> String {
        match self {
            CardAction::Move(d) => format!("Move {d} squares"),
        }
    }

    pub fn target(&self) -> Targetable {
        match self {
            CardAction::Move(d) => Targetable::Path { max_distance: *d },
        }
    }
}

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CardClickEvent>()
            .add_event::<CardSelectedEvent>()
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_cards))
            .add_system(hoverable)
            .add_plugin(YamlAssetPlugin::<Card>::new(&["card.yaml"]));
    }
}

impl Card {
    pub fn title(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        if self.actions.len() > 0 {
            self.actions.iter().map(|c| c.describe()).collect::<Vec<_>>().join("\n")
        } else {
            format!("Describing {}", self.name)
        }
    }
}

#[derive(Debug, Resource, Default)]
pub struct Cards {
    pub cards: HashMap<String, Card>,
}

fn setup_cards(mut commands: Commands, assets: Res<assets::Assets>, card_asset: Res<Assets<Card>>) {
    let mut cards = HashMap::new();
    for card_handle in assets.cards.iter() {
        if let Some(card) = card_asset.get(card_handle) {
            cards.insert(card.id.clone(), card.clone());
        }
    }

    bevy::log::info!("Cards: {:?}", cards);
    commands.insert_resource(Cards { cards });
}

#[derive(Debug, Clone, Component)]
pub struct CardUI {
    pub card_id: String,
    pub title: String,
    pub description: String,
    pub selected: bool,
    pub selectable: bool,
}

impl CardUI {
    pub fn card(card: &Card) -> Self {
        Self {
            card_id: card.id.clone(),
            title: card.title(),
            description: card.description(),
            selected: false,
            selectable: false,
        }
    }

    pub fn selectable(self) -> Self {
        Self {
            selectable: true,
            ..self
        }
    }

    fn selected_color(&self) -> Color {
        Color::BLUE
    }

    fn main_color(&self) -> Color {
        Color::MIDNIGHT_BLUE
    }

    fn hover_color(&self) -> Color {
        Color::AZURE
    }

    pub fn spawn(self, parent: &mut ChildBuilder, assets: &assets::Assets) -> Entity {
        let title = self.title.clone();
        let description = self.description.clone();
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(10.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    background_color: if self.selected {
                        self.selected_color().into()
                    } else {
                        self.main_color().into()
                    },
                    ..Default::default()
                },
                self,
            ))
            .with_children(move |parent| {
                parent.spawn(TextBundle::from_section(
                    title,
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 20.,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    description,
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 14.,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            })
            .id()
    }
}

pub struct CardClickEvent(pub String, pub Entity);
pub struct CardSelectedEvent(pub bool, pub String, pub Entity);

fn hoverable(
    mut buttons: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut CardUI),
        (Changed<Interaction>, With<Button>),
    >,
    mut click_event: EventWriter<CardClickEvent>,
    mut select_event: EventWriter<CardSelectedEvent>,
) {
    for (entity, interaction, mut background, mut card_ui) in &mut buttons {
        match *interaction {
            Interaction::Hovered => {
                *background = card_ui.hover_color().into();
            }
            Interaction::Clicked => {
                *background = card_ui.hover_color().into();
                if card_ui.selectable {
                    card_ui.selected = !card_ui.selected;
                    select_event.send(CardSelectedEvent(
                        card_ui.selected,
                        card_ui.card_id.clone(),
                        entity,
                    ))
                } else {
                    click_event.send(CardClickEvent(card_ui.card_id.clone(), entity))
                }
            }
            Interaction::None => {
                *background = if card_ui.selected {
                    card_ui.selected_color().into()
                } else {
                    card_ui.main_color().into()
                };
            }
        }
    }
}
