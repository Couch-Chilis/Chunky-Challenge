use bevy::prelude::*;

#[derive(Clone, Default, Resource)]
pub struct Fonts {
    pub poppins_light: Handle<Font>,
}
