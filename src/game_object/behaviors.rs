use std::{cmp::Ordering, collections::BTreeSet};

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    editor::EditorState,
    game_object::Pushable,
    game_state::GameState,
    levels::{Dimensions, InitialPositionAndMetadata},
    timers::{AnimationTimer, MovementTimer, TemporaryTimer, TransporterTimer},
    GameEvent, PressedTriggers, SaveLevel, SpawnObject,
};

use super::{
    components::{Animatable, Direction, Trigger, *},
    ObjectType,
};

pub fn animate_objects(
    mut timer: ResMut<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<(&Animatable, &mut TextureAtlas)>,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        for (animatable, mut atlas) in &mut query {
            atlas.index = thread_rng().gen_range(0..animatable.num_frames);
        }
    }
}

pub fn check_for_deadly(
    mut commands: Commands,
    deadly_query: Query<(Entity, &Position), With<Deadly>>,
    player_query: Query<(Entity, &Position), With<Player>>,
) {
    for (player, player_position) in &player_query {
        for (deadly, deadly_position) in &deadly_query {
            if player_position == deadly_position {
                commands.entity(player).despawn();
                commands.entity(deadly).despawn();
                commands.trigger(SpawnObject {
                    object_type: ObjectType::Grave,
                    position: player_position.into(),
                });
            }
        }
    }
}

pub fn check_for_entrance(
    mut commands: Commands,
    player_query: Query<Ref<Position>, With<Player>>,
    entrance_query: Query<(&Entrance, &Position)>,
) {
    for player_position in &player_query {
        if player_position.is_added() || !player_position.is_changed() {
            continue;
        }

        for (entrance, entrance_position) in &entrance_query {
            if player_position.as_ref() == entrance_position {
                commands.trigger(SaveLevel {
                    save_to_disk: false,
                    next_level: Some(entrance.0),
                });
                return;
            }
        }
    }
}

