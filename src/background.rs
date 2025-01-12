use bevy::prelude::*;

use crate::{
    constants::*, editor::EditorState, levels::*, load_level, menu::MenuState, on_player_moved,
    on_resize, ui_state::UiState, utils::load_repeating_asset, Player, Position, INITIAL_HUB_FOCUS,
};

const BACKGROUND_ASSET: &[u8] = include_bytes!("../assets/sprites/background.png");

#[derive(Component)]
pub struct Background;

#[derive(Event)]
pub struct UpdateBackgroundTransform;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background)
            .init_resource::<BackgroundAsset>()
            .add_event::<UpdateBackgroundTransform>()
            .add_systems(
                Update,
                resize_background
                    .after(load_level)
                    .after(on_resize)
                    .after(on_player_moved),
            )
            .add_systems(
                Update,
                on_update_background_transform.after(resize_background),
            );
    }
}

#[derive(Clone, Default, Resource)]
struct BackgroundAsset {
    pub background: Handle<Image>,
}

fn setup_background(
    mut commands: Commands,
    mut asset: ResMut<BackgroundAsset>,
    mut image_assets: ResMut<Assets<Image>>,
) {
    asset.background = image_assets.add(load_repeating_asset(BACKGROUND_ASSET));

    commands.spawn((Background, Sprite::from_image(asset.background.clone())));
}

fn resize_background(
    mut commands: Commands,
    mut background_query: Query<&mut Sprite, With<Background>>,
    dimensions: Res<Dimensions>,
) {
    if !dimensions.is_changed() {
        return;
    }

    let mut sprite = background_query
        .get_single_mut()
        .expect("there should be only one background");

    sprite.rect = Some(Rect::new(
        0.,
        0.,
        (dimensions.width * GRID_SIZE) as f32,
        (dimensions.height * GRID_SIZE) as f32,
    ));

    commands.send_event(UpdateBackgroundTransform);
}

#[expect(clippy::too_many_arguments)]
fn on_update_background_transform(
    mut reader: EventReader<UpdateBackgroundTransform>,
    mut background_query: Query<&mut Transform, With<Background>>,
    player_query: Query<&Position, With<Player>>,
    window_query: Query<&Window>,
    dimensions: Res<Dimensions>,
    editor_state: Res<EditorState>,
    menu_state: Res<MenuState>,
    ui_state: Res<UiState>,
) {
    let mut empty = true;
    for _event in reader.read() {
        empty = false;
    }
    if empty {
        return;
    }

    let (focus_x, focus_y) = if menu_state.is_in_hub_menu() {
        (INITIAL_HUB_FOCUS.0, INITIAL_HUB_FOCUS.1)
    } else if let Ok(player_position) = player_query.get_single() {
        (player_position.x, player_position.y)
    } else {
        return;
    };

    let mut transform = background_query
        .get_single_mut()
        .expect("there should be only one background");
    let window = window_query
        .get_single()
        .expect("there should be only one window");
    let window_size = window.size();

    let zoom_factor = ui_state.zoom_factor;
    transform.scale = Vec3::new(zoom_factor, zoom_factor, 1.);

    let editor_open = editor_state.is_open;
    let editor_width = if editor_open { EDITOR_WIDTH as f32 } else { 0. };
    let level_width = (dimensions.width * GRID_SIZE) as f32 * zoom_factor;
    let x = if level_width > window_size.x - editor_width {
        let max = 0.5 * (level_width - (window_size.x - editor_width));
        (zoom_factor * ((-focus_x as f32 + 0.5 * dimensions.width as f32) + 0.5) * GRID_SIZE as f32)
            .clamp(-max, max)
            - (zoom_factor * ui_state.camera_offset.0 * GRID_SIZE as f32)
    } else {
        0.
    };
    let level_height = (dimensions.height * GRID_SIZE) as f32 * zoom_factor;
    let y = if level_height > window_size.y {
        let max = 0.5 * (level_height - window_size.y);
        (zoom_factor * ((focus_y as f32 - 0.5 * dimensions.height as f32) - 0.5) * GRID_SIZE as f32)
            .clamp(-max, max)
            + (zoom_factor * ui_state.camera_offset.1 * GRID_SIZE as f32)
    } else {
        0.
    };
    transform.translation = Vec3::new(x - if editor_open { 0.5 * editor_width } else { 0. }, y, 1.);
}
