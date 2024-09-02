use bevy::prelude::*;

use crate::constants::*;

use super::{EditorObjectType, SelectedObjectType, SELECTOR_OUTLINE_WIDTH};

pub fn on_object_selector_input(
    mut interaction_query: Query<
        (&Interaction, &EditorObjectType, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut selected_object_type: ResMut<SelectedObjectType>,
) {
    for (interaction, object_type, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = WHITE.into();

                **selected_object_type = Some(*object_type);
            }
            Interaction::Hovered => {
                *color = LIGHT_GRAY.into();
            }
            Interaction::None => {
                *color = NORMAL_GRAY.into();
            }
        }
    }
}

pub fn on_selected_object_change(
    mut commands: Commands,
    mut query: Query<(Entity, &EditorObjectType, Option<&mut Outline>)>,
    selected_object_type: Res<SelectedObjectType>,
) {
    if !selected_object_type.is_changed() {
        return;
    }

    for (entity, object_type, outline) in &mut query {
        let is_selected_object_type = selected_object_type.is_some_and(|ty| ty == *object_type);

        if let Some(mut outline) = outline {
            outline.color = if is_selected_object_type {
                RED
            } else {
                Color::NONE
            };
        } else if is_selected_object_type {
            commands.entity(entity).insert(Outline::new(
                Val::Px(SELECTOR_OUTLINE_WIDTH as f32),
                Val::ZERO,
                RED,
            ));
        }
    }
}
