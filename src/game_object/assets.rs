use bevy::prelude::*;

use crate::utils::load_asset;

pub const PLAYER_ASSET: &[u8] = include_bytes!("../../assets/sprites/player.png");

#[derive(Clone, Default, Resource)]
pub struct GameObjectAssets {
    pub blue_block: Handle<Image>,
    pub bouncing_ball: Handle<Image>,
    pub bouncing_ball_editor: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub button: Handle<Image>,
    pub creature1: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub entrance: Handle<Image>,
    pub eraser: Handle<Image>,
    pub explosion: Handle<Image>,
    pub exit: Handle<Image>,
    pub gate: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub grave: Handle<Image>,
    pub mine: Handle<Image>,
    pub player: Handle<Image>,
    pub purple_block: Handle<Image>,
    pub raft: Handle<Image>,
    pub red_block: Handle<Image>,
    pub splash: Handle<Image>,
    pub transporter: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub water: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub yellow_block: Handle<Image>,
}

impl GameObjectAssets {
    pub fn load(
        images: &mut ResMut<Assets<Image>>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let one_by_two_atlas = {
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 1, 2, None, None);
            texture_atlas_layouts.add(layout)
        };
        let one_by_three_atlas = {
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 1, 3, None, None);
            texture_atlas_layouts.add(layout)
        };
        let one_by_four_atlas = {
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 1, 4, None, None);
            texture_atlas_layouts.add(layout)
        };

        Self {
            blue_block: images.add(load_asset(include_bytes!(
                "../../assets/sprites/blueblock.png"
            ))),
            bouncing_ball: images.add(load_asset(include_bytes!(
                "../../assets/sprites/greenball.png"
            ))),
            bouncing_ball_editor: (
                images.add(load_asset(include_bytes!(
                    "../../assets/sprites/greenball_editor.png"
                ))),
                one_by_four_atlas.clone(),
            ),
            button: images.add(load_asset(include_bytes!(
                "../../assets/sprites/button.png"
            ))),
            creature1: (
                images.add(load_asset(include_bytes!(
                    "../../assets/sprites/creature1.png"
                ))),
                one_by_four_atlas.clone(),
            ),
            entrance: images.add(load_asset(include_bytes!(
                "../../assets/sprites/entrance.png"
            ))),
            eraser: images.add(load_asset(include_bytes!(
                "../../assets/sprites/eraser.png"
            ))),
            exit: images.add(load_asset(include_bytes!("../../assets/sprites/exit.png"))),
            explosion: images.add(load_asset(include_bytes!(
                "../../assets/sprites/explosion.png"
            ))),
            gate: (
                images.add(load_asset(include_bytes!("../../assets/sprites/gate.png"))),
                one_by_two_atlas,
            ),
            grave: images.add(load_asset(include_bytes!("../../assets/sprites/grave.png"))),
            mine: images.add(load_asset(include_bytes!("../../assets/sprites/mine.png"))),
            player: images.add(load_asset(PLAYER_ASSET)),
            purple_block: images.add(load_asset(include_bytes!(
                "../../assets/sprites/purpleblock.png"
            ))),
            raft: images.add(load_asset(include_bytes!("../../assets/sprites/raft.png"))),
            red_block: images.add(load_asset(include_bytes!(
                "../../assets/sprites/redblock.png"
            ))),
            splash: images.add(load_asset(include_bytes!(
                "../../assets/sprites/splash.png"
            ))),
            transporter: (
                images.add(load_asset(include_bytes!(
                    "../../assets/sprites/transporter.png"
                ))),
                one_by_four_atlas,
            ),
            water: (
                images.add(load_asset(include_bytes!("../../assets/sprites/water.png"))),
                one_by_three_atlas,
            ),
            yellow_block: images.add(load_asset(include_bytes!(
                "../../assets/sprites/yellowblock.png"
            ))),
        }
    }
}
