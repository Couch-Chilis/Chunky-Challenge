use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

#[derive(Resource)]
pub struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

impl Deref for AnimationTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AnimationTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource)]
pub struct MovementTimer(Timer);

impl Default for MovementTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

impl Deref for MovementTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MovementTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource)]
pub struct PlayerMovementTimer(Timer);

impl Default for PlayerMovementTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Once))
    }
}

impl Deref for PlayerMovementTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlayerMovementTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource)]
pub struct TemporaryTimer(Timer);

impl Default for TemporaryTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.25, TimerMode::Once))
    }
}

impl Deref for TemporaryTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TemporaryTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource)]
pub struct TransporterTimer(Timer);

impl Default for TransporterTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

impl Deref for TransporterTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TransporterTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
