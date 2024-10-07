use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    background::UpdateBackgroundTransform,
    constants::*,
    fonts::Fonts,
    game_object::{Entrance, GameObjectAssets, ObjectType, Position, Teleporter},
    levels::{Dimensions, InitialPositionAndMetadata},
    timers::{MovementTimer, TemporaryTimer, TransporterTimer},
    ui_state::UiState,
    utils::level_coords_from_pointer_coords,
    Background, ChangeZoom, GameEvent, SaveLevel, SpawnObject,
};

use super::{
    button::Button, number_input::NumberInput, ActivateSelection, ChangeHeight, ChangeIdentifier,
    ChangeLevel, ChangeWidth, DeselectObject, Editor, EditorBundle, EditorObjectType, EditorState,
    IdentifierInput, Input, LevelInput, MoveAllObjects, SelectObject, SelectionOverlay,
    SelectionState, ToggleEditor, ToggleSelection,
};

pub fn on_editor_button_interaction(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &Button, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = WHITE.into();
                match button {
                    Button::Save => commands.trigger(SaveLevel {
                        save_to_disk: true,
                        next_level: None,
                    }),
                    Button::Select => commands.trigger(ToggleSelection),
                }
            }
            Interaction::Hovered => {
                *color = LIGHT_GRAY.into();
            }
            Interaction::None => {
                *color = GRAY_BACKGROUND.into();
            }
        }
    }
}

pub fn on_editor_number_input_interaction(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &Input, &NumberInput, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (interaction, input, number_input, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = WHITE.into();

                let abs_delta = if keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
                    2
                } else {
                    1
                };
                match (input, number_input) {
                    (Input::Width, NumberInput::Increase) => {
                        commands.trigger(ChangeWidth(abs_delta))
                    }
                    (Input::Width, NumberInput::Decrease) => {
                        commands.trigger(ChangeWidth(-abs_delta))
                    }
                    (Input::Height, NumberInput::Increase) => {
                        commands.trigger(ChangeHeight(abs_delta))
                    }
                    (Input::Height, NumberInput::Decrease) => {
                        commands.trigger(ChangeHeight(-abs_delta))
                    }
                    (Input::Identifier, NumberInput::Increase) => {
                        commands.trigger(ChangeIdentifier(abs_delta))
                    }
                    (Input::Identifier, NumberInput::Decrease) => {
                        commands.trigger(ChangeIdentifier(-abs_delta))
                    }
                    (Input::Level, NumberInput::Increase) => {
                        commands.trigger(ChangeLevel(abs_delta))
                    }
                    (Input::Level, NumberInput::Decrease) => {
                        commands.trigger(ChangeLevel(-abs_delta))
                    }
                    _ => continue,
                }
            }
            Interaction::Hovered => {
                *color = LIGHT_GRAY.into();
            }
            Interaction::None => {
                *color = GRAY_BACKGROUND.into();
            }
        }
    }
}

pub fn on_dimensions_changed(
    mut input_query: Query<(&Input, &NumberInput, &mut Text)>,
    dimensions: Res<Dimensions>,
) {
    if !dimensions.is_changed() {
        return;
    }

    for (input, number_input, mut text) in &mut input_query {
        match (input, number_input) {
            (Input::Width, NumberInput::Value) => {
                text.sections[0].value = dimensions.width.to_string()
            }
            (Input::Height, NumberInput::Value) => {
                text.sections[0].value = dimensions.height.to_string()
            }
            _ => continue,
        }
    }
}

#[expect(clippy::too_many_arguments, clippy::type_complexity)]
pub fn on_editor_mouse_input(
    mut commands: Commands,
    selection_query: Query<&mut Transform, With<SelectionOverlay>>,
    background_query: Query<(Entity, &Transform), (With<Background>, Without<SelectionOverlay>)>,
    objects: Query<(Entity, &ObjectType, &Position)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    editor_state: ResMut<EditorState>,
    buttons: Res<ButtonInput<MouseButton>>,
    dimensions: Res<Dimensions>,
) {
    if !editor_state.is_open {
        return;
    }

    if !buttons.pressed(MouseButton::Left) {
        if let SelectionState::Selecting { start, current } = editor_state.selection {
            commands.trigger(ActivateSelection { start, current });
        }

        return;
    }

    let window = window_query.single();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let window_size = window.size();

    if cursor_position.x >= window_size.x - EDITOR_WIDTH as f32 {
        return;
    }

    let (background, transform) = background_query.single();

    let (x, y) =
        level_coords_from_pointer_coords(cursor_position, *dimensions, transform, window_size);
    let position: Position = (x as i16, y as i16).into();

    if matches!(
        editor_state.selection,
        SelectionState::WaitingForClick | SelectionState::Selecting { .. }
    ) {
        extend_selection(
            commands,
            selection_query,
            editor_state,
            dimensions,
            background,
            position,
        );
    } else if editor_state.selected_object_type.is_some() {
        spawn_selected_object(commands, editor_state, objects, dimensions, position);
    } else if dimensions.contains(position) {
        commands.trigger(SelectObject(position));
    }
}

