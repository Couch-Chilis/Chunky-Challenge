use std::fmt::Display;
use std::str::FromStr;

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{
    errors::UnknownObjectType, fonts::Fonts, level::InitialPositionAndMetadata, ENTRANCE_TEXT,
};

use super::{
    assets::GameObjectAssets,
    object_bundles::{
        BlueBlockBundle, BouncingBallBundle, Creature1Bundle, ExitBundle, PlayerBundle, RaftBundle,
        RedBlockBundle, WaterBundle,
    },
    ButtonBundle, EntranceBundle, GateBundle, IceBundle, MineBundle, PurpleBlockBundle,
    TransporterBundle, YellowBlockBundle,
};

#[derive(Clone, Component, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ObjectType {
    BlueBlock,
    BouncingBall,
    Button,
    Creature1,
    Entrance,
    Exit,
    Gate,
    Ice,
    Mine,
    Player,
    PurpleBlock,
    Raft,
    RedBlock,
    Transporter,
    Water,
    YellowBlock,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::BlueBlock => "BlueBlock",
            Self::BouncingBall => "BouncingBall",
            Self::Button => "Button",
            Self::Creature1 => "Creature1",
            Self::Entrance => "Entrance",
            Self::Exit => "Exit",
            Self::Gate => "Gate",
            Self::Ice => "Ice",
            Self::Mine => "Mine",
            Self::Player => "Player",
            Self::PurpleBlock => "PurpleBlock",
            Self::Raft => "Raft",
            Self::RedBlock => "RedBlock",
            Self::Transporter => "Transporter",
            Self::Water => "Water",
            Self::YellowBlock => "YellowBlock",
        })
    }
}

impl FromStr for ObjectType {
    type Err = UnknownObjectType;

    fn from_str(object_type: &str) -> Result<Self, Self::Err> {
        match object_type {
            "BlueBlock" => Ok(Self::BlueBlock),
            "BouncingBall" => Ok(Self::BouncingBall),
            "Button" => Ok(Self::Button),
            "Creature1" => Ok(Self::Creature1),
            "Entrance" => Ok(Self::Entrance),
            "Exit" => Ok(Self::Exit),
            "Gate" => Ok(Self::Gate),
            "Ice" => Ok(Self::Ice),
            "Mine" => Ok(Self::Mine),
            "Player" => Ok(Self::Player),
            "PurpleBlock" => Ok(Self::PurpleBlock),
            "Raft" => Ok(Self::Raft),
            "RedBlock" => Ok(Self::RedBlock),
            "Transporter" => Ok(Self::Transporter),
            "Water" => Ok(Self::Water),
            "YellowBlock" => Ok(Self::YellowBlock),
            _ => Err(UnknownObjectType),
        }
    }
}

pub fn spawn_object_of_type<'a>(
    cb: &'a mut ChildBuilder,
    assets: &GameObjectAssets,
    fonts: &Fonts,
    object_type: ObjectType,
    initial_position: InitialPositionAndMetadata,
) -> EntityCommands<'a> {
    let position = initial_position.position;

    match object_type {
        ObjectType::BlueBlock => cb.spawn(BlueBlockBundle::spawn(assets, position)),
        ObjectType::BouncingBall => cb.spawn(BouncingBallBundle::spawn(
            assets,
            position,
            initial_position.direction.unwrap_or_default(),
        )),
        ObjectType::Button => cb.spawn(ButtonBundle::spawn(assets, position)),
        ObjectType::Creature1 => cb.spawn(Creature1Bundle::spawn(
            assets,
            position,
            initial_position.direction.unwrap_or_default(),
        )),
        ObjectType::Entrance => {
            let mut cb = cb.spawn(EntranceBundle::spawn(
                assets,
                position,
                initial_position.level.unwrap_or_default(),
            ));
            cb.with_children(|cb| {
                cb.spawn(Text2dBundle {
                    text: Text::from_section(
                        initial_position.level.unwrap_or_default().to_string(),
                        TextStyle {
                            font: fonts.poppins_light.clone(),
                            font_size: 24.,
                            color: ENTRANCE_TEXT,
                        },
                    ),
                    transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                    ..Default::default()
                });
            });
            cb
        }
        ObjectType::Exit => cb.spawn(ExitBundle::spawn(assets, position)),
        ObjectType::Gate => cb.spawn(GateBundle::spawn(assets, position)),
        ObjectType::Ice => cb.spawn(IceBundle::spawn(assets, position)),
        ObjectType::Mine => cb.spawn(MineBundle::spawn(assets, position)),
        ObjectType::Player => cb.spawn(PlayerBundle::spawn(assets, position)),
        ObjectType::PurpleBlock => cb.spawn(PurpleBlockBundle::spawn(assets, position)),
        ObjectType::Raft => cb.spawn(RaftBundle::spawn(assets, position)),
        ObjectType::RedBlock => cb.spawn(RedBlockBundle::spawn(assets, position)),
        ObjectType::Transporter => cb.spawn(TransporterBundle::spawn(
            assets,
            position,
            initial_position.direction.unwrap_or_default(),
        )),
        ObjectType::Water => cb.spawn(WaterBundle::spawn(assets, position)),
        ObjectType::YellowBlock => cb.spawn(YellowBlockBundle::spawn(assets, position)),
    }
}
