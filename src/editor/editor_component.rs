use bevy::prelude::*;

use crate::{constants::*, fonts::Fonts, game_object::GameObjectAssets, levels::Dimensions};

use super::{editor_button::EditorButton, number_input::NumberInput, ObjectSelector};

const BORDER_WIDTH: f32 = 2.;

#[derive(Clone, Component, Copy, Eq, PartialEq)]
pub enum Input {
    Width,
    Height,
    Level,
    Identifier,
}

#[derive(Component)]
pub struct IdentifierInput;

#[derive(Component)]
pub struct LevelInput;

#[derive(Component)]
pub struct SelectionOverlay;

#[derive(Component)]
#[require(Node)]
pub struct Editor;

impl Editor {
    #[expect(clippy::new_ret_no_self)]
    pub fn new() -> impl Bundle {
        (
            Editor,
            BackgroundColor(GRAY_BACKGROUND),
            BorderColor::all(RED),
            GlobalZIndex(100),
            Node {
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
                ..default()
            },
        )
    }

    pub fn populate(
        spawner: &mut ChildSpawnerCommands,
        assets: &GameObjectAssets,
        dimensions: &Dimensions,
        fonts: &Fonts,
    ) {
        spawner.spawn(NumberInput::new()).with_children(|cb| {
            NumberInput::populate(cb, Input::Width, "Width:", dimensions.width, fonts)
        });

        spawner.spawn(NumberInput::new()).with_children(|cb| {
            NumberInput::populate(cb, Input::Height, "Height:", dimensions.height, fonts)
        });

        spawner
            .spawn(ObjectSelector::new())
            .with_children(|cb| ObjectSelector::populate(cb, assets));

        spawner
            .spawn(EditorButton::new(EditorButton::Save))
            .with_children(|cb| EditorButton::populate(cb, EditorButton::Save, "Save", fonts));

        spawner
            .spawn(EditorButton::new(EditorButton::Select))
            .with_children(|cb| EditorButton::populate(cb, EditorButton::Select, "Select", fonts));

        spawner
            .spawn(NumberInput::hidden(LevelInput))
            .with_children(|cb| NumberInput::populate(cb, Input::Level, "Level:", 0, fonts));

        spawner
            .spawn(NumberInput::hidden(IdentifierInput))
            .with_children(|cb| {
                NumberInput::populate(cb, Input::Identifier, "Teleporter:", 0, fonts)
            });
    }
}
