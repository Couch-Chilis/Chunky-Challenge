use bevy::prelude::*;

use super::components::*;

pub type CollisionObjectQuery<'a> = (
    Option<Mut<'a, BlocksMovement>>,
    Option<&'a BlocksPushes>,
    Option<&'a Key>,
    Option<&'a Massive>,
    Option<&'a Mixable>,
    Option<&'a Openable>,
    Option<&'a Paint>,
    Option<&'a Paintable>,
    Mut<'a, Position>,
    Option<&'a Pushable>,
    Option<&'a Weight>,
);

pub struct CollisionObject<'a> {
    pub(super) blocks_movement: Option<&'a mut BlocksMovement>,
    pub blocks_pushes: Option<&'a BlocksPushes>,
    pub key: Option<&'a Key>,
    pub massive: Option<&'a Massive>,
    pub(super) mixable: Option<&'a Mixable>,
    pub openable: Option<&'a Openable>,
    pub paint: Option<&'a Paint>,
    pub paintable: Option<&'a Paintable>,
    pub(super) position: &'a mut Position,
    pub pushable: Option<&'a Pushable>,
    pub weight: Option<&'a Weight>,
}

impl<'a> From<CollisionObjectQuery<'a>> for CollisionObject<'a> {
    fn from(query: CollisionObjectQuery<'a>) -> Self {
        let (
            blocks_movement,
            blocks_pushes,
            key,
            massive,
            mixable,
            openable,
            paint,
            paintable,
            position,
            pushable,
            weight,
        ) = query;

        Self {
            blocks_movement: blocks_movement.map(|blocks_movement| blocks_movement.into_inner()),
            blocks_pushes,
            key,
            massive,
            mixable,
            openable,
            paint,
            paintable,
            position: position.into_inner(),
            pushable,
            weight,
        }
    }
}

impl<'a> From<&'a mut CollisionObjectQuery<'a>> for CollisionObject<'a> {
    fn from(query: &'a mut CollisionObjectQuery<'a>) -> Self {
        let (
            blocks_movement,
            blocks_pushes,
            key,
            massive,
            mixable,
            openable,
            paint,
            paintable,
            position,
            pushable,
            weight,
        ) = query;

        Self {
            blocks_movement: blocks_movement
                .as_mut()
                .map(|blocks_movement| blocks_movement.as_mut()),
            blocks_pushes: *blocks_pushes,
            key: *key,
            massive: *massive,
            mixable: *mixable,
            openable: *openable,
            paint: *paint,
            paintable: *paintable,
            position: position.as_mut(),
            pushable: *pushable,
            weight: *weight,
        }
    }
}

impl<'a> CollisionObject<'a> {
    pub fn blocks_movement(&self) -> bool {
        self.blocks_movement
            .as_ref()
            .is_some_and(|blocks| **blocks == BlocksMovement::Enabled)
    }

    pub fn can_mix_with(&self, other: &Mixable) -> bool {
        self.mixable.is_some_and(|mixable| mixable == other)
    }

    pub fn can_open_with_key(&self) -> bool {
        matches!(self.openable, Some(&Openable::Key))
    }

    pub fn can_push_on(&self) -> bool {
        self.massive.is_none() && self.pushable.is_none() && self.blocks_pushes.is_none()
    }

    pub fn has_position(&self, position: Position) -> bool {
        self.position == &position
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
        self.paint.is_some()
    }

    pub fn weight(&self) -> Weight {
        self.weight.copied().unwrap_or_default()
    }
}
