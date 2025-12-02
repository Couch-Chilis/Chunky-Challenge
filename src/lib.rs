#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod background;
mod constants;
mod editor;
mod errors;
mod game_object;
mod game_state;
mod levels;
mod timers;
mod ui;
mod utils;

use std::{
    borrow::Cow,
    collections::BTreeMap,
    fs,
    ops::{Deref, DerefMut},
};

use bevy::{
    ecs::system::NonSendMarker,
    prelude::*,
    window::{PrimaryWindow, WindowMode, WindowResized, WindowResolution},
};
use image::ImageFormat;
use winit::window::Icon;

use crate::{
    background::{Background, BackgroundPlugin, UpdateBackgroundTransform},
    constants::*,
    editor::{
        EditorPlugin, EditorState, SelectionOverlay, ToggleEditor, on_editor_keyboard_input,
        on_editor_mouse_input,
    },
    game_object::{
        CollisionObjectQuery, Direction, DirectionalSprite, Entrance, GameObjectAssets, Massive,
        ObjectType, Openable, PLAYER_ASSET, Player, Position, Teleporter, Weight, behaviors::*,
        spawn_object_of_type,
    },
    game_state::GameState,
    levels::{Dimensions, InitialPositionAndMetadata, Level, Levels},
    timers::{
        AnimationTimer, MovementTimer, PlayerMovementTimer, TemporaryTimer, TransporterTimer,
    },
    ui::{
        ControlArrow, Fonts, PinchState, UiPlugin, UiState, check_for_game_over,
        check_for_overlay_visibility, is_in_overlay, menu::*,
    },
    utils::get_level_path,
};

#[derive(Default, Resource)]
struct ExitState {
    next_level_after_background_transform: Option<u16>,
}

#[derive(Default, Resource)]
struct PressedTriggers {
    num_pressed_triggers: usize,
}

#[derive(Default, Resource)]
struct PreviousDirection(Option<Direction>);

impl Deref for PreviousDirection {
    type Target = Option<Direction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PreviousDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Resource)]
struct QueuedDirection(Option<Direction>);

impl Deref for QueuedDirection {
    type Target = Option<Direction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for QueuedDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Event)]
struct ChangeZoom(f32);

#[derive(Message)]
enum GameEvent {
    MovePlayer(Direction),
}

/// Loads the given level.
#[derive(Event)]
struct LoadLevel(u16);

/// Loads the relative level.
///
/// The level to load is calculated by adding the given delta to the current
/// level. Commonly used to load the next/previous level by specifying 1/-1,
/// respectively. Also used to reload the current level using a delta of 0.
#[derive(Event)]
struct LoadRelativeLevel(i16);

/// Resets the current level.
///
/// Resetting differs from restarting (using `LoadRelativeLevel(0)`) because it
/// always resets to the version from disk and ignores what was saved in-memory.
#[derive(Event)]
struct ResetLevel;

#[derive(Event)]
struct SaveLevel {
    save_to_disk: bool,
}

#[derive(Event)]
struct SpawnObject {
    object_type: ObjectType,
    position: InitialPositionAndMetadata,
}

