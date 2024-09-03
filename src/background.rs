use bevy::prelude::*;

use crate::{
    constants::*, editor::Editor, level::Dimensions, load_level, menu::MenuState, on_resize,
    utils::load_repeating_asset, Levels, Player, Position, Zoom, INITIAL_HUB_FOCUS,
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
            .add_systems(Update, resize_background.after(load_level).after(on_resize))
            .observe(update_background_transform);
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

    commands.spawn((
        Background,
        SpriteBundle {
            texture: asset.background.clone(),
            ..Default::default()
        },
    ));
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

    commands.trigger(UpdateBackgroundTransform);
}

#[allow(clippy::too_many_arguments)]
fn update_background_transform(
    _trigger: Trigger<UpdateBackgroundTransform>,
    mut background_query: Query<&mut Transform, With<Background>>,
    editor_query: Query<Entity, With<Editor>>,
    player_query: Query<&Position, With<Player>>,
    window_query: Query<&Window>,
    dimensions: Res<Dimensions>,
    levels: Res<Levels>,
    menu_state: Res<MenuState>,
    zoom: Res<Zoom>,
) {
    let (focus_x, focus_y) = if menu_state.is_open && levels.current_level == 0 {
        (INITIAL_HUB_FOCUS.0 as f32, INITIAL_HUB_FOCUS.1 as f32)
    } else if let Ok(player_position) = player_query.get_single() {
        (player_position.x as f32, player_position.y as f32)
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

    transform.scale = Vec3::new(zoom.factor, zoom.factor, 1.);

    let editor_open = editor_query.get_single().is_ok();
    let editor_width = if editor_open { EDITOR_WIDTH as f32 } else { 0. };
    let level_width = (dimensions.width * GRID_SIZE) as f32 * zoom.factor;
    let x = if level_width > window_size.x - editor_width {
        let max = 0.5 * (level_width - (window_size.x - editor_width));
        (zoom.factor * ((-focus_x + 0.5 * dimensions.width as f32) + 0.5) * GRID_SIZE as f32)
            .clamp(-max, max)
    } else {
        0.
    };
    let level_height = (dimensions.height * GRID_SIZE) as f32 * zoom.factor;
    let y = if level_height > window_size.y {
        let max = 0.5 * (level_height - window_size.y);
        (zoom.factor * ((focus_y - 0.5 * dimensions.height as f32) - 0.5) * GRID_SIZE as f32)
            .clamp(-max, max)
    } else {
        0.
    };
    transform.translation = Vec3::new(x - if editor_open { 0.5 * editor_width } else { 0. }, y, 1.);
}
