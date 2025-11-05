use bevy::prelude::*;

use crate::utils::load_asset;

#[allow(dead_code)] // Only used in mobile builds.
#[derive(Clone, Default, Resource)]
pub struct UiAssets {
    pub arrow_down: Handle<Image>,
    pub arrow_left: Handle<Image>,
    pub arrow_right: Handle<Image>,
    pub arrow_up: Handle<Image>,
}

impl UiAssets {
    pub fn load(images: &mut ResMut<Assets<Image>>) -> Self {
        Self {
            arrow_down: images.add(load_asset(include_bytes!("../../assets/ui/arrow_down.png"))),
            arrow_left: images.add(load_asset(include_bytes!("../../assets/ui/arrow_left.png"))),
            arrow_right: images.add(load_asset(include_bytes!(
                "../../assets/ui/arrow_right.png"
            ))),
            arrow_up: images.add(load_asset(include_bytes!("../../assets/ui/arrow_up.png"))),
        }
    }
}