#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Chunky's Challenge".to_owned(),
                    mode: get_initial_window_mode(),

                    resolution: WindowResolution::new(DEFAULT_WINDOW_SIZE, DEFAULT_WINDOW_SIZE)
                        .with_scale_factor_override(1.),
                    ..default()
                }),
                ..default()
            }),
            BackgroundPlugin,
            #[cfg(debug_assertions)]
            EditorPlugin,
            UiPlugin,
        ))
        .init_resource::<AnimationTimer>()
        .init_resource::<Dimensions>()
        .init_resource::<EditorState>()
        .init_resource::<ExitState>()
        .init_resource::<GameObjectAssets>()
        .init_resource::<Levels>()
        .init_resource::<MovementTimer>()
        .init_resource::<PlayerMovementTimer>()
        .init_resource::<PressedTriggers>()
        .init_resource::<PreviousDirection>()
        .init_resource::<QueuedDirection>()
        .init_resource::<TemporaryTimer>()
        .init_resource::<TransporterTimer>()
        .insert_resource(GameState::load())
        .add_message::<GameEvent>()
        .add_observer(load_level)
        .add_observer(load_relative_level)
        .add_observer(on_zoom_change)
        .add_observer(reset_level)
        .add_observer(save_level)
        .add_observer(spawn_object)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, (set_window_icon, post_setup))
        .add_systems(
            Update,
            (
                on_keyboard_input,
                on_gamepad_input,
                on_player_movement,
                on_mouse_input,
                on_touch_input,
                on_resize,
            ),
        )
        .add_systems(
            Update,
            (
                animate_objects,
                check_for_overlay_visibility,
                check_for_deadly,
                check_for_entrance,
                check_for_exit,
                check_for_explosive,
                check_for_liquid,
                check_for_game_over,
                check_for_slippery_and_transporter,
                despawn_volatile_objects,
                on_game_event,
            )
                .after(on_keyboard_input)
                .after(on_gamepad_input),
        )
        .add_systems(
            Update,
            check_for_movable.after(check_for_slippery_and_transporter),
        )
        .add_systems(
            Update,
            (check_for_transform_on_push, check_for_triggers)
                .after(check_for_liquid)
                .after(check_for_movable),
        )
        .add_systems(
            Update,
            (
                check_for_finished_levels,
                check_for_key,
                check_for_paint,
                check_for_teleporter,
                on_player_moved,
            )
                .after(check_for_movable)
                .after(on_mouse_input)
                .after(on_touch_input),
        )
        .add_systems(
            Update,
            (position_entities, update_entity_directions)
                .after(check_for_explosive)
                .after(check_for_liquid)
                .after(check_for_paint)
                .after(check_for_teleporter)
                .after(check_for_transform_on_push),
        )
        .run();
}

fn get_initial_window_mode() -> WindowMode {
    if cfg!(target_os = "ios") || std::env::var_os("SteamTenfoot").is_some() {
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    } else {
        WindowMode::Windowed
    }
}

fn set_window_icon(_non_send_marker: NonSendMarker) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory_with_format(PLAYER_ASSET, ImageFormat::Png)
            .unwrap()
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    bevy::winit::WINIT_WINDOWS.with_borrow(|winit_windows| {
        for window in winit_windows.windows.values() {
            window.set_window_icon(Some(icon.clone()));
        }
    });
}

fn setup(
    mut commands: Commands,
    mut game_object_assets: ResMut<GameObjectAssets>,
    mut image_assets: ResMut<Assets<Image>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    *game_object_assets.as_mut() =
        GameObjectAssets::load(&mut image_assets, &mut texture_atlas_layouts);

    commands.spawn(Camera2d);
}

fn post_setup(mut commands: Commands) {
    commands.trigger(LoadLevel(0));
}

