use bevy::prelude::*;

use crate::{constants::*, fonts::Fonts};

#[derive(Component, Eq, PartialEq)]
pub enum NumberInput {
    Increase,
    Decrease,
    Value,
}

#[derive(Bundle)]
pub struct NumberInputBundle {
    node: NodeBundle,
}

impl NumberInputBundle {
    pub fn new() -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Px(30.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    column_gap: Val::Px(20.),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    pub fn hidden(marker: impl Component) -> impl Bundle {
        let mut bundle = Self::new();
        bundle.node.style.display = Display::None;
        (bundle, marker)
    }

    pub fn populate(
        cb: &mut ChildBuilder,
        marker: impl Component + Copy,
        text: &str,
        value: i16,
        fonts: &Fonts,
    ) {
        let text_style = TextStyle {
            font: fonts.poppins_light.clone(),
            font_size: 18.,
            color: WHITE,
        };

        cb.spawn(TextBundle {
            text: Text::from_section(text, text_style.clone()),
            style: Style {
                width: Val::Px(40.),
                ..Default::default()
            },
            ..Default::default()
        });

        cb.spawn((
            marker,
            NumberInput::Value,
            TextBundle {
                text: Text::from_section(value.to_string(), text_style),
                ..Default::default()
            },
        ));

        cb.spawn(NodeBundle {
            style: Style {
                width: Val::Px(20.),
                height: Val::Px(22.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|cb| {
            cb.spawn((
                marker,
                Interaction::None,
                NumberInput::Increase,
                TextBundle {
                    style: Style {
                        width: Val::Px(20.),
                        height: Val::Px(10.),
                        margin: UiRect::bottom(Val::Px(1.)),
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "    +",
                        TextStyle {
                            font: fonts.poppins_light.clone(),
                            font_size: 10.,
                            color: WHITE,
                        },
                    ),
                    background_color: GRAY_BACKGROUND.into(),
                    ..Default::default()
                },
            ));
            cb.spawn((
                marker,
                Interaction::None,
                NumberInput::Decrease,
                TextBundle {
                    style: Style {
                        width: Val::Px(20.),
                        height: Val::Px(10.),
                        margin: UiRect::top(Val::Px(1.)),
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "    -",
                        TextStyle {
                            font: fonts.poppins_light.clone(),
                            font_size: 10.,
                            color: WHITE,
                        },
                    ),
                    background_color: GRAY_BACKGROUND.into(),
                    ..Default::default()
                },
            ));
        });
    }
}
