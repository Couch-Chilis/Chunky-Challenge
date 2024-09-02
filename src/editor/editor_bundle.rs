use bevy::prelude::*;

use crate::{constants::*, fonts::Fonts, game_object::GameObjectAssets, level::Dimensions};

use super::{
    button::{Button, EditorButtonBundle},
    number_input::NumberInputBundle,
    ObjectSelectorBundle,
};

const BORDER_WIDTH: f32 = 2.;

#[derive(Component)]
pub struct Editor;

#[derive(Clone, Component, Copy)]
pub enum Input {
    Width,
    Height,
}

#[derive(Bundle)]
pub struct EditorBundle {
    background: NodeBundle,
    editor: Editor,
}

impl EditorBundle {
    pub fn new() -> Self {
        Self {
            background: NodeBundle {
                style: Style {
                    width: Val::Px(EDITOR_WIDTH as f32 - BORDER_WIDTH),
                    height: Val::Percent(100.),
                    border: UiRect::left(Val::Px(BORDER_WIDTH)),
                    padding: UiRect::all(Val::Px(EDITOR_PADDING as f32)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    right: Val::Px(0.),
                    position_type: PositionType::Absolute,
                    row_gap: Val::Px(EDITOR_PADDING as f32),
                    ..Default::default()
                },
                background_color: GRAY_BACKGROUND.into(),
                border_color: RED.into(),
                z_index: ZIndex::Global(100),
                ..Default::default()
            },
            editor: Editor,
        }
    }

    pub fn populate(
        cb: &mut ChildBuilder,
        assets: &GameObjectAssets,
        dimensions: &Dimensions,
        fonts: &Fonts,
    ) {
        cb.spawn(NumberInputBundle::new()).with_children(|cb| {
            NumberInputBundle::populate(cb, Input::Width, "Width:", dimensions.width, fonts)
        });

        cb.spawn(NumberInputBundle::new()).with_children(|cb| {
            NumberInputBundle::populate(cb, Input::Height, "Height:", dimensions.height, fonts)
        });

        cb.spawn(ObjectSelectorBundle::new())
            .with_children(|cb| ObjectSelectorBundle::populate(cb, assets));

        cb.spawn(EditorButtonBundle::new(Button::Save))
            .with_children(|cb| EditorButtonBundle::populate(cb, "Save", fonts));
    }
}