fn extend_selection(
    mut commands: Commands,
    mut selection_query: Query<&mut Transform, With<SelectionOverlay>>,
    mut editor_state: ResMut<EditorState>,
    dimensions: Res<Dimensions>,
    background: Entity,
    current: Position,
) {
    let start = match editor_state.selection {
        SelectionState::Selecting { start, .. } => start,
        _ => current,
    };
    editor_state.selection = SelectionState::Selecting { start, current };

    let grid_size = GRID_SIZE as f32;
    let half_grid_size = HALF_GRID_SIZE as f32;

    let center_x = (dimensions.width * HALF_GRID_SIZE) as f32;
    let center_y = (dimensions.height * HALF_GRID_SIZE) as f32;

    let min_x = start.x.min(current.x) as f32;
    let min_y = start.y.min(current.y) as f32;
    let max_x = start.x.max(current.x) as f32;
    let max_y = start.y.max(current.y) as f32;

    let translation = Vec3::new(
        -center_x + (min_x + 0.5 * (max_x - min_x)) * grid_size - half_grid_size,
        center_y - (min_y + 0.5 * (max_y - min_y)) * grid_size + half_grid_size,
        99.,
    );

    let scale = Vec3::new(
        (max_x + 1. - min_x) * grid_size,
        (max_y + 1. - min_y) * grid_size,
        1.,
    );

    if let Ok(mut selection_transform) = selection_query.get_single_mut() {
        selection_transform.translation = translation;
        selection_transform.scale = scale;
    } else {
        commands.entity(background).with_children(|cb| {
            cb.spawn((
                SelectionOverlay,
                SpriteBundle {
                    sprite: Sprite {
                        color: BLUE.with_alpha(0.3),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(translation).with_scale(scale),
                    ..Default::default()
                },
            ));
        });
    }
}

fn spawn_selected_object(
    mut commands: Commands,
    mut editor_state: ResMut<EditorState>,
    objects: Query<(Entity, &ObjectType, &Position)>,
    dimensions: Res<Dimensions>,
    position: Position,
) {
    let (object_type, direction) = match editor_state
        .selected_object_type
        .and_then(EditorObjectType::get_object_type_and_direction)
    {
        Some((object_type, direction)) => (Some(object_type), Some(direction)),
        None => (None, None),
    };

    for (entity, existing_object_type, object_position) in &objects {
        if object_type == Some(ObjectType::Player) && *existing_object_type == ObjectType::Player
            || *object_position == position
        {
            commands.entity(entity).despawn_recursive();
        }
    }

    if !dimensions.contains(position) {
        return;
    }

    if let Some(object_type) = object_type {
        commands.trigger(SpawnObject {
            object_type,
            position: InitialPositionAndMetadata {
                position,
                direction,
                identifier: Some(1),
                level: Some(1),
            },
        });

        if matches!(object_type, ObjectType::Entrance | ObjectType::Teleporter) {
            commands.trigger(SelectObject(position));
            editor_state.selected_object_type = None;
        }
    }
}

pub fn on_editor_keyboard_input(
    mut commands: Commands,
    mut events: EventWriter<GameEvent>,
    mut editor_state: ResMut<EditorState>,
    mut ui_state: ResMut<UiState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            ArrowUp => {
                if matches!(editor_state.selection, SelectionState::Active { .. }) {
                    commands.trigger(MoveAllObjects { dx: 0, dy: -1 });
                } else {
                    ui_state.camera_offset.1 -= 1.;
                    commands.trigger(UpdateBackgroundTransform);
                }
            }
            ArrowRight => {
                if matches!(editor_state.selection, SelectionState::Active { .. }) {
                    commands.trigger(MoveAllObjects { dx: 1, dy: 0 });
                } else {
                    ui_state.camera_offset.0 += 1.;
                    commands.trigger(UpdateBackgroundTransform);
                }
            }
            ArrowDown => {
                if matches!(editor_state.selection, SelectionState::Active { .. }) {
                    commands.trigger(MoveAllObjects { dx: 0, dy: 1 });
                } else {
                    ui_state.camera_offset.1 += 1.;
                    commands.trigger(UpdateBackgroundTransform);
                }
            }
            ArrowLeft => {
                if matches!(editor_state.selection, SelectionState::Active { .. }) {
                    commands.trigger(MoveAllObjects { dx: -1, dy: 0 });
                } else {
                    ui_state.camera_offset.0 -= 1.;
                    commands.trigger(UpdateBackgroundTransform);
                }
            }
            Equal => {
                commands.trigger(ChangeZoom(1.25));
            }
            Minus => {
                commands.trigger(ChangeZoom(0.8));
            }
            KeyR => {
                events.send(GameEvent::LoadRelativeLevel(0));
            }
            KeyE => {
                commands.trigger(ToggleEditor);
            }
            Escape => {
                if !matches!(editor_state.selection, SelectionState::Disabled) {
                    editor_state.selection = SelectionState::Disabled;
                } else if editor_state.selected_object_type.is_some() {
                    editor_state.selected_object_type = None;
                } else {
                    commands.trigger(ToggleEditor);
                }
            }

            _ => continue,
        };
    }
}