#[expect(clippy::too_many_arguments, clippy::type_complexity)]
pub fn on_mouse_input(
    mut commands: Commands,
    selection_query: Query<&mut Transform, With<SelectionOverlay>>,
    background_query: Query<(Entity, &Transform), (With<Background>, Without<SelectionOverlay>)>,
    objects: Query<(Entity, &ObjectType, &Position)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    editor_state: ResMut<EditorState>,
    mut ui_state: ResMut<UiState>,
    buttons: Res<ButtonInput<MouseButton>>,
    dimensions: Res<Dimensions>,
    menu_state: Res<MenuState>,
) -> Result<()> {
    if editor_state.is_open {
        return on_editor_mouse_input(
            commands,
            selection_query,
            background_query,
            objects,
            window_query,
            editor_state,
            buttons,
            dimensions,
        );
    } else if menu_state.is_open() {
        return Ok(());
    }

    if !buttons.pressed(MouseButton::Left) {
        if ui_state.drag_start.is_some() {
            ui_state.drag_start = None;
        }
        return Ok(());
    }

    let window = window_query.single()?;
    let Some(cursor_position) = window.cursor_position() else {
        return Ok(());
    };

    if ui_state.drag_start.is_none() && is_in_overlay(cursor_position, window) {
        return Ok(());
    }

    let zoom_factor = ui_state.zoom_factor;
    let x = cursor_position.x / (zoom_factor * GRID_SIZE as f32);
    let y = cursor_position.y / (zoom_factor * GRID_SIZE as f32);

    if let Some(drag_start) = ui_state.drag_start {
        let new_camera_offset = Vec2::new(drag_start.x - x, drag_start.y - y);
        if ui_state.camera_offset != new_camera_offset {
            ui_state.camera_offset.x += new_camera_offset.x;
            ui_state.camera_offset.y += new_camera_offset.y;
            commands.write_message(UpdateBackgroundTransform::Fast);
        }
    }

    ui_state.drag_start = Some(Vec2::new(x, y));

    Ok(())
}

pub fn on_touch_input(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    touches: Res<Touches>,
    menu_state: Res<MenuState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !touches.is_changed() || menu_state.is_open() {
        return;
    }

    let mut touches = touches.iter();
    let first = touches.next();
    let second = touches.next();
    let third = touches.next();

    if first.is_none() || second.is_some() {
        ui_state.drag_start = None;
    }
    if second.is_none() {
        ui_state.pinch_state = None;
    }

    let Some(first) = first else {
        return;
    };

    if ui_state.drag_start.is_none()
        && ui_state.pinch_state.is_none()
        && !window_query
            .single()
            .is_ok_and(|window| !is_in_overlay(first.position(), window))
    {
        return;
    }

    match (second, third) {
        (None, None) => {
            let zoom_factor = ui_state.zoom_factor;
            let x = first.position().x / (zoom_factor * GRID_SIZE as f32);
            let y = first.position().y / (zoom_factor * GRID_SIZE as f32);

            if let Some(drag_start) = ui_state.drag_start {
                let new_camera_offset = Vec2::new(drag_start.x - x, drag_start.y - y);
                if ui_state.camera_offset != new_camera_offset {
                    ui_state.camera_offset.x += new_camera_offset.x;
                    ui_state.camera_offset.y += new_camera_offset.y;
                    commands.write_message(UpdateBackgroundTransform::Fast);
                }
            }

            ui_state.drag_start = Some(Vec2::new(x, y));
        }
        (Some(second), None) => {
            let distance = first.position().distance(second.position());
            if distance > 0. {
                if let Some(pinch_state) = ui_state.pinch_state {
                    ui_state.zoom_factor =
                        pinch_state.initial_zoom_factor * (distance / pinch_state.initial_distance);
                } else {
                    ui_state.pinch_state = Some(PinchState {
                        initial_distance: distance,
                        initial_zoom_factor: ui_state.zoom_factor,
                    });
                }
            }
        }
        _ => {}
    }
}

fn on_gamepad_input(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut menu_state: ResMut<MenuState>,
    editor_state: ResMut<EditorState>,
    exit_state: Res<ExitState>,
    game_state: Res<GameState>,
    gamepads: Query<&Gamepad>,
) {
    if editor_state.is_open {
        return;
    } else if menu_state.is_open() {
        on_menu_gamepad_input(commands, menu_state, gamepads);
        return;
    } else if exit_state.next_level_after_background_transform.is_some() {
        return;
    }

    for gamepad in gamepads {
        for button in gamepad.get_just_pressed() {
            use GamepadButton::*;
            match button {
                South if player_query.single().is_err() => {
                    commands.trigger(LoadRelativeLevel(0));
                }
                RightTrigger2 => {
                    commands.trigger(ChangeZoom(1.25));
                }
                LeftTrigger2 => {
                    commands.trigger(ChangeZoom(0.8));
                }
                North => {
                    commands.trigger(LoadRelativeLevel(0));
                }
                Start => {
                    menu_state.set_open(if game_state.is_in_hub() {
                        MenuKind::Hub
                    } else {
                        MenuKind::Level
                    });
                }

                _ => continue,
            };
        }
    }
}

