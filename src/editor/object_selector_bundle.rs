use bevy::prelude::*;

use crate::{
    constants::*,
    game_object::{Direction, GameObjectAssets, ObjectType},
};

const NUM_OBJECTS: i16 = EditorObjectType::__Last as i16;
const NUM_COLUMNS: i16 = EDITOR_WIDTH / GRID_SIZE;
const NUM_ROWS: i16 =
    NUM_OBJECTS / NUM_COLUMNS + if NUM_OBJECTS % NUM_COLUMNS == 0 { 0 } else { 1 };
pub const SELECTOR_OUTLINE_WIDTH: i16 = 1;
const SELECTOR_WIDTH: i16 = NUM_COLUMNS * GRID_SIZE + (NUM_COLUMNS - 1) * SELECTOR_OUTLINE_WIDTH;
const SELECTOR_HEIGHT: i16 = NUM_ROWS * GRID_SIZE + (NUM_ROWS - 1) * SELECTOR_OUTLINE_WIDTH;

#[derive(Component)]
pub struct ObjectSelector;

#[derive(Clone, Component, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EditorObjectType {
    Eraser,
    BlueBlock,
    BluePaint,
    BouncingBallUp,
    BouncingBallRight,
    BouncingBallDown,
    BouncingBallLeft,
    Button,
    Creature1Up,
    Creature1Right,
    Creature1Down,
    Creature1Left,
    Door,
    Entrance,
    Exit,
    Gate,
    Key,
    Ice,
    Mine,
    Player,
    PurpleBlock,
    PurplePaint,
    Raft,
    RedBlock,
    RedPaint,
    Teleporter,
    TransporterUp,
    TransporterRight,
    TransporterDown,
    TransporterLeft,
    Water,
    YellowBlock,
    __Last,
}

impl EditorObjectType {
    pub fn get_object_type_and_direction(self) -> Option<(ObjectType, Direction)> {
        let object_type = match self {
            Self::BlueBlock => Some(ObjectType::BlueBlock),
            Self::BluePaint => Some(ObjectType::BluePaint),
            Self::BouncingBallUp
            | Self::BouncingBallRight
            | Self::BouncingBallDown
            | Self::BouncingBallLeft => Some(ObjectType::BouncingBall),
            Self::Button => Some(ObjectType::Button),
            Self::Creature1Up
            | Self::Creature1Right
            | Self::Creature1Down
            | Self::Creature1Left => Some(ObjectType::Creature1),
            Self::Door => Some(ObjectType::Door),
            Self::Entrance => Some(ObjectType::Entrance),
            Self::Exit => Some(ObjectType::Exit),
            Self::Gate => Some(ObjectType::Gate),
            Self::Ice => Some(ObjectType::Ice),
            Self::Key => Some(ObjectType::Key),
            Self::Mine => Some(ObjectType::Mine),
            Self::Player => Some(ObjectType::Player),
            Self::PurpleBlock => Some(ObjectType::PurpleBlock),
            Self::PurplePaint => Some(ObjectType::PurplePaint),
            Self::Raft => Some(ObjectType::Raft),
            Self::RedBlock => Some(ObjectType::RedBlock),
            Self::RedPaint => Some(ObjectType::RedPaint),
            Self::Teleporter => Some(ObjectType::Teleporter),
            Self::TransporterUp
            | Self::TransporterRight
            | Self::TransporterDown
            | Self::TransporterLeft => Some(ObjectType::Transporter),
            Self::Water => Some(ObjectType::Water),
            Self::YellowBlock => Some(ObjectType::YellowBlock),
            Self::Eraser | Self::__Last => None,
        };

        let direction = match self {
            Self::BouncingBallUp => Direction::Up,
            Self::BouncingBallRight => Direction::Right,
            Self::BouncingBallDown => Direction::Down,
            Self::BouncingBallLeft => Direction::Left,
            Self::Creature1Up => Direction::Up,
            Self::Creature1Right => Direction::Right,
            Self::Creature1Down => Direction::Down,
            Self::Creature1Left => Direction::Left,
            Self::TransporterUp => Direction::Up,
            Self::TransporterRight => Direction::Right,
            Self::TransporterDown => Direction::Down,
            Self::TransporterLeft => Direction::Left,
            _ => Direction::default(),
        };

        object_type.map(|object_type| (object_type, direction))
    }