pub fn on_select_object(
    trigger: Trigger<SelectObject>,
    mut commands: Commands,
    mut identifier_input_query: Query<&mut Style, (With<IdentifierInput>, Without<LevelInput>)>,
    mut level_input_query: Query<&mut Style, With<LevelInput>>,
    mut input_query: Query<(&Input, &NumberInput, &mut Text)>,
    objects: Query<(&Position, Option<&Entrance>, Option<&Teleporter>)>,
    mut editor_state: ResMut<EditorState>,
) {
    let selected_position = trigger.event().0;

    let Some((_, entrance, teleporter)) = objects
        .iter()
        .find(|(position, ..)| **position == selected_position)
    else {
        commands.trigger(DeselectObject);
        return;
    };

    editor_state.selected_object = Some(selected_position);

    let (input_to_update, value) = if let Some(entrance) = entrance {
        level_input_query.single_mut().display = Display::Flex;
        (Input::Level, entrance.0)
    } else if let Some(teleporter) = teleporter {
        identifier_input_query.single_mut().display = Display::Flex;
        (Input::Identifier, teleporter.0)
    } else {
        return;
    };

    for (input, number_input, mut text) in &mut input_query {
        if *input == input_to_update && *number_input == NumberInput::Value {
            text.sections[0].value = value.to_string();
        }
    }
}

pub fn on_deselect_object(
    _trigger: Trigger<DeselectObject>,
    mut identifier_input_query: Query<&mut Style, (With<IdentifierInput>, Without<LevelInput>)>,
    mut level_input_query: Query<&mut Style, With<LevelInput>>,
    mut editor_state: ResMut<EditorState>,
) {
    editor_state.selected_object = None;

    level_input_query.single_mut().display = Display::None;
    identifier_input_query.single_mut().display = Display::None;
}

#[expect(clippy::too_many_arguments)]
pub fn on_toggle_editor(
    _trigger: Trigger<ToggleEditor>,
    mut commands: Commands,
    mut selection_query: Query<Entity, With<SelectionOverlay>>,
    mut editor_state: ResMut<EditorState>,
    mut movement_timer: ResMut<MovementTimer>,
    mut temporary_timer: ResMut<TemporaryTimer>,
    mut transporter_timer: ResMut<TransporterTimer>,
    editor_query: Query<Entity, With<Editor>>,
    assets: Res<GameObjectAssets>,
    dimensions: Res<Dimensions>,
    fonts: Res<Fonts>,
) {
    if let Ok(editor) = editor_query.get_single() {
        *editor_state = EditorState::default();

        commands.entity(editor).despawn_recursive();

        for selection in &mut selection_query {
            commands.entity(selection).despawn();
        }

        movement_timer.unpause();
        temporary_timer.unpause();
        transporter_timer.unpause();
    } else {
        editor_state.is_open = true;

        commands
            .spawn(EditorBundle::new())
            .with_children(|cb| EditorBundle::populate(cb, &assets, &dimensions, &fonts));

        movement_timer.pause();
        temporary_timer.pause();
        transporter_timer.pause();
    }

    commands.trigger(UpdateBackgroundTransform);
}

pub fn on_toggle_selection(
    _trigger: Trigger<ToggleSelection>,
    mut commands: Commands,
    mut selection_query: Query<Entity, With<SelectionOverlay>>,
    mut button_query: Query<(&Button, &mut Text)>,
    mut editor_state: ResMut<EditorState>,
) {
    editor_state.selection = if editor_state.selection == SelectionState::Disabled {
        SelectionState::WaitingForClick
    } else {
        SelectionState::Disabled
    };

    commands.trigger(DeselectObject);

    for selection in &mut selection_query {
        commands.entity(selection).despawn();
    }

    for (button, mut text) in &mut button_query {
        if button == &Button::Select {
            text.sections[0] = TextSection::new(
                if editor_state.selection == SelectionState::WaitingForClick {
                    "Cancel Selection"
                } else {
                    "Select"
                },
                text.sections[0].style.clone(),
            );
        }
    }
}

