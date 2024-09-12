use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    background::UpdateBackgroundTransform,
    constants::*,
    fonts::Fonts,
    game_object::{spawn_object_of_type, GameObjectAssets, ObjectType, Position},
    level::{Dimensions, InitialPositionAndMetadata},
    timers::{MovementTimer, TemporaryTimer, TransporterTimer},
    Background, GameEvent, SaveLevel,
};

use super::{
    button::Button, number_input::NumberInput, Editor, EditorBundle, Input, SelectedObjectType,
    ToggleEditor,
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
    mut events: EventWriter<GameEvent>,
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
                let event = match (input, number_input) {
                    (Input::Width, NumberInput::Increase) => GameEvent::ChangeWidth(abs_delta),
                    (Input::Width, NumberInput::Decrease) => GameEvent::ChangeWidth(-abs_delta),
                    (Input::Height, NumberInput::Increase) => GameEvent::ChangeHeight(abs_delta),
                    (Input::Height, NumberInput::Decrease) => GameEvent::ChangeHeight(-abs_delta),
                    _ => continue,
                };
                events.send(event);
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

#[allow(clippy::too_many_arguments)]
pub fn spawn_selected_object(
    mut commands: Commands,
    background_query: Query<(Entity, &Transform), With<Background>>,
    objects: Query<(Entity, &Position, &ObjectType)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<GameObjectAssets>,
    buttons: Res<ButtonInput<MouseButton>>,
    dimensions: Res<Dimensions>,
    fonts: Res<Fonts>,
    selected_object_type: Res<SelectedObjectType>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let Some(selected_object_type) = **selected_object_type else {
        return;
    };

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
    let object_type_and_direction = selected_object_type.get_object_type_and_direction();

    for (entity, object_position, existing_object_type) in &objects {
        if *existing_object_type != ObjectType::Player && *object_position != position {
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

    if x < 1 || x > dimensions.width || y < 1 || y > dimensions.height {
        return;
    }

    if let Some((object_type, direction)) = object_type_and_direction {
        let mut background = commands.entity(background);

        background.with_children(|cb| {
            spawn_object_of_type(
                cb,
                &assets,
                &fonts,
                object_type,
                InitialPositionAndMetadata {
                    position,
                    direction: Some(direction),
                    level: Some(1),
                },
            );
        });
    }
}

#[allow(clippy::too_many_arguments)]
pub fn on_toggle_editor(
    _trigger: Trigger<ToggleEditor>,
    mut commands: Commands,
    mut movement_timer: ResMut<MovementTimer>,
    mut selected_object_type: ResMut<SelectedObjectType>,
    mut temporary_timer: ResMut<TemporaryTimer>,
    mut transporter_timer: ResMut<TransporterTimer>,
    editor_query: Query<Entity, With<Editor>>,
    assets: Res<GameObjectAssets>,
    dimensions: Res<Dimensions>,
    fonts: Res<Fonts>,
) {
    if let Ok(editor) = editor_query.get_single() {
        commands.entity(editor).despawn_recursive();
        **selected_object_type = None;

        movement_timer.unpause();
        temporary_timer.unpause();
        transporter_timer.unpause();
    } else {
        commands
            .spawn(EditorBundle::new())
            .with_children(|cb| EditorBundle::populate(cb, &assets, &dimensions, &fonts));

        movement_timer.pause();
        temporary_timer.pause();
        transporter_timer.pause();
    }

    commands.trigger(UpdateBackgroundTransform);
}