#[expect(clippy::too_many_arguments)]
fn on_keyboard_input(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut menu_state: ResMut<MenuState>,
    editor_state: ResMut<EditorState>,
    ui_state: ResMut<UiState>,
    exit_state: Res<ExitState>,
    game_state: Res<GameState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if editor_state.is_open {
        on_editor_keyboard_input(commands, editor_state, ui_state, keys);
        return;
    } else if menu_state.is_open() {
        on_menu_keyboard_input(commands, menu_state, keys);
        return;
    } else if exit_state.next_level_after_background_transform.is_some() {
        return;
    }

    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            Enter if player_query.single().is_err() => {
                commands.trigger(LoadRelativeLevel(0));
            }
            Equal => {
                commands.trigger(ChangeZoom(1.25));
            }
            Minus => {
                commands.trigger(ChangeZoom(0.8));
            }
            BracketRight => {
                commands.trigger(LoadRelativeLevel(1));
            }
            BracketLeft => {
                commands.trigger(LoadRelativeLevel(-1));
            }
            KeyE => {
                commands.trigger(ToggleEditor);
            }
            KeyR => {
                commands.trigger(LoadRelativeLevel(0));
            }
            Escape => {
                menu_state.set_open(if game_state.is_in_hub() {
                    MenuKind::Hub
                } else {
                    MenuKind::Level
                });
            }

            _ => continue,
        };
    }
}

#[expect(clippy::too_many_arguments)]
fn on_player_movement(
    mut queued_direction: ResMut<QueuedDirection>,
    mut timer: ResMut<PlayerMovementTimer>,
    mut game_events: MessageWriter<GameEvent>,
    menu_state: Res<MenuState>,
    editor_state: Res<EditorState>,
    exit_state: Res<ExitState>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    control_arrows: Query<(&Direction, Ref<Interaction>), With<ControlArrow>>,
    previous_direction: Res<PreviousDirection>,
    time: Res<Time>,
) {
    if editor_state.is_open
        || menu_state.is_open()
        || exit_state.next_level_after_background_transform.is_some()
    {
        return;
    }

    timer.tick(time.delta());

    if timer.just_finished() {
        timer.reset();

        if let Some(direction) = queued_direction.take() {
            game_events.write(GameEvent::MovePlayer(direction));
        } else if let (Some(first), second) =
            directions_from_inputs(control_arrows, gamepads, &keys, **previous_direction, false)
        {
            game_events.write(GameEvent::MovePlayer(first));

            if let Some(direction) = second {
                *queued_direction = QueuedDirection(Some(direction));
            }
        } else {
            timer.pause();
        }
    } else if timer.is_paused() {
        if let (Some(first), second) =
            directions_from_inputs(control_arrows, gamepads, &keys, **previous_direction, true)
        {
            game_events.write(GameEvent::MovePlayer(first));

            if let Some(direction) = second {
                *queued_direction = QueuedDirection(Some(direction));
            }

            timer.unpause();
        }
    } else if let (Some(direction), _) =
        directions_from_inputs(control_arrows, gamepads, &keys, **previous_direction, true)
    {
        *queued_direction = QueuedDirection(Some(direction));
    }
}

