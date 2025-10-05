mod editor_button;
mod editor_component;
mod editor_system;
mod number_input;
mod object_selector;
mod object_selector_system;

use bevy::prelude::*;
pub use editor_component::*;
pub use editor_system::*;
pub use object_selector::*;
use object_selector_system::*;

use crate::game_object::Position;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                on_dimensions_changed,
                on_editor_button_interaction,
                on_editor_number_input_interaction,
                on_object_selector_input,
                on_selected_object_change,
            ),
        )
        .init_resource::<EditorState>()
        .add_observer(change_height)
        .add_observer(change_identifier)
        .add_observer(change_level)
        .add_observer(change_width)
        .add_observer(move_all_objects)
        .add_observer(on_activate_selection)
        .add_observer(on_deselect_object)
        .add_observer(on_select_object)
        .add_observer(on_toggle_editor)
        .add_observer(on_toggle_selection);
    }
}

#[derive(Clone, Default, Resource)]
pub struct EditorState {
    pub is_open: bool,
    pub selected_object: Option<Position>,
    pub selected_object_type: Option<EditorObjectType>,
    pub selection: SelectionState,
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum SelectionState {
    #[default]
    Disabled,
    WaitingForClick,
    Selecting {
        start: Position,
        current: Position,
    },
    Active {
        top_left: Position,
        bottom_right: Position,
    },
}

#[derive(Event)]
pub struct ChangeHeight(i16);

#[derive(Event)]
pub struct ChangeIdentifier(i16);

#[derive(Event)]
pub struct ChangeLevel(i16);

#[derive(Event)]
pub struct ChangeWidth(i16);

#[derive(Event)]
pub struct DeselectObject;

#[derive(Event)]
pub struct MoveAllObjects {
    dx: i16,
    dy: i16,
}

#[derive(Event)]
pub struct SelectObject(Position);

#[derive(Event)]
pub struct ToggleEditor;

#[derive(Event)]
pub struct ToggleSelection;

#[derive(Event)]
pub struct ActivateSelection {
    start: Position,
    current: Position,
}