pub fn check_for_exit(
    player_query: Query<Ref<Position>, With<Player>>,
    exit_query: Query<&Position, With<Exit>>,
    mut level_events: EventWriter<GameEvent>,
    mut game_state: ResMut<GameState>,
) {
    for player_position in &player_query {
        if player_position.is_added() || !player_position.is_changed() {
            continue;
        }

        for exit_position in &exit_query {
            if player_position.as_ref() == exit_position {
                let finished_level = game_state.current_level;
                game_state.finished_levels.insert(finished_level);
                level_events.send(GameEvent::LoadLevel(0));
                return;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn check_for_explosive(
    mut commands: Commands,
    explosive_query: Query<(Entity, &Position), With<Explosive>>,
    moved_objects_query: Query<(Entity, &Position), (Changed<Position>, Without<Explosive>)>,
    mut temporary_timer: ResMut<TemporaryTimer>,
) {
    for (object, position) in &moved_objects_query {
        for (explosive, explosive_position, ..) in &explosive_query {
            if explosive_position == position {
                commands.entity(explosive).despawn();
                commands.entity(object).despawn();
                commands.trigger(SpawnObject {
                    object_type: ObjectType::Explosion,
                    position: position.into(),
                });
                if temporary_timer.finished() {
                    temporary_timer.reset();
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn check_for_finished_levels(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        Option<&Entrance>,
        Option<&Openable>,
        Option<&Massive>,
        Option<&mut TextureAtlas>,
    )>,
    game_state: Res<GameState>,
) {
    if !game_state.is_changed() {
        return;
    }

    for (entity, entrance, openable, massive, atlas) in &mut query {
        if let Some(entrance) = entrance {
            if game_state.finished_levels.contains(&entrance.0) {
                if let Some(mut atlas) = atlas {
                    atlas.index = 1;
                }
            }
        } else if let Some(Openable::LevelFinished(level)) = openable {
            let opened = game_state.finished_levels.contains(level);
            if opened && massive.is_some() {
                commands.entity(entity).remove::<Massive>();

                if let Some(mut atlas) = atlas {
                    atlas.index = 1;
                }
            } else if !opened && massive.is_none() {
                commands.entity(entity).insert(Massive);

                if let Some(mut atlas) = atlas {
                    atlas.index = 0;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn check_for_key(
    mut commands: Commands,
    mut openable_query: Query<(Entity, &Position, &Openable, Option<&mut TextureAtlas>)>,
    moved_keys_query: Query<(Entity, &Position), (Changed<Position>, With<Key>)>,
) {
    for (key_entity, key_position) in &moved_keys_query {
        for (openable_entity, openable_position, openable, atlas) in &mut openable_query {
            if matches!(openable, Openable::Key) && key_position == openable_position {
                commands.entity(key_entity).despawn();
                commands.entity(openable_entity).remove::<Massive>();

                if let Some(mut atlas) = atlas {
                    atlas.index = 1;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn check_for_liquid(
    mut commands: Commands,
    liquid_query: Query<&Position, With<Liquid>>,
    moved_objects_query: Query<
        (Entity, &Position, Option<&Floatable>),
        (Changed<Position>, Without<Liquid>),
    >,
    floatable_objects_query: Query<(Entity, &Position), With<Floatable>>,
    mut temporary_timer: ResMut<TemporaryTimer>,
) {
    for (object, position, floatable) in &moved_objects_query {
        for liquid_position in &liquid_query {
            if liquid_position == position {
                if floatable.is_some() {
                    if !floatable_objects_query
                        .iter()
                        .any(|(other, other_position)| {
                            other != object && other_position == position
                        })
                    {
                        let mut object = commands.entity(object);
                        object.remove::<Pushable>();
                    }
                } else if !floatable_objects_query
                    .iter()
                    .any(|(_, other_position)| other_position == position)
                {
                    commands.entity(object).despawn();
                    commands.trigger(SpawnObject {
                        object_type: ObjectType::Splash,
                        position: position.into(),
                    });
                    if temporary_timer.finished() {
                        temporary_timer.reset();
                    }
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn check_for_mixables(
    mut commands: Commands,
    moved_mixables_query: Query<(Entity, &Position, &Mixable), Changed<Position>>,
    all_mixables_query: Query<(Entity, &Position, &Mixable)>,
) {
    for (moved_mixable_entity, mixable_position, mixable) in &moved_mixables_query {
        for (other_mixable_entity, other_position, other_mixable) in &all_mixables_query {
            if moved_mixable_entity != other_mixable_entity
                && mixable_position == other_position
                && mixable == other_mixable
            {
                commands.entity(moved_mixable_entity).despawn();
                commands.entity(other_mixable_entity).despawn();
                commands.trigger(SpawnObject {
                    object_type: mixable.0,
                    position: other_position.into(),
                });
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn check_for_paint(
    mut commands: Commands,
    moved_paint_query: Query<(Entity, &Position, &Paint), Changed<Position>>,
    paintable_query: Query<(Entity, &Position), With<Paintable>>,
) {
    for (paint_entity, paint_position, paint) in &moved_paint_query {
        for (paintable_entity, paintable_position) in &paintable_query {
            if paint_position == paintable_position {
                commands.entity(paint_entity).despawn();
                commands.entity(paintable_entity).despawn();
                commands.trigger(SpawnObject {
                    object_type: paint.0,
                    position: paintable_position.into(),
                });
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn check_for_transform_on_push(
    mut commands: Commands,
    transform_query: Query<
        (Entity, Option<&Direction>, Ref<Position>, &TransformOnPush),
        With<Pushable>,
    >,
    editor_state: Res<EditorState>,
) {
    if editor_state.is_open {
        return;
    }

    for (entity, direction, position, TransformOnPush(object_type)) in &transform_query {
        if position.is_changed() && !position.is_added() {
            commands.entity(entity).despawn();
            commands.trigger(SpawnObject {
                object_type: *object_type,
                position: InitialPositionAndMetadata {
                    position: *position,
                    direction: direction.copied(),
                    identifier: None,
                    level: None,
                },
            });
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn check_for_slippery_and_transporter(
    mut slippery_query: Query<
        (&Position, &mut BlocksMovement),
        (With<Slippery>, Without<Transporter>),
    >,
    mut transporter_query: Query<
        (&Position, &Direction, &mut BlocksMovement),
        (With<Transporter>, Without<Slippery>),
    >,
    mut potential_transportees_query: Query<
        (Entity, Option<&Direction>, CollisionObject),
        (Without<Slippery>, Without<Transporter>),
    >,
    mut timer: ResMut<TransporterTimer>,
    dimensions: Res<Dimensions>,
    time: Res<Time>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    let mut already_moved = BTreeSet::new();
    for (slippery_position, mut blocks_movement) in &mut slippery_query {
        let (mut transportees, collision_objects): (Vec<_>, Vec<_>) = potential_transportees_query
            .iter_mut()
            .partition(|(.., (position, ..))| position.as_ref() == slippery_position);
        transportees.retain(|(entity, ..)| !already_moved.contains(entity));

        if let Some((transportee, transportee_direction, .., (position, ..))) =
            transportees.first_mut()
        {
            let Some(direction) = transportee_direction else {
                continue;
            };

            if !move_object(
                position,
                direction.as_delta(),
                &dimensions,
                collision_objects.into_iter().map(|(.., object)| object),
                Weight::None,
            ) {
                // If an object on a slippery entity cannot be moved, the
                // slippery entity's [BlocksMovement] component is disabled
                // until the object is moved away.
                *blocks_movement = BlocksMovement::Disabled;
            }
            already_moved.insert(*transportee);
        }
    }

    for (transporter_position, direction, mut blocks_movement) in &mut transporter_query {
        let (mut transportees, collision_objects): (Vec<_>, Vec<_>) = potential_transportees_query
            .iter_mut()
            .partition(|(.., (position, ..))| position.as_ref() == transporter_position);
        transportees.retain(|(entity, ..)| !already_moved.contains(entity));

        if let Some((transportee, .., (position, ..))) = transportees.first_mut() {
            if !move_object(
                position,
                direction.as_delta(),
                &dimensions,
                collision_objects.into_iter().map(|(.., object)| object),
                Weight::None,
            ) {
                // If an object on a transporter cannot be moved, the
                // transporter's [BlocksMovement] component is disabled until
                // the object is moved away.
                *blocks_movement = BlocksMovement::Disabled;
            }
            already_moved.insert(*transportee);
        }
    }
}

pub type TriggerSystemObject<'a> = (
    Entity,
    &'a Position,
    Option<&'a Openable>,
    Option<&'a Massive>,
    Option<&'a Trigger>,
    Option<&'a mut TextureAtlas>,
);

pub fn check_for_triggers(
    mut commands: Commands,
    mut query: Query<TriggerSystemObject>,
    mut pressed_triggers: ResMut<PressedTriggers>,
) {
    let mut triggers = Vec::new();
    let mut openables = Vec::new();
    let mut objects = Vec::new();
    for (entity, position, openable, massive, trigger, atlas) in &mut query {
        if trigger.is_some() {
            triggers.push(position);
        } else if matches!(openable, Some(Openable::Trigger)) {
            openables.push((entity, massive, atlas));
        } else {
            objects.push(position);
        }
    }

    let num_pressed_triggers = triggers
        .iter()
        .filter(|trigger_position| objects.iter().any(|position| position == *trigger_position))
        .count();

    let opened = match num_pressed_triggers.cmp(&pressed_triggers.num_pressed_triggers) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => return, // No change.
    };

    for (entity, massive, atlas) in openables {
        if opened && massive.is_some() {
            commands.entity(entity).remove::<Massive>();

            if let Some(mut atlas) = atlas {
                atlas.index = 1;
            }
        } else if !opened && massive.is_none() {
            commands.entity(entity).insert(Massive);

            if let Some(mut atlas) = atlas {
                atlas.index = 0;
            }
        }
    }

    pressed_triggers.num_pressed_triggers = num_pressed_triggers;
}

pub fn despawn_volatile_objects(
    mut commands: Commands,
    query: Query<Entity, With<Volatile>>,
    mut timer: ResMut<TemporaryTimer>,
    time: Res<Time>,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}

pub type CollisionObject<'a> = (
    Mut<'a, Position>,
    Option<&'a Pushable>,
    Option<&'a Massive>,
    Option<&'a BlocksPushes>,
    Option<&'a Weight>,
    Option<Mut<'a, BlocksMovement>>,
    Option<&'a Key>,
    Option<&'a Openable>,
    Option<&'a Mixable>,
    Option<&'a Paint>,
    Option<&'a Paintable>,
);

pub fn move_objects(
    mut movable_query: Query<(&mut Direction, &Movable, &mut Position, Option<&Weight>)>,
    mut collision_objects_query: Query<CollisionObject, Without<Movable>>,
    mut timer: ResMut<MovementTimer>,
    dimensions: Res<Dimensions>,
    time: Res<Time>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    for (mut direction, movable, mut position, weight) in &mut movable_query {
        match movable {
            Movable::Bounce => {
                if !move_object(
                    &mut position,
                    direction.as_delta(),
                    &dimensions,
                    collision_objects_query.iter_mut(),
                    weight.copied().unwrap_or_default(),
                ) {
                    *direction = direction.inverse();
                }
            }
            Movable::FollowRightHand => {
                if move_object(
                    &mut position,
                    direction.right_hand().as_delta(),
                    &dimensions,
                    collision_objects_query.iter_mut(),
                    weight.copied().unwrap_or_default(),
                ) {
                    *direction = direction.right_hand();
                } else if !move_object(
                    &mut position,
                    direction.as_delta(),
                    &dimensions,
                    collision_objects_query.iter_mut(),
                    weight.copied().unwrap_or_default(),
                ) {
                    *direction = direction.left_hand();
                }
            }
        }
    }
}

pub fn move_object<'a>(
    object_position: &mut Mut<Position>,
    (dx, dy): (i16, i16),
    dimensions: &Dimensions,
    collision_objects: impl Iterator<Item = CollisionObject<'a>>,
    max_weight: Weight,
) -> bool {
    let new_x = object_position.x + dx;
    let new_y = object_position.y + dy;
    if new_x < 1 || new_x > dimensions.width || new_y < 1 || new_y > dimensions.height {
        return false;
    }

    let mut collision_objects: Vec<_> = collision_objects
        .filter(|(position, ..)| {
            position.as_ref() == object_position.as_ref()
                || if dx > 0 {
                    position.x >= new_x && position.y == new_y
                } else if dx < 0 {
                    position.x <= new_x && position.y == new_y
                } else if dy > 0 {
                    position.x == new_x && position.y >= new_y
                } else if dy < 0 {
                    position.x == new_x && position.y <= new_y
                } else {
                    false
                }
        })
        .collect();

    collision_objects.sort_unstable_by_key(|(position, ..)| {
        (position.x - new_x).abs() + (position.y - new_y).abs()
    });

    let can_mix_with = |x: i16, y: i16, other: &Mixable| -> bool {
        if x < 1 || x > dimensions.width || y < 1 || y > dimensions.height {
            return false;
        }
        collision_objects
            .iter()
            .any(|(position, .., mixable, _, _)| {
                position.x == x
                    && position.y == y
                    && mixable.is_some_and(|mixable| mixable == other)
            })
    };

    let can_open_with_key = |x: i16, y: i16| -> bool {
        if x < 1 || x > dimensions.width || y < 1 || y > dimensions.height {
            return false;
        }
        collision_objects
            .iter()
            .any(|(position, .., openable, _, _, _)| {
                position.x == x && position.y == y && matches!(openable, Some(&Openable::Key))
            })
    };

    let can_paint = |x: i16, y: i16| -> bool {
        if x < 1 || x > dimensions.width || y < 1 || y > dimensions.height {
            return false;
        }
        collision_objects.iter().any(|(position, .., paintable)| {
            position.x == x && position.y == y && paintable.is_some()
        })
    };

    let can_push_to = |x: i16, y: i16| -> bool {
        if x < 1 || x > dimensions.width || y < 1 || y > dimensions.height {
            return false;
        }
        collision_objects
            .iter()
            .all(|(position, pushable, massive, blocks_pushes, ..)| {
                if position.x == x && position.y == y {
                    !massive.is_some() && !pushable.is_some() && !blocks_pushes.is_some()
                } else {
                    true
                }
            })
    };

    let mut pushed_object_indices = Vec::new();
    for (
        index,
        (position, pushable, massive, _, weight, .., blocks_movement, key, _, mixable, paint, _),
    ) in collision_objects.iter().enumerate()
    {
        if position.as_ref() == object_position.as_ref()
            && blocks_movement
                .as_ref()
                .is_some_and(|blocks| *blocks.as_ref() == BlocksMovement::Enabled)
        {
            return false;
        }

        if position.x == new_x && position.y == new_y {
            let can_push_to_or_mix_or_open_or_paint = |x: i16, y: i16| -> bool {
                can_push_to(x, y)
                    || key.is_some() && can_open_with_key(x, y)
                    || mixable.is_some_and(|mixable| can_mix_with(x, y, mixable))
                    || paint.is_some() && can_paint(x, y)
            };

            let weight = weight.copied().unwrap_or_default();
            if weight <= max_weight
                && pushable.is_some()
                && can_push_to_or_mix_or_open_or_paint(new_x + dx, new_y + dy)
            {
                pushed_object_indices.push(index);
                continue;
            }

            if massive.is_some() {
                return false;
            }
        }
    }

    for index in pushed_object_indices {
        let position = &mut collision_objects[index].0;
        position.x += dx;
        position.y += dy;
    }

    for (position, .., blocks_movement, _, _, _, _, _) in &mut collision_objects {
        if let Some(blocks_movement) = blocks_movement {
            if position.as_ref() == object_position.as_ref() {
                **blocks_movement = BlocksMovement::Enabled;
            }
        }
    }

    object_position.x = new_x;
    object_position.y = new_y;
    true
}
