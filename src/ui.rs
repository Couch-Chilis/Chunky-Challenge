#[cfg(any(target_os = "android", target_os = "ios"))]
#[path = "ui/controls_overlay.mobile.rs"]
mod controls_overlay;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[path = "ui/controls_overlay.desktop.rs"]
mod controls_overlay;

mod fonts;
mod gameover;
mod ui_assets;
mod ui_state;

pub mod menu;

pub use controls_overlay::*;
pub use fonts::*;
pub use gameover::*;
pub use ui_assets::*;
pub use ui_state::*;

use bevy::prelude::*;

use menu::MenuPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Fonts>()
            .init_resource::<UiAssets>()
            .init_resource::<UiState>()
            .add_plugins(MenuPlugin)
            .add_systems(Startup, setup_ui)
            .add_systems(PostStartup, (setup_controls_overlay, setup_gameover));
    }
}

fn setup_ui(
    mut fonts: ResMut<Fonts>,
    mut font_assets: ResMut<Assets<Font>>,
    mut image_assets: ResMut<Assets<Image>>,
    mut ui_assets: ResMut<UiAssets>,
) {
    *ui_assets.as_mut() = UiAssets::load(&mut image_assets);

    fonts.poppins_light = font_assets.add(
        Font::try_from_bytes(Vec::from(include_bytes!(
            "../assets/font/Poppins/Poppins-Light.ttf"
        )))
        .unwrap(),
    );
}
