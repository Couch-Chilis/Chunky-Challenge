use std::{fs, path::PathBuf};

use bevy::{
    image::{
        CompressedImageFormats, ImageAddressMode, ImageSampler, ImageSamplerDescriptor, ImageType,
    },
    prelude::*,
    render::render_asset::RenderAssetUsages,
};

use crate::{levels::Dimensions, GRID_SIZE};

pub fn level_coords_from_pointer_coords(
    coords: Vec2,
    dimensions: Dimensions,
    transform: &Transform,
    window_size: Vec2,
) -> (f32, f32) {
    let center_x = 0.5 * window_size.x + transform.translation.x;
    let x = ((coords.x - center_x) / (transform.scale.x * GRID_SIZE as f32)
        + 0.5 * dimensions.width as f32)
        + 1.;

    let center_y = 0.5 * window_size.y - transform.translation.y;
    let y = ((coords.y - center_y) / (transform.scale.y * GRID_SIZE as f32)
        + 0.5 * dimensions.height as f32)
        + 1.;

    (x, y)
}

pub fn ensure_chunky_dir() -> PathBuf {
    #[expect(deprecated)]
    let parent_dir = std::env::home_dir().unwrap_or(PathBuf::from("/tmp"));

    let chunky_dir = parent_dir.join(if cfg!(target_os = "ios") {
        "Library/Application support"
    } else {
        ".chunky"
    });
    if chunky_dir.exists() {
        return chunky_dir;
    }

    match fs::create_dir_all(&chunky_dir) {
        Ok(()) => chunky_dir,
        Err(err) => {
            warn!("Falling back to parent dir ({parent_dir:?}): {err:?}");
            parent_dir
        }
    }
}

pub fn get_level_path(level_number: u16) -> String {
    format!("assets/levels/level{level_number:0>3}")
}

pub fn load_asset(bytes: &[u8]) -> Image {
    Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::all(),
        true,
        ImageSampler::Default,
        RenderAssetUsages::all(),
    )
    .expect("cannot load game object asset")
}

pub fn load_repeating_asset(bytes: &[u8]) -> Image {
    Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::all(),
        true,
        ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            ..default()
        }),
        RenderAssetUsages::all(),
    )
    .expect("cannot load game object asset")
}
