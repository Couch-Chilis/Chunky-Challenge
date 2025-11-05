use bevy::prelude::*;

#[derive(Component)]
pub struct ControlArrow;

pub fn setup_overlays() {}

pub fn is_in_overlay(_position: Vec2, _window: &Window) -> bool {
    false
}

pub fn check_for_menu_button_interaction() {}

pub fn check_for_overlay_visibility() {}