fn directions_from_gamepad(
    gamepad: &Gamepad,
    previous_direction: Option<Direction>,
    just_pressed: bool,
) -> (Option<Direction>, Option<Direction>) {
    let pressed = if just_pressed {
        Gamepad::just_pressed
    } else {
        Gamepad::pressed
    };

    let mut dx = 0;
    let mut dy = 0;
    if pressed(gamepad, GamepadButton::DPadUp) {
        dy -= 1;
    }
    if pressed(gamepad, GamepadButton::DPadRight) {
        dx += 1
    }
    if pressed(gamepad, GamepadButton::DPadDown) {
        dy += 1;
    }
    if pressed(gamepad, GamepadButton::DPadLeft) {
        dx -= 1
    }

    directions_from_deltas(dx, dy, previous_direction)
}

fn directions_from_deltas(
    dx: i16,
    dy: i16,
    previous_direction: Option<Direction>,
) -> (Option<Direction>, Option<Direction>) {
    let x_dir = Direction::try_from((dx, 0)).ok();
    let y_dir = Direction::try_from((0, dy)).ok();
    if x_dir.is_some()
        && (y_dir.is_none() || previous_direction.is_some_and(|prev| prev.as_delta().0 == 0))
    {
        (x_dir, y_dir)
    } else {
        (y_dir, x_dir)
    }
}

fn directions_from_inputs(
    arrows: Query<(&Direction, Ref<Interaction>), With<ControlArrow>>,
    gamepads: Query<&Gamepad>,
    keys: &ButtonInput<KeyCode>,
    previous_direction: Option<Direction>,
    just_pressed: bool,
) -> (Option<Direction>, Option<Direction>) {
    let directions = directions_from_keys(keys, previous_direction, just_pressed);
    if directions.0.is_some() {
        return directions;
    }

    let directions = directions_from_control_arrows(arrows, previous_direction, just_pressed);
    if directions.0.is_some() {
        return directions;
    }

    for gamepad in gamepads {
        let directions = directions_from_gamepad(gamepad, previous_direction, just_pressed);
        if directions.0.is_some() {
            return directions;
        }
    }

    (None, None)
}

fn directions_from_control_arrows(
    arrows: Query<(&Direction, Ref<Interaction>), With<ControlArrow>>,
    previous_direction: Option<Direction>,
    just_pressed: bool,
) -> (Option<Direction>, Option<Direction>) {
    let mut dx = 0;
    let mut dy = 0;
    for (direction, interaction) in arrows {
        if *interaction == Interaction::Pressed && (!just_pressed || interaction.is_changed()) {
            let delta = direction.as_delta();
            dx += delta.0;
            dy += delta.1;
        }
    }

    directions_from_deltas(dx, dy, previous_direction)
}

fn directions_from_keys(
    keys: &ButtonInput<KeyCode>,
    previous_direction: Option<Direction>,
    just_pressed: bool,
) -> (Option<Direction>, Option<Direction>) {
    let pressed = if just_pressed {
        ButtonInput::just_pressed
    } else {
        ButtonInput::pressed
    };

    let mut dx = 0;
    let mut dy = 0;
    if pressed(keys, KeyCode::ArrowUp) {
        dy -= 1;
    }
    if pressed(keys, KeyCode::ArrowRight) {
        dx += 1
    }
    if pressed(keys, KeyCode::ArrowDown) {
        dy += 1;
    }
    if pressed(keys, KeyCode::ArrowLeft) {
        dx -= 1
    }

    directions_from_deltas(dx, dy, previous_direction)
}

fn position_entities(
    mut query: Query<(Ref<Position>, &mut Transform)>,
    dimensions: Res<Dimensions>,
) {
    for (position, mut transform) in &mut query {
        if position.is_changed() || dimensions.is_changed() {
            transform.translation.x = (-(dimensions.width * HALF_GRID_SIZE)
                + position.x * GRID_SIZE
                - HALF_GRID_SIZE) as f32;
            transform.translation.y = ((dimensions.height * HALF_GRID_SIZE)
                - position.y * GRID_SIZE
                + HALF_GRID_SIZE) as f32;
        }
    }
}

