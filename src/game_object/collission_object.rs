use bevy::prelude::*;

use super::{components::*, ObjectType};

pub type CollisionObjectQuery<'a> = (
    Option<Mut<'a, BlocksMovement>>,
    Option<&'a BlocksPushes>,
    Option<&'a Key>,
    Option<&'a Massive>,
    &'a ObjectType,
    Option<&'a Openable>,
    Option<&'a Paint>,
    Option<&'a Paintable>,
    Mut<'a, Position>,
    Option<&'a Pushable>,
    Option<&'a Weight>,
);

pub struct CollisionObject<'a> {
    pub(super) blocks_movement: Option<Mut<'a, BlocksMovement>>,
    blocks_pushes: Option<&'a BlocksPushes>,
    key: Option<&'a Key>,
    massive: Option<&'a Massive>,
    object_type: &'a ObjectType,
    openable: Option<&'a Openable>,
    paint: Option<&'a Paint>,
    paintable: Option<&'a Paintable>,
    pub(super) position: Mut<'a, Position>,
    pushable: Option<&'a Pushable>,
    weight: Option<&'a Weight>,
}

impl<'a> From<CollisionObjectQuery<'a>> for CollisionObject<'a> {
    fn from(query: CollisionObjectQuery<'a>) -> Self {
        let (
            blocks_movement,
            blocks_pushes,
            key,
            massive,
            object_type,
            openable,
            paint,
            paintable,
            position,
            pushable,
            weight,
        ) = query;

        Self {
            blocks_movement,
            blocks_pushes,
            key,
            massive,
            object_type,
            openable,
            paint,
            paintable,
            position,
            pushable,
            weight,
        }
    }
}

impl<'a> CollisionObject<'a> {
    pub fn blocks_movement(&self) -> bool {
        self.blocks_movement
            .as_ref()
            .is_some_and(|blocks| **blocks == BlocksMovement::Enabled)
    }

    pub fn can_mix_with(&self, other: ObjectType) -> bool {
        self.object_type.mix_with(other).is_some()
    }

    pub fn can_open_with_key(&self) -> bool {
        matches!(self.openable, Some(&Openable::Key))
    }

    pub fn can_push_on(&self) -> bool {
        self.massive.is_none() && self.pushable.is_none() && self.blocks_pushes.is_none()
    }

    pub fn has_position(&self, position: Position) -> bool {
        self.position.as_ref() == &position
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
