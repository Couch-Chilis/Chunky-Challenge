use bevy::prelude::*;

#[derive(Resource)]
pub struct UiState {
    /// Offset of the camera relative to the player.
    pub camera_offset: Vec2,

    /// Start position where a drag of the map was initiated.
    pub drag_start: Option<Vec2>,

    pub pinch_state: Option<PinchState>,

    pub zoom_factor: f32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            camera_offset: Vec2::new(0., 0.),
            drag_start: None,
            pinch_state: None,
            zoom_factor: 1.,
        }
    }
}

/// State used for tracking pinch gestures for zooming the map.
#[derive(Clone, Copy)]
pub struct PinchState {
    /// Distance between the two touch points at the start of the pinch.
    pub initial_distance: f32,

    /// Zoom factor at the start of the pinch.
    pub initial_zoom_factor: f32,
}