#[expect(clippy::type_complexity)]
fn update_entity_directions(
    mut query: Query<(&Direction, &mut Sprite), (Changed<Direction>, With<DirectionalSprite>)>,
) {
    for (direction, mut sprite) in &mut query {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = *direction as usize;
        }
    }
}

fn on_game_event(
    mut game_events: MessageReader<GameEvent>,
    mut collision_objects_query: Query<CollisionObjectQuery, Without<Player>>,
    mut player_query: Query<(&mut Position, &mut Direction, Option<&Weight>), With<Player>>,
    mut previous_direction: ResMut<PreviousDirection>,
    mut ui_state: ResMut<UiState>,
    dimensions: Res<Dimensions>,
) {
    for event in game_events.read() {
        match event {
            GameEvent::MovePlayer(direction) => {
                *previous_direction = PreviousDirection(Some(*direction));

                if let Ok((mut position, mut player_direction, weight)) = player_query.single_mut()
                {
                    ui_state.camera_offset = Default::default();

                    if move_object(
                        MoveObjectSubject::Player,
                        &mut position,
                        *direction,
                        &dimensions,
                        collision_objects_query.iter_mut().map(Into::into),
                        weight.copied().unwrap_or_default(),
                        MoveObjectInitiator::Player,
                    )
                    .is_ok()
                    {
                        *player_direction = *direction;
                    }
                }
            }
        }
    }
}

fn on_player_moved(mut commands: Commands, query: Query<Ref<Position>, With<Player>>) {
    for player_position in &query {
        if player_position.is_changed() {
            commands.write_message(UpdateBackgroundTransform::Fast);
        }
    }
}

fn on_resize(mut commands: Commands, mut resize_reader: MessageReader<WindowResized>) {
    if resize_reader.read().last().is_some() {
        commands.write_message(UpdateBackgroundTransform::Immediate);
    }
}

fn on_zoom_change(trigger: On<ChangeZoom>, mut commands: Commands, mut ui_state: ResMut<UiState>) {
    let ChangeZoom(factor) = trigger.event();

    let zoom_factor = ui_state.zoom_factor;
    if (*factor < 1. && zoom_factor >= 0.2) || (*factor > 1. && zoom_factor <= 5.) {
        ui_state.zoom_factor = zoom_factor * factor;
        commands.write_message(UpdateBackgroundTransform::Fast);
    }
}

#[expect(clippy::too_many_arguments)]
fn load_level(
    trigger: On<LoadLevel>,
    mut commands: Commands,
    mut background_query: Query<Entity, With<Background>>,
    mut background_events: MessageWriter<UpdateBackgroundTransform>,
    mut dimensions: ResMut<Dimensions>,
    mut game_state: ResMut<GameState>,
    mut pressed_triggers: ResMut<PressedTriggers>,
    mut timer: ResMut<PlayerMovementTimer>,
    assets: Res<GameObjectAssets>,
    fonts: Res<Fonts>,
    levels: Res<Levels>,
    menu_state: Res<MenuState>,
) -> Result<()> {
    let LoadLevel(level) = trigger.event();
    game_state.set_current_level(*level);

    let level_data = levels.get(*level).unwrap_or({
        &Cow::Borrowed(
            r#"[Player]
Position=1,1

[Exit]
Position=2,1
"#,
        )
    });

    let mut level = Level::load(level_data);

    // If we come from a previous level, we check if the new level has an
    // entrance to the previous level. If it does, it will be the player's
    // starting position instead of the one specified by the level.
    if let Some(previous_level) = game_state.previous_level {
        let entrance_position = level
            .objects
            .get(&ObjectType::Entrance)
            .and_then(|entrances| {
                entrances
                    .iter()
                    .find(|entrance| entrance.level == Some(previous_level))
            })
            .map(|entrance| entrance.position);

        if let Some(entrance_position) = entrance_position
            && let Some(players) = level.objects.get_mut(&ObjectType::Player)
        {
            for player in players {
                player.position = entrance_position;
            }
        }
    }

    let background_entity = background_query.single_mut()?;
    let mut background = commands.entity(background_entity);
    background.despawn_related::<Children>();
    background.with_children(|spawner| {
        spawn_level_objects(spawner, level.objects, &assets, &fonts);
    });

    *dimensions = level.dimensions;

    pressed_triggers.num_pressed_triggers = 0;

    timer.reset();
    timer.pause();

    background_events.write(if menu_state.is_in_start_menu() {
        UpdateBackgroundTransform::Immediate
    } else {
        UpdateBackgroundTransform::LevelEntrance
    });

    Ok(())
}

