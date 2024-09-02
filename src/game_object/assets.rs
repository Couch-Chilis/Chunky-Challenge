use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::{BevyDefault, ImageSampler},
    },
};

use crate::utils::load_asset;

pub const PLAYER_ASSET: &[u8] = include_bytes!("../../assets/sprites/player.png");

#[derive(Clone, Default, Resource)]
pub struct GameObjectAssets {
    pub black_fill: Handle<Image>,
    pub blue_block: Handle<Image>,
    pub bouncing_ball: Handle<Image>,
    pub bouncing_ball_editor: (Handle<Image>, Handle<TextureAtlasLayout>),
    pub button: Handle<Image>,
    pub creature1: (Handle<Image>, Handle<TextureAtlasLayout>),
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
            black_fill: images.add(black_pixel()),
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

fn black_pixel() -> Image {
    let format = TextureFormat::bevy_default();
    let data = vec![0, 0, 0, 255];
    Image {
        data,
        texture_descriptor: TextureDescriptor {
            size: Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            format,
            dimension: TextureDimension::D2,
            label: None,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        },
        sampler: ImageSampler::Default,
        texture_view_descriptor: None,
        asset_usage: RenderAssetUsages::default(),
    }
}
