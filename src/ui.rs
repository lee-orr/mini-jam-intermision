use bevy::{text::TextStyle, prelude::*, ui::{JustifyContent, AlignItems}, ecs::{system::EntityCommands, schedule::StateData}};
use std::hash::Hash;
use std::fmt::Debug;

use crate::assets::Assets;

pub fn main_text<T>(text: T, size: f32, assets: &Assets) -> TextBundle where T: Into<String> {
    TextBundle::from_section(
        text,
        TextStyle { font: assets.font.clone(), font_size: size, color: Color::rgb(0.9, 0.9, 0.9) }
    )    
}

#[derive(Clone, Copy, Component)]
pub enum MenuButton {
    Primary
}

#[derive(Clone, Component)]
pub struct ButtonName(String);

impl MenuButton {
    pub fn spawn<T: Into<String>, R: Into<String>>(&self, name: R, text: T, parent: &mut ChildBuilder, assets: &Assets) -> Entity {
        parent.spawn((ButtonBundle {
            style: Style {
                padding: UiRect::all(Val::Px(20.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: self.main_color().into(),
            ..Default::default()
        }, self.clone(), ButtonName(name.into())))
        .with_children(move |parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle { font: assets.font.clone(), font_size: 30., color: Color::rgb(0.9, 0.9, 0.9) }
            ));
        }).id()
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

pub struct StylePlugin;

pub struct ButtonClickEvent(pub String);

#[derive(Component)]
pub struct UiRoot;

impl UiRoot {
    pub fn spawn<T: FnMut(&mut ChildBuilder) -> ()>(commands: &mut Commands, children: T) -> Entity {
        commands.spawn((NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        }, UiRoot)).with_children(children).id()
    }
}


impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ButtonClickEvent>()
            .add_system(hoverable);
    }
}

fn hoverable(mut buttons: Query<(&Interaction, &mut BackgroundColor, &MenuButton, &ButtonName), (Changed<Interaction>, With<Button>)>, mut click_event: EventWriter<ButtonClickEvent>) {
    for (interaction, mut background, button_type, name) in &mut buttons {
        match *interaction {
            Interaction::Hovered => {
                *background = button_type.hover_color().into();
            }
            Interaction::Clicked => {
                *background = button_type.click_color().into();
                click_event.send(ButtonClickEvent(name.0.clone()))
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

pub fn clear_ui_system_set<T: Debug + Clone + Eq + PartialEq + Hash + StateData>(t: T) -> SystemSet {
    SystemSet::on_exit(t).with_system(clear_ui)
}