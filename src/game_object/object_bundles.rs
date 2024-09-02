use bevy::prelude::*;

use super::{
    assets::GameObjectAssets,
    components::{Exit, Liquid, Massive, Player, Position, Pushable},
    Animatable, BlocksMovement, BlocksPushes, Deadly, Direction, Entrance, Explosive, Floatable,
    Movable, ObjectType, Openable, TransformOnPush, Transporter, Trigger, Volatile, Weight,
};

#[derive(Bundle)]
pub struct BlackFillBundle {
    object_type: ObjectType,
    position: Position,
    sprite: SpriteBundle,
}

impl BlackFillBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::BlackFill,
            position,
            sprite: SpriteBundle {
                texture: assets.black_fill.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.))
                    .with_scale(Vec3::new(48.1, 48.1, 1.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct BlueBlockBundle {
    object_type: ObjectType,
    massive: Massive,
    position: Position,
    pushable: Pushable,
    sprite: SpriteBundle,
    weight: Weight,
}

impl BlueBlockBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::BlueBlock,
            massive: Massive,
            position,
            pushable: Pushable,
            sprite: SpriteBundle {
                texture: assets.blue_block.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 3.)),
                ..Default::default()
            },
            weight: Weight::Heavy,
        }
    }
}

#[derive(Bundle)]
pub struct BouncingBallBundle {
    object_type: ObjectType,
    blocks_pushes: BlocksPushes,
    deadly: Deadly,
    direction: Direction,
    movable: Movable,
    position: Position,
    sprite: SpriteBundle,
}

impl BouncingBallBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position, direction: Direction) -> Self {
        Self {
            object_type: ObjectType::BouncingBall,
            blocks_pushes: BlocksPushes,
            deadly: Deadly,
            direction,
            movable: Movable::Bounce,
            position,
            sprite: SpriteBundle {
                texture: assets.bouncing_ball.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 4.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct ButtonBundle {
    object_type: ObjectType,
    position: Position,
    sprite: SpriteBundle,
    trigger: Trigger,
}

impl ButtonBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::Button,
            position,
            sprite: SpriteBundle {
                texture: assets.button.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
            trigger: Trigger,
        }
    }
}

#[derive(Bundle)]
pub struct Creature1Bundle {
    object_type: ObjectType,
    atlas: TextureAtlas,
    blocks_pushes: BlocksPushes,
    deadly: Deadly,
    direction: Direction,
    movable: Movable,
    position: Position,
    sprite: SpriteBundle,
}

impl Creature1Bundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position, direction: Direction) -> Self {
        Self {
            object_type: ObjectType::Creature1,
            atlas: TextureAtlas {
                layout: assets.creature1.1.clone(),
                index: direction as usize,
            },
            blocks_pushes: BlocksPushes,
            deadly: Deadly,
            direction,
            movable: Movable::FollowRightHand,
            position,
            sprite: SpriteBundle {
                texture: assets.creature1.0.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 4.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct EntranceBundle {
    object_type: ObjectType,
    entrance: Entrance,
    position: Position,
    sprite: SpriteBundle,
}

impl EntranceBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position, level: u16) -> Self {
        Self {
            object_type: ObjectType::Entrance,
            entrance: Entrance(level),
            position,
            sprite: SpriteBundle {
                texture: assets.entrance.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct ExitBundle {
    object_type: ObjectType,
    exit: Exit,
    position: Position,
    sprite: SpriteBundle,
}

impl ExitBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::Exit,
            exit: Exit,
            position,
            sprite: SpriteBundle {
                texture: assets.exit.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct ExplosionBundle {
    position: Position,
    sprite: SpriteBundle,
    volatile: Volatile,
}

impl ExplosionBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            position,
            sprite: SpriteBundle {
                texture: assets.explosion.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 4.)),
                ..Default::default()
            },
            volatile: Volatile,
        }
    }
}

#[derive(Bundle)]
pub struct GateBundle {
    object_type: ObjectType,
    atlas: TextureAtlas,
    openable: Openable,
    massive: Massive,
    position: Position,
    sprite: SpriteBundle,
}

impl GateBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::Gate,
            atlas: TextureAtlas {
                layout: assets.gate.1.clone(),
                index: 0,
            },
            massive: Massive,
            openable: Openable,
            position,
            sprite: SpriteBundle {
                texture: assets.gate.0.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 5.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct GraveBundle {
    massive: Massive,
    position: Position,
    sprite: SpriteBundle,
}

impl GraveBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            massive: Massive,
            position,
            sprite: SpriteBundle {
                texture: assets.grave.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 4.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct MineBundle {
    object_type: ObjectType,
    explosive: Explosive,
    position: Position,
    sprite: SpriteBundle,
}

impl MineBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::Mine,
            explosive: Explosive,
            position,
            sprite: SpriteBundle {
                texture: assets.mine.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    object_type: ObjectType,
    blocks_pushes: BlocksPushes,
    player: Player,
    position: Position,
    sprite: SpriteBundle,
    weight: Weight,
}

impl PlayerBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::Player,
            blocks_pushes: BlocksPushes,
            player: Player,
            position,
            sprite: SpriteBundle {
                texture: assets.player.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 3.)),
                ..Default::default()
            },
            weight: Weight::Heavy,
        }
    }
}

