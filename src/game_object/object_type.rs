use std::fmt::Display;
use std::str::FromStr;

use bevy::prelude::*;

use crate::{errors::UnknownObjectType, fonts::Fonts, levels::InitialPositionAndMetadata};

use super::{
    assets::GameObjectAssets,
    object_bundles::{BlueBlock, BouncingBall, Creature1, Raft, RedBlock, Water},
    BluePaint, Button, Direction, Door, Entrance, Exit, Explosion, Flash, Gate, Grave, Ice, Key,
    Mine, Player, PurpleBlock, PurplePaint, RedPaint, Splash, Teleporter, Transporter, YellowBlock,
};

#[derive(Clone, Component, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[require(Direction)]
pub enum ObjectType {
    BlueBlock,
    BluePaint,
    BouncingBall,
    Button,
    Creature1,
    Door,
    Entrance,
    Exit,
    Explosion,
    Flash,
    Gate,
    Grave,
    Ice,
    Key,
    Mine,
    Player,
    PurpleBlock,
    PurplePaint,
    Raft,
    RedBlock,
    RedPaint,
    Splash,
    Teleporter,
    Transporter,
    Water,
    YellowBlock,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::BlueBlock => "BlueBlock",
            Self::BluePaint => "BluePaint",
            Self::BouncingBall => "BouncingBall",
            Self::Button => "Button",
            Self::Creature1 => "Creature1",
            Self::Door => "Door",
            Self::Entrance => "Entrance",
            Self::Exit => "Exit",
            Self::Explosion => "Explosion",
            Self::Flash => "Flash",
            Self::Gate => "Gate",
            Self::Grave => "Grave",
            Self::Ice => "Ice",
            Self::Key => "Key",
            Self::Mine => "Mine",
            Self::Player => "Player",
            Self::PurpleBlock => "PurpleBlock",
            Self::PurplePaint => "PurplePaint",
            Self::Raft => "Raft",
            Self::RedBlock => "RedBlock",
            Self::RedPaint => "RedPaint",
            Self::Splash => "Splash",
            Self::Teleporter => "Teleporter",
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
            "BluePaint" => Ok(Self::BluePaint),
            "BouncingBall" => Ok(Self::BouncingBall),
            "Button" => Ok(Self::Button),
            "Creature1" => Ok(Self::Creature1),
            "Door" => Ok(Self::Door),
            "Entrance" => Ok(Self::Entrance),
            "Exit" => Ok(Self::Exit),
            "Gate" => Ok(Self::Gate),
            "Ice" => Ok(Self::Ice),
            "Key" => Ok(Self::Key),
            "Mine" => Ok(Self::Mine),
            "Player" => Ok(Self::Player),
            "PurpleBlock" => Ok(Self::PurpleBlock),
            "PurplePaint" => Ok(Self::PurplePaint),
            "Raft" => Ok(Self::Raft),
            "RedBlock" => Ok(Self::RedBlock),
            "RedPaint" => Ok(Self::RedPaint),
            "Teleporter" => Ok(Self::Teleporter),
            "Transporter" => Ok(Self::Transporter),
            "Water" => Ok(Self::Water),
            "YellowBlock" => Ok(Self::YellowBlock),
            _ => Err(UnknownObjectType),
        }
    }
}

impl ObjectType {
    /// Returns the object type this turns into when mixed with another.
    ///
    /// Only used for mixing of paint.
    pub fn mix_with(self, other: ObjectType) -> Option<Self> {
        match (self, other) {
            (Self::BluePaint, Self::BluePaint) => Some(Self::BluePaint),
            (Self::BluePaint, Self::PurplePaint) => Some(Self::PurplePaint),
            (Self::BluePaint, Self::RedPaint) => Some(Self::PurplePaint),
            (Self::PurplePaint, Self::BluePaint) => Some(Self::PurplePaint),
            (Self::PurplePaint, Self::PurplePaint) => Some(Self::PurplePaint),
            (Self::PurplePaint, Self::RedPaint) => Some(Self::PurplePaint),
            (Self::RedPaint, Self::BluePaint) => Some(Self::PurplePaint),
            (Self::RedPaint, Self::PurplePaint) => Some(Self::PurplePaint),
            (Self::RedPaint, Self::RedPaint) => Some(Self::RedPaint),
            _ => None,
        }
    }
}

pub fn spawn_object_of_type(
    cb: &mut ChildBuilder,
    assets: &GameObjectAssets,
    fonts: &Fonts,
    object_type: ObjectType,
    initial_position: InitialPositionAndMetadata,
) {
    match object_type {
        ObjectType::BlueBlock => cb.spawn(BlueBlock::spawn(assets, initial_position)),
        ObjectType::BluePaint => cb.spawn(BluePaint::spawn(assets, initial_position)),
        ObjectType::BouncingBall => cb.spawn(BouncingBall::spawn(assets, initial_position)),
        ObjectType::Button => cb.spawn(Button::spawn(assets, initial_position)),
        ObjectType::Creature1 => cb.spawn(Creature1::spawn(assets, initial_position)),
        ObjectType::Door => Door::spawn(cb, assets, initial_position),
        ObjectType::Entrance => Entrance::spawn(cb, assets, fonts, initial_position),
        ObjectType::Exit => cb.spawn(Exit::spawn(assets, initial_position)),
        ObjectType::Explosion => cb.spawn(Explosion::spawn(assets, initial_position)),
        ObjectType::Flash => cb.spawn(Flash::spawn(assets, initial_position)),
        ObjectType::Gate => Gate::spawn(cb, assets, initial_position),
        ObjectType::Grave => cb.spawn(Grave::spawn(assets, initial_position)),
        ObjectType::Ice => cb.spawn(Ice::spawn(assets, initial_position)),
        ObjectType::Key => cb.spawn(Key::spawn(assets, initial_position)),
        ObjectType::Mine => cb.spawn(Mine::spawn(assets, initial_position)),
        ObjectType::Player => cb.spawn(Player::spawn(assets, initial_position)),
        ObjectType::PurpleBlock => cb.spawn(PurpleBlock::spawn(assets, initial_position)),
        ObjectType::PurplePaint => cb.spawn(PurplePaint::spawn(assets, initial_position)),
        ObjectType::Raft => cb.spawn(Raft::spawn(assets, initial_position)),
        ObjectType::RedBlock => cb.spawn(RedBlock::spawn(assets, initial_position)),
        ObjectType::RedPaint => cb.spawn(RedPaint::spawn(assets, initial_position)),
        ObjectType::Splash => cb.spawn(Splash::spawn(assets, initial_position)),
        ObjectType::Teleporter => cb.spawn(Teleporter::spawn(assets, initial_position)),
        ObjectType::Transporter => cb.spawn(Transporter::spawn(assets, initial_position)),
        ObjectType::Water => cb.spawn(Water::spawn(assets, initial_position)),
        ObjectType::YellowBlock => cb.spawn(YellowBlock::spawn(assets, initial_position)),
    };
}
