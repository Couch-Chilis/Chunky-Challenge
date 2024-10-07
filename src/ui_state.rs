use bevy::prelude::*;

pub const HUB_DEFAULT_ZOOM_FACTOR: f32 = 0.32768;
pub const LEVEL_DEFAULT_ZOOM_FACTOR: f32 = 1.;

#[derive(Default, Resource)]
pub struct UiState {
    pub camera_offset: (f32, f32),
    pub drag_start: Option<(f32, f32)>,
    pub zoom_factor: f32,
}
