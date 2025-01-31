use bevy::prelude::*;

#[derive(Resource)]
pub struct UiState {
    pub camera_offset: (f32, f32),
    pub drag_start: Option<(f32, f32)>,
    pub zoom_factor: f32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            camera_offset: (0., 0.),
            drag_start: None,
            zoom_factor: 1.,
        }
    }
}
