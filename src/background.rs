use std::time::Duration;

use bevy::prelude::*;

use crate::{
    constants::*, editor::EditorState, levels::*, load_level, menu::MenuState, on_player_moved,
    on_resize, ui_state::UiState, utils::load_repeating_asset, ExitState, LoadLevel, Player,
    Position,
};

const BACKGROUND_ASSET: &[u8] = include_bytes!("../assets/sprites/background.png");

const INITIAL_HUB_FOCUS: (i16, i16) = (33, 26);
const INITIAL_HUB_ZOOM_FACTOR: f32 = 0.32768;

#[derive(Component)]
pub struct Background;

#[derive(Default, Resource)]
pub enum BackgroundTransformAnimation {
    #[default]
    Paused,
    Active {
        scale_curve: EasingCurve<Vec3>,
        translation_curve: EasingCurve<Vec3>,
        timer: Timer,
    },
}

#[derive(Eq, Event, Ord, PartialEq, PartialOrd)]
pub enum UpdateBackgroundTransform {
    Immediate,
    Fast,
    HubIntro,
    LevelExit,
    LevelEntrance,
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background)
            .init_resource::<BackgroundAsset>()
            .init_resource::<BackgroundTransformAnimation>()
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
            )
            .add_systems(
                Update,
                on_background_transform_animation.after(on_update_background_transform),
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

    commands.send_event(UpdateBackgroundTransform::Immediate);
}

#[expect(clippy::too_many_arguments)]
fn on_update_background_transform(
    mut reader: EventReader<UpdateBackgroundTransform>,
    mut background_query: Query<&mut Transform, With<Background>>,
    mut animation: ResMut<BackgroundTransformAnimation>,
    player_query: Query<&Position, With<Player>>,
    window_query: Query<&Window>,
    dimensions: Res<Dimensions>,
    editor_state: Res<EditorState>,
    menu_state: Res<MenuState>,
    ui_state: Res<UiState>,
) {
    let event = reader.read().reduce(|slowest, event| event.max(slowest));
    let duration_ms = match event {
        Some(UpdateBackgroundTransform::Immediate) => 0,
        Some(UpdateBackgroundTransform::Fast) => 200,
        Some(UpdateBackgroundTransform::LevelExit | UpdateBackgroundTransform::LevelEntrance) => {
            400
        }
        Some(UpdateBackgroundTransform::HubIntro) => 2000,
        None => return,
    };

    let Ok(player_position) = player_query.get_single() else {
        return;
    };
    let focus_position = if menu_state.is_in_hub_menu() {
        (INITIAL_HUB_FOCUS.0, INITIAL_HUB_FOCUS.1)
    } else {
        (player_position.x, player_position.y)
    };

    let mut transform = background_query
        .get_single_mut()
        .expect("there should be only one background");
    let window = window_query
        .get_single()
        .expect("there should be only one window");

    let window_size = window.size();
    let zoom_factor = if menu_state.is_in_hub_menu() {
        INITIAL_HUB_ZOOM_FACTOR
    } else if event == Some(&UpdateBackgroundTransform::LevelExit) {
        (window_size.x / GRID_SIZE as f32).max(window_size.y / GRID_SIZE as f32)
    } else {
        ui_state.zoom_factor
    };
    let (scale, translation) = calculate_background_transform_with_zoom_factor(
        &dimensions,
        &editor_state,
        focus_position,
        &ui_state,
        window_size,
        zoom_factor,
    );

    if duration_ms > 0 {
        if event == Some(&UpdateBackgroundTransform::LevelEntrance) {
            let (start_scale, start_translation) = calculate_background_transform_with_zoom_factor(
                &dimensions,
                &editor_state,
                (player_position.x, player_position.y),
                &ui_state,
                window_size,
                (window_size.x / GRID_SIZE as f32).max(window_size.y / GRID_SIZE as f32),
            );
            *transform = Transform::from_scale(start_scale).with_translation(start_translation);
        }

        *animation = BackgroundTransformAnimation::Active {
            scale_curve: EasingCurve::new(
                transform.scale,
                scale,
                match event {
                    Some(UpdateBackgroundTransform::LevelExit) => EaseFunction::QuarticIn,
                    _ => EaseFunction::QuarticOut,
                },
            ),
            translation_curve: EasingCurve::new(
                transform.translation,
                translation,
                match event {
                    Some(UpdateBackgroundTransform::HubIntro) => EaseFunction::QuarticInOut,
                    Some(UpdateBackgroundTransform::LevelExit) => EaseFunction::QuarticIn,
                    _ => EaseFunction::QuarticOut,
                },
            ),
            timer: Timer::new(Duration::from_millis(duration_ms), TimerMode::Once),
        };
    } else {
        *transform = Transform::from_scale(scale).with_translation(translation);
        *animation = BackgroundTransformAnimation::Paused;
    }
}

fn on_background_transform_animation(
    mut commands: Commands,
    mut background_query: Query<&mut Transform, With<Background>>,
    mut animation: ResMut<BackgroundTransformAnimation>,
    exit_state: Res<ExitState>,
    time: Res<Time<Virtual>>,
) {
    let BackgroundTransformAnimation::Active {
        scale_curve,
        translation_curve,
        timer,
    } = animation.as_mut()
    else {
        return;
    };

    timer.tick(time.delta());

    let mut transform = background_query
        .get_single_mut()
        .expect("there should be only one background");

    let t = timer.fraction();
    *transform = Transform::from_scale(scale_curve.sample_unchecked(t))
        .with_translation(translation_curve.sample_unchecked(t));

    if timer.finished() {
        *animation = BackgroundTransformAnimation::Paused;

        if let Some(next_level) = exit_state.next_level {
            commands.trigger(LoadLevel(next_level));
        }
    }
}

fn calculate_background_transform_with_zoom_factor(
    dimensions: &Dimensions,
    editor_state: &EditorState,
    (focus_x, focus_y): (i16, i16),
    ui_state: &UiState,
    window_size: Vec2,
    zoom_factor: f32,
) -> (Vec3, Vec3) {
    let scale = Vec3::new(zoom_factor, zoom_factor, 1.);

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
    let translation = Vec3::new(x - if editor_open { 0.5 * editor_width } else { 0. }, y, 1.);

    (scale, translation)
}