#[derive(Bundle)]
pub struct PurpleBlockBundle {
    object_type: ObjectType,
    massive: Massive,
    position: Position,
    pushable: Pushable,
    sprite: SpriteBundle,
    transforms: TransformOnPush,
}

impl PurpleBlockBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::PurpleBlock,
            massive: Massive,
            position,
            pushable: Pushable,
            sprite: SpriteBundle {
                texture: assets.purple_block.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 3.)),
                ..Default::default()
            },
            transforms: TransformOnPush(ObjectType::RedBlock),
        }
    }
}

#[derive(Bundle)]
pub struct RaftBundle {
    object_type: ObjectType,
    floatable: Floatable,
    position: Position,
    pushable: Pushable,
    sprite: SpriteBundle,
    weight: Weight,
}

impl RaftBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::Raft,
            floatable: Floatable,
            position,
            pushable: Pushable,
            sprite: SpriteBundle {
                texture: assets.raft.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                ..Default::default()
            },
            weight: Weight::Heavy,
        }
    }
}

#[derive(Bundle)]
pub struct RedBlockBundle {
    object_type: ObjectType,
    massive: Massive,
    position: Position,
    sprite: SpriteBundle,
}

impl RedBlockBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::RedBlock,
            massive: Massive,
            position,
            sprite: SpriteBundle {
                texture: assets.red_block.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct SplashBundle {
    floatable: Floatable,
    position: Position,
    sprite: SpriteBundle,
    volatile: Volatile,
}

impl SplashBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            floatable: Floatable,
            position,
            sprite: SpriteBundle {
                texture: assets.splash.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 4.)),
                ..Default::default()
            },
            volatile: Volatile,
        }
    }
}

#[derive(Bundle)]
pub struct TransporterBundle {
    object_type: ObjectType,
    atlas: TextureAtlas,
    blocks_movement: BlocksMovement,
    direction: Direction,
    position: Position,
    sprite: SpriteBundle,
    transporter: Transporter,
}

impl TransporterBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position, direction: Direction) -> Self {
        Self {
            object_type: ObjectType::Transporter,
            atlas: TextureAtlas {
                layout: assets.transporter.1.clone(),
                index: 0,
            },
            blocks_movement: BlocksMovement::Enabled,
            direction,
            position,
            sprite: SpriteBundle {
                texture: assets.transporter.0.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
            transporter: Transporter,
        }
    }
}

#[derive(Bundle)]
pub struct WaterBundle {
    object_type: ObjectType,
    animatable: Animatable,
    atlas: TextureAtlas,
    liquid: Liquid,
    position: Position,
    sprite: SpriteBundle,
}

impl WaterBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::Water,
            animatable: Animatable { num_frames: 3 },
            atlas: TextureAtlas {
                layout: assets.water.1.clone(),
                index: 0,
            },
            liquid: Liquid,
            position,
            sprite: SpriteBundle {
                texture: assets.water.0.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct YellowBlockBundle {
    object_type: ObjectType,
    massive: Massive,
    position: Position,
    pushable: Pushable,
    sprite: SpriteBundle,
}

impl YellowBlockBundle {
    pub fn spawn(assets: &GameObjectAssets, position: Position) -> Self {
        Self {
            object_type: ObjectType::YellowBlock,
            massive: Massive,
            position,
            pushable: Pushable,
            sprite: SpriteBundle {
                texture: assets.yellow_block.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 3.)),
                ..Default::default()
            },
        }
    }
}
