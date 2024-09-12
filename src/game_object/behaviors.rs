use std::{cmp::Ordering, collections::BTreeSet};

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    fonts::Fonts,
    game_object::Pushable,
    level::{Dimensions, InitialPositionAndMetadata},
    timers::{AnimationTimer, MovementTimer, TemporaryTimer, TransporterTimer},
    Background, GameEvent, PressedTriggers, SaveLevel,
};

use super::{
    components::{Animatable, Direction, Trigger, *},
    object_bundles::*,
    spawn_object_of_type, GameObjectAssets,
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
    background_query: Query<Entity, With<Background>>,
    deadly_query: Query<(Entity, &Position), With<Deadly>>,
    player_query: Query<(Entity, &Position), With<Player>>,
    assets: Res<GameObjectAssets>,
) {
    for (player, player_position) in &player_query {
        for (deadly, deadly_position) in &deadly_query {
            if player_position == deadly_position {
                commands.entity(player).despawn();
                commands.entity(deadly).despawn();

                let background = background_query
                    .get_single()
                    .expect("there should be only one background");
                let mut background = commands.entity(background);
                background.with_children(|cb| {
                    cb.spawn(GraveBundle::spawn(&assets, *player_position));
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
) {
    for player_position in &player_query {
        if player_position.is_added() || !player_position.is_changed() {
            continue;
        }

        for exit_position in &exit_query {
            if player_position.as_ref() == exit_position {
                level_events.send(GameEvent::LoadLevel(0));
                return;
            }
        }
    }
}

pub type ExplosiveSystemObject<'a> = (Entity, Ref<'a, Position>, Option<&'a Explosive>);

pub fn check_for_explosive(
    mut commands: Commands,
    explosive_query: Query<ExplosiveSystemObject>,
    background_query: Query<Entity, With<Background>>,
    mut temporary_timer: ResMut<TemporaryTimer>,
    assets: Res<GameObjectAssets>,
) {
    let (explosives, objects): (Vec<ExplosiveSystemObject>, Vec<ExplosiveSystemObject>) =
        explosive_query
            .iter()
            .partition(|(_, _, explosive)| explosive.is_some());

    for (object, position, _) in &objects {
        if !position.is_changed() {
            continue;
        }

        for (explosive, explosive_position, ..) in &explosives {
            if explosive_position.as_ref() == position.as_ref() {
                commands.entity(*explosive).despawn();
                commands.entity(*object).despawn();

                let background = background_query
                    .get_single()
                    .expect("there should be only one background");
                let mut background = commands.entity(background);
                background.with_children(|cb| {
                    cb.spawn(ExplosionBundle::spawn(&assets, **position));
                });
                if temporary_timer.finished() {
                    temporary_timer.reset();
                }
            }
        }
    }
}

pub type LiquidSystemObject<'a> = (
    Entity,
    Ref<'a, Position>,
    Option<&'a Liquid>,
    Option<&'a Floatable>,
);

pub fn check_for_liquid(
    mut commands: Commands,
    liquid_query: Query<LiquidSystemObject>,
    background_query: Query<Entity, With<Background>>,
    mut temporary_timer: ResMut<TemporaryTimer>,
    assets: Res<GameObjectAssets>,
) {
    let (liquids, objects): (Vec<LiquidSystemObject>, Vec<LiquidSystemObject>) = liquid_query
        .iter()
        .partition(|(_, _, liquid, ..)| liquid.is_some());

    for (object, position, _, floatable) in &objects {
        if !position.is_changed() {
            continue;
        }

        for (_liquid, liquid_position, ..) in &liquids {
            if liquid_position.as_ref() == position.as_ref() {
                if floatable.is_some() {
                    if !objects.iter().any(|(other, other_position, _, floatable)| {
                        other != object
                            && other_position.as_ref() == position.as_ref()
                            && floatable.is_some()
                    }) {
                        let mut object = commands.entity(*object);
                        object.remove::<Pushable>();
                    }
                } else if !objects.iter().any(|(_, other_position, _, floatable)| {
                    other_position.as_ref() == position.as_ref() && floatable.is_some()
                }) {
                    commands.entity(*object).despawn();

                    let background = background_query
                        .get_single()
                        .expect("there should be only one background");
                    let mut background = commands.entity(background);
                    background.with_children(|cb| {
                        cb.spawn(SplashBundle::spawn(&assets, **position));
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
pub fn check_for_transform_on_push(
    mut commands: Commands,
    background_query: Query<Entity, With<Background>>,
    transform_query: Query<
        (Entity, Option<&Direction>, Ref<Position>, &TransformOnPush),
        With<Pushable>,
    >,
    assets: Res<GameObjectAssets>,
    fonts: Res<Fonts>,
) {
    for (entity, direction, position, TransformOnPush(object_type)) in &transform_query {
        if position.is_changed() && !position.is_added() {
            commands.entity(entity).despawn();

            let background_entity = background_query
                .get_single()
                .expect("there should be only one background");

            let mut background = commands.entity(background_entity);
            background.with_children(|cb| {
                spawn_object_of_type(
                    cb,
                    &assets,
                    &fonts,
                    *object_type,
                    InitialPositionAndMetadata {
                        position: *position,
                        direction: direction.copied(),
                        level: None,
                    },
                );
            });
        }
    }
}

pub fn check_for_transporter(
    mut transporter_query: Query<(&Direction, &Position, &mut BlocksMovement), With<Transporter>>,
    mut collision_objects_query: Query<(Entity, CollisionObject), Without<Transporter>>,
    mut timer: ResMut<TransporterTimer>,
    dimensions: Res<Dimensions>,
    time: Res<Time>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    let mut already_moved = BTreeSet::new();
    for (direction, transporter_position, mut blocks_movement) in &mut transporter_query {
        let (mut transported_objects, collision_objects): (Vec<_>, Vec<_>) =
            collision_objects_query
                .iter_mut()
                .partition(|(_, (position, ..))| position.as_ref() == transporter_position);
        transported_objects.retain(|(entity, _)| !already_moved.contains(entity));
        if let Some((transported, (position, ..))) = transported_objects.first_mut() {
            if !move_object(
                position,
                direction.as_delta(),
                &dimensions,
                collision_objects.into_iter().map(|(_, object)| object),
                Weight::None,
            ) {
                // If an object on a transporter cannot be moved, the
                // transporter's [BlocksMovement] component is disabled until
                // the object is moved away.
                *blocks_movement = BlocksMovement::Disabled;
            }
            already_moved.insert(*transported);
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
        } else if openable.is_some() {
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

    for (openable, massive, atlas) in openables {
        if opened && massive.is_some() {
            commands.entity(openable).remove::<Massive>();

            if let Some(mut atlas) = atlas {
                atlas.index = 1;
            }
        } else if !opened && massive.is_none() {
            commands.entity(openable).insert(Massive);

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

    let can_push_to = |x: i16, y: i16| -> bool {
        if x < 1 || x > dimensions.width || y < 1 || y > dimensions.height {
            return false;
        }
        for (position, pushable, massive, blocks_pushes, ..) in &collision_objects {
            let has_target_position = position.x == x && position.y == y;
            if !has_target_position {
                continue;
            }

            let can_push_to = !pushable.is_some() && !massive.is_some() && !blocks_pushes.is_some();
            if !can_push_to {
                return false;
            }
        }
        true
    };

    let mut pushed_object_indices = Vec::new();
    for (index, (position, pushable, massive, _, weight, .., blocks_movement)) in
        collision_objects.iter().enumerate()
    {
        if position.as_ref() == object_position.as_ref()
            && blocks_movement
                .as_ref()
                .is_some_and(|blocks| *blocks.as_ref() == BlocksMovement::Enabled)
        {
            return false;
        }

        if position.x == new_x && position.y == new_y {
            let weight = weight.copied().unwrap_or_default();
            if weight <= max_weight && pushable.is_some() && can_push_to(new_x + dx, new_y + dy) {
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

    for (position, .., blocks_movement) in &mut collision_objects {
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
