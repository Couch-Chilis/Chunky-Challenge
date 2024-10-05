use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    background::UpdateBackgroundTransform,
    constants::*,
    fonts::Fonts,
    game_object::{spawn_object_of_type, GameObjectAssets, ObjectType, Position},
    levels::{Dimensions, InitialPositionAndMetadata},
    timers::{MovementTimer, TemporaryTimer, TransporterTimer},
    Background, ChangeZoom, GameEvent, SaveLevel,
};

use super::{
    button::Button, number_input::NumberInput, ActivateSelection, ChangeHeight, ChangeWidth,
    Editor, EditorBundle, EditorObjectType, EditorState, Input, MoveAllObjects, SelectionOverlay,
    SelectionState, SpawnObject, ToggleEditor, ToggleSelection,
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

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn on_left_click(
    mut commands: Commands,
    selection_query: Query<&mut Transform, With<SelectionOverlay>>,
    background_query: Query<(Entity, &Transform), (With<Background>, Without<SelectionOverlay>)>,
    objects: Query<(Entity, &Position, &ObjectType)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    dimensions: Res<Dimensions>,
    editor_state: ResMut<EditorState>,
) {
    if !buttons.pressed(MouseButton::Left) {
        if let SelectionState::Selecting { start, current } = editor_state.selection {
            commands.trigger(ActivateSelection { start, current });
        }

        return;
    }

    let window = window_query
        .get_single()
        .expect("there should be only one window");
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let window_size = window.size();

    if cursor_position.x >= window_size.x - EDITOR_WIDTH as f32 {
        return;
    }

    let (background, transform) = background_query
        .get_single()
        .expect("there should be only one background");

    let center_x = 0.5 * window_size.x + transform.translation.x;
    let x = ((cursor_position.x - center_x) / (transform.scale.x * GRID_SIZE as f32)
        + 0.5 * dimensions.width as f32) as i16
        + 1;

    let center_y = 0.5 * window_size.y - transform.translation.y;
    let y = ((cursor_position.y - center_y) / (transform.scale.y * GRID_SIZE as f32)
        + 0.5 * dimensions.height as f32) as i16
        + 1;

    let position = Position { x, y };

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
    } else if let Some(selected_object_type) = editor_state.selected_object_type {
        spawn_selected_object(
            commands,
            objects,
            dimensions,
            position,
            selected_object_type,
        );
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
    objects: Query<(Entity, &Position, &ObjectType)>,
    dimensions: Res<Dimensions>,
    position: Position,
    selected_object_type: EditorObjectType,
) {
    let object_type_and_direction = selected_object_type.get_object_type_and_direction();

    for (entity, object_position, existing_object_type) in &objects {
        if *object_position != position {
            continue;
        }

        if object_type_and_direction
            .as_ref()
            .is_some_and(|(object_type, _)| existing_object_type == object_type)
        {
            return; // This object is already there.
        }

        commands.entity(entity).despawn_recursive();
    }

    if position.x < 1
        || position.x > dimensions.width
        || position.y < 1
        || position.y > dimensions.height
    {
        return;
    }

    if let Some((object_type, direction)) = object_type_and_direction {
        commands.trigger(SpawnObject {
            object_type,
            initial_position: InitialPositionAndMetadata {
                position,
                direction: Some(direction),
                identifier: Some(1),
                level: Some(1),
            },
        });
    }
}

pub fn on_editor_keyboard_input(
    mut commands: Commands,
    mut events: EventWriter<GameEvent>,
    mut editor_state: ResMut<EditorState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            ArrowUp => {
                if matches!(editor_state.selection, SelectionState::Active { .. }) {
                    commands.trigger(MoveAllObjects { dx: 0, dy: -1 });
                } else {
                    editor_state.camera_offset.1 -= 1;
                    commands.trigger(UpdateBackgroundTransform);
                }
            }
            ArrowRight => {
                if matches!(editor_state.selection, SelectionState::Active { .. }) {
                    commands.trigger(MoveAllObjects { dx: 1, dy: 0 });
                } else {
                    editor_state.camera_offset.0 += 1;
                    commands.trigger(UpdateBackgroundTransform);
                }
            }
            ArrowDown => {
                if matches!(editor_state.selection, SelectionState::Active { .. }) {
                    commands.trigger(MoveAllObjects { dx: 0, dy: 1 });
                } else {
                    editor_state.camera_offset.1 += 1;
                    commands.trigger(UpdateBackgroundTransform);
                }
            }
            ArrowLeft => {
                if matches!(editor_state.selection, SelectionState::Active { .. }) {
                    commands.trigger(MoveAllObjects { dx: -1, dy: 0 });
                } else {
                    editor_state.camera_offset.0 -= 1;
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
            KeyE | Escape => {
                commands.trigger(ToggleEditor);
            }

            _ => continue,
        };
    }
}

pub fn on_spawn_object(
    trigger: Trigger<SpawnObject>,
    mut commands: Commands,
    background_query: Query<Entity, With<Background>>,
    assets: Res<GameObjectAssets>,
    fonts: Res<Fonts>,
) {
    let SpawnObject {
        object_type,
        initial_position,
    } = trigger.event();

    let background = background_query
        .get_single()
        .expect("there should be only one background");

    commands.entity(background).with_children(|cb| {
        spawn_object_of_type(cb, &assets, &fonts, *object_type, initial_position.clone());
    });
}

#[allow(clippy::too_many_arguments)]
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
