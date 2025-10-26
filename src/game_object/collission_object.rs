use bevy::prelude::*;

use super::{ObjectType, components::*};

pub type CollisionObjectQuery<'a> = (
    Option<Mut<'a, BlocksMovement>>,
    Option<&'a BlocksPushes>,
    Option<&'a Deadly>,
    Mut<'a, Direction>,
    Option<&'a Key>,
    Option<&'a Massive>,
    &'a ObjectType,
    Option<&'a Openable>,
    Option<&'a Paint>,
    Option<&'a Paintable>,
    Option<&'a Player>,
    Mut<'a, Position>,
    Option<&'a Pushable>,
    Option<&'a Weight>,
);

pub struct CollisionObject<'a> {
    pub(super) blocks_movement: Option<Mut<'a, BlocksMovement>>,
    blocks_pushes: Option<&'a BlocksPushes>,
    deadly: Option<&'a Deadly>,
    pub(super) direction: Mut<'a, Direction>,
    key: Option<&'a Key>,
    massive: Option<&'a Massive>,
    object_type: &'a ObjectType,
    openable: Option<&'a Openable>,
    paint: Option<&'a Paint>,
    paintable: Option<&'a Paintable>,
    player: Option<&'a Player>,
    pub(super) position: Mut<'a, Position>,
    pushable: Option<&'a Pushable>,
    pub(super) weight: Option<&'a Weight>,
}

impl<'a> From<CollisionObjectQuery<'a>> for CollisionObject<'a> {
    fn from(query: CollisionObjectQuery<'a>) -> Self {
        let (
            blocks_movement,
            blocks_pushes,
            deadly,
            direction,
            key,
            massive,
            object_type,
            openable,
            paint,
            paintable,
            player,
            position,
            pushable,
            weight,
        ) = query;

        Self {
            blocks_movement,
            blocks_pushes,
            deadly,
            direction,
            key,
            massive,
            object_type,
            openable,
            paint,
            paintable,
            player,
            position,
            pushable,
            weight,
        }
    }
}

impl CollisionObject<'_> {
    pub fn blocks_movement(&self) -> bool {
        self.blocks_movement
            .as_ref()
            .is_some_and(|blocks| **blocks == BlocksMovement::Enabled)
    }

    pub fn blocks_pushes(&self) -> bool {
        self.blocks_pushes.is_some()
    }

    pub fn can_mix_with(&self, other: ObjectType) -> bool {
        self.object_type.mix_with(other).is_some()
    }

    pub fn can_open_with_key(&self) -> bool {
        matches!(self.openable, Some(&Openable::Key))
    }

    pub fn can_push_on(&self) -> bool {
        self.massive.is_none() && self.pushable.is_none() && !self.blocks_pushes()
    }

    pub fn has_position(&self, position: Position) -> bool {
        self.position.as_ref() == &position
    }

    pub fn is_deadly(&self) -> bool {
        self.deadly.is_some()
    }

    pub fn is_key(&self) -> bool {
        self.key.is_some()
    }

    pub fn is_massive(&self) -> bool {
        self.massive.is_some()
    }

    pub fn is_paint(&self) -> bool {
        self.paint.is_some()
    }

    pub fn is_paintable(&self) -> bool {
        self.paintable.is_some()
    }

    pub fn is_player(&self) -> bool {
        self.player.is_some()
    }

    pub fn is_pushable(&self) -> bool {
        self.pushable.is_some()
    }

    pub fn object_type(&self) -> ObjectType {
        *self.object_type
    }

    pub fn weight(&self) -> Weight {
        self.weight.copied().unwrap_or_default()
    }
}
