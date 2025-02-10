use bevy::prelude::*;

use crate::levels::InitialPositionAndMetadata;

use super::{
    assets::GameObjectAssets,
    components::{Exit, Liquid, Massive, Player, Position, Pushable},
    Animatable, BlocksMovement, BlocksPushes, Deadly, Direction, Entrance, Explosive, Floatable,
    Key, Movable, ObjectType, Openable, Paint, Paintable, Slippery, Teleporter, TransformOnPush,
    Transporter, Trigger, Volatile, Weight,
};

pub struct BlueBlock;

impl BlueBlock {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::BlueBlock,
            Massive,
            Paintable,
            position,
            Pushable,
            Sprite::from_image(assets.blue_block.clone()),
            Transform::from_translation(Vec3::new(0., 0., 3.)),
            Weight::Heavy,
        )
    }
}

pub struct BluePaint;

impl BluePaint {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::BluePaint,
            Paint(ObjectType::BlueBlock),
            position,
            Pushable,
            Sprite::from_image(assets.blue_paint.clone()),
            Transform::from_translation(Vec3::new(0., 0., 3.)),
            Weight::Light,
        )
    }
}

pub struct BouncingBall;

impl BouncingBall {
    pub fn spawn(
        assets: &GameObjectAssets,
        position: Position,
        direction: Direction,
    ) -> impl Bundle {
        (
            ObjectType::BouncingBall,
            BlocksPushes,
            Deadly,
            direction,
            Movable::Bounce,
            position,
            Sprite::from_image(assets.bouncing_ball.clone()),
            Transform::from_translation(Vec3::new(0., 0., 4.)),
            Weight::Light,
        )
    }
}

pub struct Button;

impl Button {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::Button,
            position,
            Sprite::from_image(assets.button.clone()),
            Transform::from_translation(Vec3::new(0., 0., 1.)),
            Trigger,
        )
    }
}

pub struct Creature1;

impl Creature1 {
    pub fn spawn(
        assets: &GameObjectAssets,
        position: Position,
        direction: Direction,
    ) -> impl Bundle {
        (
            ObjectType::Creature1,
            BlocksPushes,
            Deadly,
            direction,
            Movable::FollowRightHand,
            position,
            Sprite::from_atlas_image(
                assets.creature1.0.clone(),
                TextureAtlas {
                    layout: assets.creature1.1.clone(),
                    index: direction as usize,
                },
            ),
            Transform::from_translation(Vec3::new(0., 0., 4.)),
            Weight::Light,
        )
    }
}

pub struct Door;

impl Door {
    pub fn spawn<'a>(
        cb: &'a mut ChildBuilder,
        assets: &GameObjectAssets,
        position: Position,
        initial_position: InitialPositionAndMetadata,
    ) -> EntityCommands<'a> {
        let InitialPositionAndMetadata { open, .. } = initial_position;

        let openable = Openable::Key;
        let sprite = Sprite::from_atlas_image(
            assets.door.0.clone(),
            TextureAtlas {
                layout: assets.door.1.clone(),
                index: if open { 1 } else { 0 },
            },
        );
        let transform = Transform::from_translation(Vec3::new(0., 0., 5.));

        if open {
            cb.spawn((ObjectType::Door, openable, position, sprite, transform))
        } else {
            cb.spawn((
                ObjectType::Door,
                Massive,
                openable,
                position,
                sprite,
                transform,
            ))
        }
    }
}

impl Entrance {
    pub fn spawn(assets: &GameObjectAssets, position: Position, level: u16) -> impl Bundle {
        (
            ObjectType::Entrance,
            BlocksPushes,
            Entrance(level),
            position,
            Sprite::from_atlas_image(
                assets.entrance.0.clone(),
                TextureAtlas {
                    layout: assets.entrance.1.clone(),
                    index: 0,
                },
            ),
            Transform::from_translation(Vec3::new(0., 0., 1.)),
        )
    }
}

impl Exit {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::Exit,
            BlocksPushes,
            Exit,
            position,
            Sprite::from_image(assets.exit.clone()),
            Transform::from_translation(Vec3::new(0., 0., 1.)),
        )
    }
}

pub struct Explosion;

impl Explosion {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            position,
            Sprite::from_image(assets.explosion.clone()),
            Transform::from_translation(Vec3::new(0., 0., 4.)),
            Volatile,
        )
    }
}

pub struct Gate;

impl Gate {
    pub fn spawn<'a>(
        cb: &'a mut ChildBuilder,
        assets: &GameObjectAssets,
        position: Position,
        initial_position: InitialPositionAndMetadata,
    ) -> EntityCommands<'a> {
        let InitialPositionAndMetadata { level, open, .. } = initial_position;

        let openable = if let Some(level) = level {
            Openable::LevelFinished(level)
        } else {
            Openable::Trigger
        };
        let sprite = Sprite::from_atlas_image(
            assets.gate.0.clone(),
            TextureAtlas {
                layout: assets.gate.1.clone(),
                index: if open { 1 } else { 0 },
            },
        );
        let transform = Transform::from_translation(Vec3::new(0., 0., 5.));

        if open {
            cb.spawn((ObjectType::Gate, openable, position, sprite, transform))
        } else {
            cb.spawn((
                ObjectType::Gate,
                Massive,
                openable,
                position,
                sprite,
                transform,
            ))
        }
    }
}