fn load_relative_level(
    trigger: On<LoadRelativeLevel>,
    mut commands: Commands,
    game_state: Res<GameState>,
) {
    let LoadRelativeLevel(delta) = trigger.event();
    let new_level = game_state
        .current_level
        .saturating_add_signed(*delta)
        .min(100);
    commands.trigger(LoadLevel(new_level));
}

fn reset_level(
    _trigger: On<ResetLevel>,
    mut game_state: ResMut<GameState>,
    mut levels: ResMut<Levels>,
) {
    let level = game_state.current_level;
    levels.reset_level(level);
    game_state.set_current_level(level);
}

#[expect(clippy::type_complexity)]
fn save_level(
    trigger: On<SaveLevel>,
    mut levels: ResMut<Levels>,
    dimensions: Res<Dimensions>,
    game_state: Res<GameState>,
    objects_query: Query<(
        &ObjectType,
        &Position,
        &Direction,
        Option<&Entrance>,
        Option<&Massive>,
        Option<&Openable>,
        Option<&Teleporter>,
    )>,
) {
    let SaveLevel { save_to_disk } = trigger.event();

    let mut objects = BTreeMap::new();
    for (object_type, position, direction, entrance, massive, openable, teleporter) in
        &objects_query
    {
        if position.x > 0
            && position.x <= dimensions.width
            && position.y > 0
            && position.y <= dimensions.height
        {
            let positions = objects.entry(*object_type).or_insert(Vec::new());
            positions.push(InitialPositionAndMetadata {
                position: *position,
                direction: *direction,
                identifier: teleporter.map(|teleporter| teleporter.0),
                level: entrance
                    .map(Entrance::level)
                    .or_else(|| openable.and_then(Openable::level)),
                open: openable.is_some() && massive.is_none(),
            });
        }
    }

    if objects
        .get(&ObjectType::Player)
        .is_none_or(|player_locations| player_locations.len() != 1)
    {
        return; // Only save levels with exactly one player.
    }

    let level = Level {
        dimensions: *dimensions,
        objects,
    };
    let content = level.save();
    let current_level = game_state.current_level;

    if *save_to_disk {
        if let Err(error) = fs::write(get_level_path(current_level), &content) {
            println!("Could not save level: {error}");
        }

        levels.insert_stored(current_level, content);
    } else {
        levels.insert_current(current_level, content);
    }
}

fn spawn_level_objects(
    spawner: &mut ChildSpawnerCommands,
    objects: BTreeMap<ObjectType, Vec<InitialPositionAndMetadata>>,
    assets: &GameObjectAssets,
    fonts: &Fonts,
) {
    for (object_type, initial_positions) in objects {
        for initial_position in initial_positions {
            spawn_object_of_type(spawner, assets, fonts, object_type, initial_position);
        }
    }
}

fn spawn_object(
    trigger: On<SpawnObject>,
    mut commands: Commands,
    background_query: Query<Entity, With<Background>>,
    assets: Res<GameObjectAssets>,
    fonts: Res<Fonts>,
) -> Result<()> {
    let SpawnObject {
        object_type,
        position,
    } = trigger.event();

    let background = background_query.single()?;
    let mut background = commands.entity(background);
    background.with_children(|cb| {
        spawn_object_of_type(cb, &assets, &fonts, *object_type, position.clone());
    });

    Ok(())
}
