use bevy::prelude::*;

use crate::{
    constants::{DARK_GRAY, *},
    fonts::Fonts,
};

#[derive(Clone, Component, Copy, Eq, PartialEq)]
pub enum Button {
    Save,
    Select,
}

#[derive(Bundle)]
pub struct EditorButtonBundle {
    button: ButtonBundle,
}

impl EditorButtonBundle {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(marker: impl Bundle) -> impl Bundle {
        (
            marker,
            Self {
                button: ButtonBundle {
                    background_color: DARK_GRAY.into(),
                    border_radius: BorderRadius::all(Val::Px(4.)),
                    style: Style {
                        height: Val::Px(30.),
                        width: Val::Px(150.),
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
        )
    }

    pub fn populate(
        cb: &mut ChildBuilder,
        marker: impl Bundle,
        text: impl Into<String>,
        fonts: &Fonts,
    ) {
        cb.spawn((
            marker,
            TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: fonts.poppins_light.clone(),
                        font_size: 18.,
                        color: WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    }
}