pub struct Grave;

impl Grave {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            Massive,
            position,
            Sprite::from_image(assets.grave.clone()),
            Transform::from_translation(Vec3::new(0., 0., 4.)),
        )
    }
}

pub struct Ice;

impl Ice {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::Ice,
            BlocksMovement::Enabled,
            position,
            Slippery,
            Sprite::from_image(assets.ice.clone()),
            Transform::from_translation(Vec3::new(0., 0., 1.)),
        )
    }
}

impl Key {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::Key,
            Key,
            position,
            Pushable,
            Sprite::from_image(assets.key.clone()),
            Transform::from_translation(Vec3::new(0., 0., 2.)),
            Weight::Light,
        )
    }
}

pub struct Mine;

impl Mine {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::Mine,
            Explosive,
            position,
            Sprite::from_image(assets.mine.clone()),
            Transform::from_translation(Vec3::new(0., 0., 1.)),
        )
    }
}

impl Player {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::Player,
            BlocksPushes,
            Player,
            position,
            Sprite::from_image(assets.player.clone()),
            Transform::from_translation(Vec3::new(0., 0., 4.)),
            Weight::Heavy,
        )
    }
}

pub struct PurpleBlock;

impl PurpleBlock {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::PurpleBlock,
            Massive,
            Paintable,
            position,
            Pushable,
            Sprite::from_image(assets.purple_block.clone()),
            Transform::from_translation(Vec3::new(0., 0., 3.)),
            TransformOnPush(ObjectType::RedBlock),
            Weight::Heavy,
        )
    }
}

pub struct PurplePaint;

impl PurplePaint {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::PurplePaint,
            Paint(ObjectType::PurpleBlock),
            position,
            Pushable,
            Sprite::from_image(assets.purple_paint.clone()),
            Transform::from_translation(Vec3::new(0., 0., 3.)),
            Weight::Light,
        )
    }
}

pub struct Raft;

impl Raft {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::Raft,
            Floatable,
            position,
            Pushable,
            Sprite::from_image(assets.raft.clone()),
            Transform::from_translation(Vec3::new(0., 0., 2.)),
            Weight::Heavy,
        )
    }
}

pub struct RedBlock;

impl RedBlock {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::RedBlock,
            Massive,
            Paintable,
            position,
            Sprite::from_image(assets.red_block.clone()),
            Transform::from_translation(Vec3::new(0., 0., 2.)),
        )
    }
}

pub struct RedPaint;

impl RedPaint {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::RedPaint,
            Paint(ObjectType::RedBlock),
            position,
            Pushable,
            Sprite::from_image(assets.red_paint.clone()),
            Transform::from_translation(Vec3::new(0., 0., 3.)),
            Weight::Light,
        )
    }
}

pub struct Splash;

impl Splash {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            Floatable,
            position,
            Sprite::from_image(assets.splash.clone()),
            Transform::from_translation(Vec3::new(0., 0., 4.)),
            Volatile,
        )
    }
}

impl Teleporter {
    pub fn spawn(assets: &GameObjectAssets, position: Position, identifier: u16) -> impl Bundle {
        (
            ObjectType::Teleporter,
            position,
            Sprite::from_image(assets.teleporter.clone()),
            Transform::from_translation(Vec3::new(0., 0., 1.)),
            Teleporter(identifier),
        )
    }
}

impl Transporter {
    pub fn spawn(
        assets: &GameObjectAssets,
        position: Position,
        direction: Direction,
    ) -> impl Bundle {
        (
            ObjectType::Transporter,
            BlocksMovement::Enabled,
            direction,
            position,
            Sprite::from_atlas_image(
                assets.transporter.0.clone(),
                TextureAtlas {
                    layout: assets.transporter.1.clone(),
                    index: 0,
                },
            ),
            Transform::from_translation(Vec3::new(0., 0., 1.)),
            Transporter,
        )
    }
}

pub struct Water;

impl Water {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::Water,
            Animatable { num_frames: 3 },
            Liquid,
            position,
            Sprite::from_atlas_image(
                assets.water.0.clone(),
                TextureAtlas {
                    layout: assets.water.1.clone(),
                    index: 0,
                },
            ),
            Transform::from_translation(Vec3::new(0., 0., 1.)),
        )
    }
}

pub struct YellowBlock;

impl YellowBlock {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> impl Bundle {
        (
            ObjectType::YellowBlock,
            Massive,
            Paintable,
            position,
            Pushable,
            Sprite::from_image(assets.yellow_block.clone()),
            Transform::from_translation(Vec3::new(0., 0., 3.)),
            Weight::Light,
        )
    }
}
