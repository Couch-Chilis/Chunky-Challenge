use bevy::prelude::*;

use crate::{
    constants::{DARK_GRAY, *},
    ui::Fonts,
};

#[derive(Clone, Component, Copy, Eq, PartialEq)]
pub enum EditorButton {
    Save,
    Select,
}

impl EditorButton {
    #[expect(clippy::new_ret_no_self)]
    pub fn new(marker: impl Bundle) -> impl Bundle {
        (
            marker,
            Button,
            BackgroundColor(DARK_GRAY),
            BorderRadius::all(Val::Px(4.)),
            Node {
                height: Val::Px(30.),
                width: Val::Px(150.),
                align_content: AlignContent::Center,
                ..Default::default()
            },
        )
    }

    pub fn populate(
        spawner: &mut ChildSpawnerCommands,
        marker: impl Bundle,
        text: impl Into<String>,
        fonts: &Fonts,
    ) {
        spawner.spawn((
            marker,
            Text::new(text),
            TextColor(WHITE),
            TextFont::from(fonts.poppins_light.clone()).with_font_size(18.),
            Node {
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
        ));
    }
}