pub fn on_activate_selection(
    trigger: Trigger<ActivateSelection>,
    mut button_query: Query<(&Button, &mut Text)>,
    mut editor_state: ResMut<EditorState>,
) {
    let ActivateSelection { start, current } = trigger.event();
    let top_left = Position {
        x: start.x.min(current.x),
        y: start.y.min(current.y),
    };
    let bottom_right = Position {
        x: start.x.max(current.x),
        y: start.y.max(current.y),
    };
    editor_state.selection = SelectionState::Active {
        top_left,
        bottom_right,
    };

    for (button, mut text) in &mut button_query {
        if button == &Button::Select {
            text.sections[0] = TextSection::new("Clear Selection", text.sections[0].style.clone());
        }
    }
}

pub fn change_height(
    trigger: Trigger<ChangeHeight>,
    mut commands: Commands,
    mut dimensions: ResMut<Dimensions>,
) {
    let ChangeHeight(delta) = trigger.event();

    if dimensions.height + delta > 0 {
        dimensions.height += delta;

        if delta.abs() > 1 {
            commands.trigger(MoveAllObjects {
                dx: 0,
                dy: delta / 2,
            });
        }
    }
}

pub fn change_identifier(
    trigger: Trigger<ChangeIdentifier>,
    mut commands: Commands,
    mut teleporters: Query<(&Position, &mut Teleporter)>,
    mut input_query: Query<(&Input, &NumberInput, &mut Text)>,
    editor_state: Res<EditorState>,
) {
    let ChangeIdentifier(delta) = trigger.event();

    let Some((_, mut teleporter)) = editor_state.selected_object.and_then(|selected_position| {
        teleporters
            .iter_mut()
            .find(|(position, ..)| **position == selected_position)
    }) else {
        commands.trigger(DeselectObject);
        return;
    };

    teleporter.0 = teleporter.0.saturating_add_signed(*delta);

    for (input, number_input, mut text) in &mut input_query {
        if *input == Input::Identifier && *number_input == NumberInput::Value {
            text.sections[0].value = teleporter.0.to_string();
        }
    }
}

pub fn change_level(
    trigger: Trigger<ChangeLevel>,
    mut commands: Commands,
    mut entrances: Query<(Entity, &Position, &mut Entrance)>,
    mut input_query: Query<(&Input, &NumberInput, &mut Text)>,
    editor_state: Res<EditorState>,
) {
    let ChangeLevel(delta) = trigger.event();

    let Some((entity, position, mut entrance)) =
        editor_state.selected_object.and_then(|selected_position| {
            entrances
                .iter_mut()
                .find(|(_, position, ..)| **position == selected_position)
        })
    else {
        commands.trigger(DeselectObject);
        return;
    };

    entrance.0 = entrance.0.saturating_add_signed(*delta);

    // Respawn to update the entrance text:
    commands.entity(entity).despawn_recursive();
    commands.trigger(SpawnObject {
        object_type: ObjectType::Entrance,
        position: InitialPositionAndMetadata {
            position: *position,
            direction: None,
            identifier: None,
            level: Some(entrance.0),
        },
    });

    for (input, number_input, mut text) in &mut input_query {
        if *input == Input::Level && *number_input == NumberInput::Value {
            text.sections[0].value = entrance.0.to_string();
        }
    }
}

pub fn change_width(
    trigger: Trigger<ChangeWidth>,
    mut commands: Commands,
    mut dimensions: ResMut<Dimensions>,
) {
    let ChangeWidth(delta) = trigger.event();

    if dimensions.width + delta > 0 {
        dimensions.width += delta;

        if delta.abs() > 1 {
            commands.trigger(MoveAllObjects {
                dx: delta / 2,
                dy: 0,
            });
        }
    }
}

pub fn move_all_objects(
    trigger: Trigger<MoveAllObjects>,
    mut query: Query<&mut Position>,
    mut editor_state: ResMut<EditorState>,
) {
    let MoveAllObjects { dx, dy } = trigger.event();

    for mut position in &mut query {
        if let SelectionState::Active {
            top_left,
            bottom_right,
        } = editor_state.selection
        {
            if position.x < top_left.x
                || position.x > bottom_right.x
                || position.y < top_left.y
                || position.y > bottom_right.y
            {
                continue;
            }
        }

        position.x += *dx;
        position.y += *dy;
    }

    if let SelectionState::Active {
        top_left,
        bottom_right,
    } = &mut editor_state.selection
    {
        top_left.x += dx;
        top_left.y += dy;
        bottom_right.x += dx;
        bottom_right.y += dy;
    }
}
