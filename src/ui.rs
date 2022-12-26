use bevy::{
    ecs::schedule::StateData,
    prelude::*,
    text::TextStyle,
    ui::{AlignItems, JustifyContent},
};
use std::fmt::Debug;
use std::hash::Hash;

use crate::assets::Assets;

#[derive(Clone, Component)]
pub struct MainText {
    text: String,
    size: f32,
    alignment: JustifyContent,
}

impl Default for MainText {
    fn default() -> Self {
        Self {
            text: Default::default(),
            size: 30.,
            alignment: JustifyContent::FlexStart,
        }
    }
}

impl MainText {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn size(self, size: f32) -> Self {
        Self { size, ..self }
    }

    pub fn alignment(self, alignment: JustifyContent) -> Self {
        Self { alignment, ..self }
    }

    pub fn spawn(self, parent: &mut ChildBuilder, assets: &Assets) -> Entity {
        let text = self.text.clone();
        let size = self.size;
        let justify_content = self.alignment;

        let style = TextStyle {
            font: assets.font.clone(),
            font_size: size,
            color: Color::rgb(0.9, 0.9, 0.9),
        };
        parent
            .spawn((
                self,
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                for line in text.lines() {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                align_content: AlignContent::FlexStart,
                                justify_content,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            for word in line.split_whitespace() {
                                parent.spawn(
                                    TextBundle::from_section(format!("{word} "), style.clone())
                                        .with_style(Style {
                                            max_size: Size::new(Val::Undefined, Val::Px(size)),
                                            margin: UiRect::all(Val::Px(4.)),
                                            ..Default::default()
                                        }),
                                );
                            }
                        });
                }
            })
            .id()
    }
}

#[derive(Clone, Copy, Component)]
pub enum MenuButton {
    Primary,
}

#[derive(Clone, Component)]
pub struct ButtonName(String);

impl MenuButton {
    pub fn spawn<T: Into<String>, R: Into<String>>(
        &self,
        name: R,
        text: T,
        parent: &mut ChildBuilder,
        assets: &Assets,
    ) -> Entity {
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(20.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: self.main_color().into(),
                    ..Default::default()
                },
                *self,
                ButtonName(name.into()),
            ))
            .with_children(move |parent| {
                parent.spawn(TextBundle::from_section(
                    text,
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 30.,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            })
            .id()
    }

    pub fn main_color(&self) -> Color {
        match self {
            Self::Primary => Color::hex("0392ce").unwrap_or_default(),
        }
    }

    pub fn hover_color(&self) -> Color {
        match self {
            Self::Primary => Color::hex("026d9b").unwrap_or_default(),
        }
    }

    pub fn click_color(&self) -> Color {
        match self {
            Self::Primary => Color::hex("00131b").unwrap_or_default(),
        }
    }
}

pub struct UIPlugin;

#[derive(Debug, Clone)]
pub struct ButtonClickEvent(pub String, pub Entity);

#[derive(Component, Debug)]
pub struct UiRoot;

impl UiRoot {
    pub fn spawn<T: FnMut(&mut ChildBuilder)>(commands: &mut Commands, children: T) -> Entity {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        max_size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                UiRoot,
            ))
            .with_children(children)
            .id()
    }
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonClickEvent>().add_system(hoverable);
    }
}

fn hoverable(
    mut buttons: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &MenuButton,
            &ButtonName,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut click_event: EventWriter<ButtonClickEvent>,
) {
    for (entity, interaction, mut background, button_type, name) in &mut buttons {
        match *interaction {
            Interaction::Hovered => {
                *background = button_type.hover_color().into();
            }
            Interaction::Clicked => {
                *background = button_type.click_color().into();
                info!("Clicked on {} - {:?}", &name.0, &entity);
                click_event.send(ButtonClickEvent(name.0.clone(), entity))
            }
            Interaction::None => {
                *background = button_type.main_color().into();
            }
        }
    }
}

fn clear_ui(mut commands: Commands, query: Query<Entity, With<UiRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn clear_ui_system_set<T: Debug + Clone + Eq + PartialEq + Hash + StateData>(
    t: T,
) -> SystemSet {

    let name = format!("Value: {:?}", &t);
    SystemSet::on_exit(t).with_system(move |mut commands: Commands, query: Query<Entity, With<UiRoot>>| {
        info!("Clearing for - {:?}", name);
        clear_ui(commands, query);
    })
}
