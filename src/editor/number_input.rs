use bevy::prelude::*;

use crate::{constants::*, fonts::Fonts};

#[derive(Component, Eq, PartialEq)]
#[require(Node)]
pub enum NumberInput {
    Increase,
    Decrease,
    Value,
}

impl NumberInput {
    #[expect(clippy::new_ret_no_self)]
    pub fn new() -> Node {
        Node {
            width: Val::Percent(100.),
            height: Val::Px(30.),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            column_gap: Val::Px(20.),
            ..default()
        }
    }

    pub fn hidden(marker: impl Component) -> impl Bundle {
        let mut node = Self::new();
        node.display = Display::None;
        (node, marker)
    }

    pub fn populate(
        cb: &mut ChildBuilder,
        marker: impl Component + Copy,
        text: &str,
        value: i16,
        fonts: &Fonts,
    ) {
        let font = TextFont::from_font(fonts.poppins_light.clone()).with_font_size(18.);
        let font_small = font.clone().with_font_size(10.);

        cb.spawn((
            Text::new(text),
            TextColor(WHITE),
            font.clone(),
            Node {
                width: Val::Px(60.),
                ..default()
            },
        ));

        cb.spawn((
            marker,
            NumberInput::Value,
            Text::new(value.to_string()),
            TextColor(WHITE),
            font,
        ));

        cb.spawn(Node {
            width: Val::Px(20.),
            height: Val::Px(22.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        })
        .with_children(|cb| {
            cb.spawn((
                marker,
                Interaction::None,
                NumberInput::Increase,
                Text::new("    +"),
                TextColor(WHITE),
                font_small.clone(),
                BackgroundColor(GRAY_BACKGROUND),
                Node {
                    width: Val::Px(20.),
                    height: Val::Px(10.),
                    margin: UiRect::bottom(Val::Px(1.)),
                    align_content: AlignContent::Center,
                    ..default()
                },
            ));
            cb.spawn((
                marker,
                Interaction::None,
                NumberInput::Decrease,
                Text::new("    -"),
                TextColor(WHITE),
                font_small,
                BackgroundColor(GRAY_BACKGROUND),
                Node {
                    width: Val::Px(20.),
                    height: Val::Px(10.),
                    margin: UiRect::top(Val::Px(1.)),
                    align_content: AlignContent::Center,
                    ..default()
                },
            ));
        });
    }
}