    fn get_texture(self, assets: &GameObjectAssets) -> (Handle<Image>, Option<TextureAtlas>) {
        let image = match self {
            Self::Eraser => assets.eraser.clone(),
            Self::BlueBlock => assets.blue_block.clone(),
            Self::BluePaint => assets.blue_paint.clone(),
            Self::BouncingBallUp
            | Self::BouncingBallRight
            | Self::BouncingBallDown
            | Self::BouncingBallLeft => assets.bouncing_ball_editor.0.clone(),
            Self::Button => assets.button.clone(),
            Self::Creature1Up => assets.creature1.0.clone(),
            Self::Creature1Right => assets.creature1.0.clone(),
            Self::Creature1Down => assets.creature1.0.clone(),
            Self::Creature1Left => assets.creature1.0.clone(),
            Self::Door => assets.door.0.clone(),
            Self::Entrance => assets.entrance.0.clone(),
            Self::Exit => assets.exit.clone(),
            Self::Gate => assets.gate.0.clone(),
            Self::Ice => assets.ice.clone(),
            Self::Key => assets.key.clone(),
            Self::Mine => assets.mine.clone(),
            Self::Player => assets.player.clone(),
            Self::PurpleBlock => assets.purple_block.clone(),
            Self::PurplePaint => assets.purple_paint.clone(),
            Self::Raft => assets.raft.clone(),
            Self::RedBlock => assets.red_block.clone(),
            Self::RedPaint => assets.red_paint.clone(),
            Self::Teleporter => assets.teleporter.clone(),
            Self::TransporterUp
            | Self::TransporterRight
            | Self::TransporterDown
            | Self::TransporterLeft => assets.transporter.0.clone(),
            Self::Water => assets.water.0.clone(),
            Self::YellowBlock => assets.yellow_block.clone(),
            Self::__Last => unreachable!(),
        };

        let atlas = match self {
            Self::BouncingBallUp => Some(TextureAtlas {
                layout: assets.bouncing_ball_editor.1.clone(),
                index: 0,
            }),
            Self::BouncingBallRight => Some(TextureAtlas {
                layout: assets.bouncing_ball_editor.1.clone(),
                index: 1,
            }),
            Self::BouncingBallDown => Some(TextureAtlas {
                layout: assets.bouncing_ball_editor.1.clone(),
                index: 2,
            }),
            Self::BouncingBallLeft => Some(TextureAtlas {
                layout: assets.bouncing_ball_editor.1.clone(),
                index: 3,
            }),
            Self::Creature1Up => Some(TextureAtlas {
                layout: assets.creature1.1.clone(),
                index: 0,
            }),
            Self::Creature1Right => Some(TextureAtlas {
                layout: assets.creature1.1.clone(),
                index: 1,
            }),
            Self::Creature1Down => Some(TextureAtlas {
                layout: assets.creature1.1.clone(),
                index: 2,
            }),
            Self::Creature1Left => Some(TextureAtlas {
                layout: assets.creature1.1.clone(),
                index: 3,
            }),
            Self::Door => Some(TextureAtlas {
                layout: assets.door.1.clone(),
                index: 0,
            }),
            Self::Entrance => Some(TextureAtlas {
                layout: assets.entrance.1.clone(),
                index: 0,
            }),
            Self::Gate => Some(TextureAtlas {
                layout: assets.gate.1.clone(),
                index: 0,
            }),
            Self::TransporterUp => Some(TextureAtlas {
                layout: assets.transporter.1.clone(),
                index: 0,
            }),
            Self::TransporterRight => Some(TextureAtlas {
                layout: assets.transporter.1.clone(),
                index: 1,
            }),
            Self::TransporterDown => Some(TextureAtlas {
                layout: assets.transporter.1.clone(),
                index: 2,
            }),
            Self::TransporterLeft => Some(TextureAtlas {
                layout: assets.transporter.1.clone(),
                index: 3,
            }),
            Self::Water => Some(TextureAtlas {
                layout: assets.water.1.clone(),
                index: 0,
            }),
            _ => None,
        };

        (image, atlas)
    }
}

impl TryFrom<i16> for EditorObjectType {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let object_type = match value {
            0 => Self::Eraser,
            1 => Self::Player,
            2 => Self::Exit,
            3 => Self::RedBlock,
            4 => Self::BlueBlock,
            5 => Self::BouncingBallUp,
            6 => Self::BouncingBallRight,
            7 => Self::BouncingBallDown,
            8 => Self::BouncingBallLeft,
            9 => Self::Water,
            10 => Self::Creature1Up,
            11 => Self::Creature1Right,
            12 => Self::Creature1Down,
            13 => Self::Creature1Left,
            14 => Self::Raft,
            15 => Self::TransporterUp,
            16 => Self::TransporterRight,
            17 => Self::TransporterDown,
            18 => Self::TransporterLeft,
            19 => Self::Mine,
            20 => Self::Gate,
            21 => Self::Button,
            22 => Self::PurpleBlock,
            23 => Self::YellowBlock,
            24 => Self::Ice,
            25 => Self::Door,
            26 => Self::Key,
            27 => Self::PurplePaint,
            28 => Self::RedPaint,
            29 => Self::BluePaint,
            30 => Self::Teleporter,
            31 => Self::Entrance,
            _ => return Err(()),
        };
        Ok(object_type)
    }
}

#[derive(Bundle)]
pub struct ObjectSelectorBundle {
    background: NodeBundle,
    selector: ObjectSelector,
}

impl ObjectSelectorBundle {
    pub fn new() -> Self {
        Self {
            background: NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Px(SELECTOR_WIDTH as f32),
                    height: Val::Px(SELECTOR_HEIGHT as f32),
                    grid_template_columns: (0..NUM_COLUMNS)
                        .map(|_| GridTrack::px(GRID_SIZE as f32))
                        .collect(),
                    grid_template_rows: (0..NUM_ROWS)
                        .map(|_| GridTrack::px(GRID_SIZE as f32))
                        .collect(),
                    row_gap: Val::Px(SELECTOR_OUTLINE_WIDTH as f32),
                    column_gap: Val::Px(SELECTOR_OUTLINE_WIDTH as f32),
                    ..Default::default()
                },
                background_color: NORMAL_GRAY.into(),
                ..Default::default()
            },
            selector: ObjectSelector,
        }
    }

    pub fn populate(cb: &mut ChildBuilder, assets: &GameObjectAssets) {
        for i in 0..NUM_OBJECTS {
            let object_type = EditorObjectType::try_from(i).unwrap();
            let (texture, atlas) = object_type.get_texture(assets);
            let image = ImageBundle {
                image: UiImage::new(texture),
                ..Default::default()
            };
            let interaction = Interaction::None;

            if let Some(atlas) = atlas {
                cb.spawn((object_type, interaction, image, atlas));
            } else {
                cb.spawn((object_type, interaction, image));
            }
        }
    }
}
