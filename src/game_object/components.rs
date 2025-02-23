use std::{fmt::Display, str::FromStr};

use bevy::prelude::*;

use crate::errors::UnknownDirection;

use super::ObjectType;

/// Game object position.
///
/// The top-left square of a level is position (1, 1).
#[derive(Clone, Component, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Position {
    /// 1-based X coordinate of the object's position.
    pub x: i16,

    /// 1-based Y coordinate of the object's position.
    pub y: i16,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{}", self.x, self.y))
    }
}

impl From<(i16, i16)> for Position {
    fn from((x, y): (i16, i16)) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Component, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Up => "Up",
            Self::Right => "Right",
            Self::Down => "Down",
            Self::Left => "Left",
        })
    }
}

impl Direction {
    pub fn inverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    pub fn left_hand(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    pub fn right_hand(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    /// Returns a `(dx, dy)` tuple for the direction.
    pub fn as_delta(self) -> (i16, i16) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = UnknownDirection;

    fn from_str(direction: &str) -> Result<Self, Self::Err> {
        match direction {
            "Up" => Ok(Self::Up),
            "Right" => Ok(Self::Right),
            "Down" => Ok(Self::Down),
            "Left" => Ok(Self::Left),
            _ => Err(UnknownDirection),
        }
    }
}

impl TryFrom<(i16, i16)> for Direction {
    type Error = ();

    fn try_from((dx, dy): (i16, i16)) -> Result<Self, Self::Error> {
        match (dx, dy) {
            (0, -1) => Ok(Self::Up),
            (1, 0) => Ok(Self::Right),
            (0, 1) => Ok(Self::Down),
            (-1, 0) => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

#[derive(Component, Debug)]
pub struct Animatable {
    pub num_frames: usize,
}

/// An entity that prevents the [Player] as well as other [Movable] entities
/// from moving when on the same [Position].
///
/// Can be temporarily disabled. This is used for transporters and slippery
/// entities, which will temporarily stop blocking movement of objects it cannot
/// push further.
#[derive(Clone, Component, Copy, Debug, Default, Eq, PartialEq)]
pub enum BlocksMovement {
    #[default]
    Enabled,
    Disabled,
}

/// A non-[Massive] entity that rejects being pushed on.
#[derive(Component, Debug)]
pub struct BlocksPushes;

/// A deadly entity will kill the player if it comes into contact with it.
#[derive(Component, Debug)]
pub struct Deadly;

/// An entrance to another level.
#[derive(Component, Debug)]
pub struct Entrance(pub u16);

/// An exit completes the level when stepped on.
#[derive(Component, Debug)]
pub struct Exit;

/// Explodes on contact.
///
/// Should not be combined with [Deadly]. Dying is implied if the player
/// explodes.
#[derive(Component, Debug)]
pub struct Explosive;

/// A floatable entity will not sink when it comes into contact with a liquid.
#[derive(Component, Debug)]
pub struct Floatable;

/// Cannot be moved, not even when on a transporter.
#[derive(Component, Debug)]
pub struct Immovable;

/// Entity acts as a key for opening [Openable::Key] entities.
#[derive(Component, Debug)]
pub struct Key;

/// Liquid entities will cause other entities to sink when it comes into
/// contact with them. An exception are [Floatable] entities.
///
/// Should not be combined with [Deadly]. Dying is implied if the player
/// sinks.
#[derive(Component, Debug)]
pub struct Liquid;

/// A massive entity will prevent other entities from moving onto it.
///
/// An entity that is both massive and [Pushable] can still be pushed, but will
/// prevent other entities from moving onto it when it cannot be pushed further.
#[derive(Component, Debug)]
pub struct Massive;

/// Movable entities move by themselves.
///
/// They face a given [Direction], while the [Movable] variant decides what will
/// be their next direction.
#[derive(Component, Debug)]
pub enum Movable {
    /// Bounces back in the opposite direction whenever they cannot move further
    /// in their current direction.
    Bounce,

    /// Turns right whenever they can, while following whatever obstacles they
    /// have on their right.
    FollowRightHand,
}

/// A [Massive] entity that can be opened externally.
#[derive(Component, Debug)]
pub enum Openable {
    /// Entity opens when a key is pushed onto it.
    Key,

    /// Entity opens when the given level is finished.
    LevelFinished(u16),

    /// Entity opens when a [Trigger] is pressed.
    Trigger,
}

/// Entity is controlled by the player.
#[derive(Component, Debug)]
pub struct Player;

/// Entity that can paint [Paintable] entities.
///
/// Painting transforms the paintable entity into the given [ObjectType].
#[derive(Component, Debug)]
pub struct Paint(pub ObjectType);

/// An entity that can be [Paint]ed.
#[derive(Component, Debug)]
pub struct Paintable;

/// A movable entity will be "pushed" if possible when another entity attempts
/// to move onto it.
///
/// Pushable entities can only be pushed by other entities of equal or more
/// weight.
#[derive(Component, Debug)]
pub struct Pushable;

/// When an entity with a [Direction] gets onto a slippery entity, it keeps
/// sliding in that direction until it's no longer on a slippery entity or
/// cannot move further.
#[derive(Component, Debug)]
pub struct Slippery;

/// After pushing, entity transforms into another of the given type.
#[derive(Component, Debug)]
pub struct TransformOnPush(pub ObjectType);

/// Entity that can transport other entities to another teleporter.
///
/// Teleporters are bi-directional and the target teleporter is the one with the
/// same identifier.
#[derive(Component, Debug, Eq, PartialEq)]
pub struct Teleporter(pub u16);

/// Entity pushes all other entities that are placed on it towards a given
/// [Direction].
///
/// This is not limited to [Pushable] entities, although the behavior for
/// pushing uses the same constraints as for pushing [Pushable] entities.
#[derive(Component, Debug)]
pub struct Transporter;

/// Entity acts as trigger for opening [Openable::Trigger] entities.
#[derive(Component, Debug)]
pub struct Trigger;

/// Automatically disappears after spawning.
#[derive(Component, Debug)]
pub struct Volatile;

/// Weight of an entity.
///
/// Pushable entities can only be pushed by other entities of equal or more
/// weight.
#[derive(Clone, Component, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Weight {
    #[default]
    None,
    Light,
    Heavy,
}
