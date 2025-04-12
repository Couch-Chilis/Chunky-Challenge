#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod background;
mod constants;
mod editor;
mod errors;
mod fonts;
mod game_object;
mod game_state;
mod gameover;
mod levels;
mod menu;
mod timers;
mod ui_state;
mod utils;

use std::{borrow::Cow, collections::BTreeMap, fs};

use background::{Background, BackgroundPlugin, UpdateBackgroundTransform};
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode, WindowResized, WindowResolution},
    winit::WinitWindows,
};
use constants::*;
use editor::{
    on_editor_keyboard_input, on_editor_mouse_input, EditorPlugin, EditorState, SelectionOverlay,
    ToggleEditor,
};
use fonts::Fonts;
use game_object::{
    behaviors::*, spawn_object_of_type, CollisionObjectQuery, Direction, Entrance,
    GameObjectAssets, Massive, ObjectType, Openable, Player, Position, Teleporter, Weight,
    PLAYER_ASSET,
};
use game_state::GameState;
use gameover::{check_for_game_over, setup_gameover};
use levels::{Dimensions, InitialPositionAndMetadata, Level, Levels};
use menu::{on_menu_keyboard_input, MenuKind, MenuPlugin, MenuState};
use timers::{AnimationTimer, MovementTimer, TemporaryTimer, TransporterTimer};
use ui_state::UiState;
use utils::get_level_path;
use winit::window::Icon;

#[derive(Default, Resource)]
struct ExitState {
    next_level: Option<u16>,
}

#[derive(Default, Resource)]
struct PressedTriggers {
    num_pressed_triggers: usize,
}

#[derive(Event)]
struct ChangeZoom(f32);

#[derive(Event)]
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

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Chunky's Challenge".to_owned(),
                    mode: get_initial_window_mode(),
                    resolution: WindowResolution::from((DEFAULT_WINDOW_SIZE, DEFAULT_WINDOW_SIZE))
                        .with_scale_factor_override(1.),
                    ..default()
                }),
                ..default()
            }),
            BackgroundPlugin,
            EditorPlugin,
            MenuPlugin,
        ))
        .init_resource::<AnimationTimer>()
        .init_resource::<Dimensions>()
        .init_resource::<ExitState>()
        .init_resource::<Fonts>()
        .init_resource::<GameObjectAssets>()
        .init_resource::<Levels>()
        .init_resource::<MovementTimer>()
        .init_resource::<PressedTriggers>()
        .init_resource::<TemporaryTimer>()
        .init_resource::<TransporterTimer>()
        .init_resource::<UiState>()
        .insert_resource(GameState::load())
        .add_event::<ChangeZoom>()
        .add_event::<GameEvent>()
        .add_event::<LoadLevel>()
        .add_event::<LoadRelativeLevel>()
        .add_event::<ResetLevel>()
        .add_event::<SaveLevel>()
        .add_event::<SpawnObject>()
        .add_observer(load_level)
        .add_observer(load_relative_level)
        .add_observer(on_zoom_change)
        .add_observer(reset_level)
        .add_observer(save_level)
        .add_observer(spawn_object)
        .add_systems(Startup, (set_window_icon, setup))
        .add_systems(PostStartup, post_setup)
        .add_systems(Update, (on_keyboard_input, on_mouse_input, on_resize))
        .add_systems(
            Update,
            (
                animate_objects,
                check_for_deadly,
                check_for_entrance,
                check_for_exit,
                check_for_explosive,
                check_for_liquid,
                check_for_game_over,
                check_for_slippery_and_transporter,
                despawn_volatile_objects,
                move_objects,
                on_game_event,
            )
                .after(on_keyboard_input),
        )
        .add_systems(
            Update,
            (check_for_transform_on_push, check_for_triggers)
                .after(check_for_liquid)
                .after(on_keyboard_input)
                .after(move_objects),
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
                .after(move_objects)
                .after(on_mouse_input),
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

fn set_window_icon(windows: NonSend<WinitWindows>) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory_with_format(PLAYER_ASSET, image::ImageFormat::Png)
            .unwrap()
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

fn setup(
    mut commands: Commands,
    mut fonts: ResMut<Fonts>,
    mut font_assets: ResMut<Assets<Font>>,
    mut game_object_assets: ResMut<GameObjectAssets>,
    mut image_assets: ResMut<Assets<Image>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    *game_object_assets.as_mut() =
        GameObjectAssets::load(&mut image_assets, &mut texture_atlas_layouts);

    fonts.poppins_light = font_assets.add(
        Font::try_from_bytes(Vec::from(include_bytes!(
            "../assets/font/Poppins/Poppins-Light.ttf"
        )))
        .unwrap(),
    );

    commands.spawn(Camera2d);

    setup_gameover(&mut commands, &fonts);
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
) {
    if editor_state.is_open {
        on_editor_mouse_input(
            commands,
            selection_query,
            background_query,
            objects,
            window_query,
            editor_state,
            buttons,
            dimensions,
        );
        return;
    } else if menu_state.is_open() {
        return;
    }

    if !buttons.pressed(MouseButton::Left) {
        if ui_state.drag_start.is_some() {
            ui_state.drag_start = None;
        }
        return;
    }

    let window = window_query.single();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let zoom_factor = ui_state.zoom_factor;
    let x = cursor_position.x / (zoom_factor * GRID_SIZE as f32);
    let y = cursor_position.y / (zoom_factor * GRID_SIZE as f32);

    if let Some((start_x, start_y)) = ui_state.drag_start {
        let new_camera_offset = (start_x - x, start_y - y);
        if ui_state.camera_offset != new_camera_offset {
            ui_state.camera_offset.0 += new_camera_offset.0;
            ui_state.camera_offset.1 += new_camera_offset.1;
            commands.send_event(UpdateBackgroundTransform::Fast);
        }
    }

    ui_state.drag_start = Some((x, y));
}

#[expect(clippy::too_many_arguments)]
fn on_keyboard_input(
    mut commands: Commands,
    mut game_events: EventWriter<GameEvent>,
    app_exit_events: EventWriter<AppExit>,
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
        on_menu_keyboard_input(commands, app_exit_events, menu_state, keys);
        return;
    } else if exit_state.next_level.is_some() {
        return;
    }

    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            ArrowUp => {
                game_events.send(GameEvent::MovePlayer(Direction::Up));
            }
            ArrowRight => {
                game_events.send(GameEvent::MovePlayer(Direction::Right));
            }
            ArrowDown => {
                game_events.send(GameEvent::MovePlayer(Direction::Down));
            }
            ArrowLeft => {
                game_events.send(GameEvent::MovePlayer(Direction::Left));
            }
            Enter if player_query.get_single().is_err() => {
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

fn update_entity_directions(mut query: Query<(&Direction, &mut Sprite), Changed<Direction>>) {
    for (direction, mut sprite) in &mut query {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = *direction as usize;
        }
    }
}

fn on_game_event(
    mut level_events: EventReader<GameEvent>,
    mut collision_objects_query: Query<CollisionObjectQuery, Without<Player>>,
    mut player_query: Query<(&mut Position, &mut Direction, Option<&Weight>), With<Player>>,
    mut ui_state: ResMut<UiState>,
    dimensions: Res<Dimensions>,
) {
    for event in level_events.read() {
        match event {
            GameEvent::MovePlayer(direction) => {
                if let Ok((mut position, mut player_direction, weight)) =
                    player_query.get_single_mut()
                {
                    ui_state.camera_offset = Default::default();

                    if move_object(
                        &mut position,
                        *direction,
                        &dimensions,
                        collision_objects_query.iter_mut().map(Into::into),
                        weight.copied().unwrap_or_default(),
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
            commands.send_event(UpdateBackgroundTransform::Fast);
        }
    }
}

fn on_resize(mut commands: Commands, mut resize_reader: EventReader<WindowResized>) {
    if resize_reader.read().last().is_some() {
        commands.send_event(UpdateBackgroundTransform::Immediate);
    }
}

fn on_zoom_change(
    trigger: Trigger<ChangeZoom>,
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
) {
    let ChangeZoom(factor) = trigger.event();

    let zoom_factor = ui_state.zoom_factor;
    if (*factor < 1. && zoom_factor >= 0.2) || (*factor > 1. && zoom_factor <= 5.) {
        ui_state.zoom_factor = zoom_factor * factor;
        commands.send_event(UpdateBackgroundTransform::Fast);
    }
}

#[expect(clippy::too_many_arguments)]
fn load_level(
    trigger: Trigger<LoadLevel>,
    mut commands: Commands,
    mut background_query: Query<Entity, With<Background>>,
    mut background_events: EventWriter<UpdateBackgroundTransform>,
    mut dimensions: ResMut<Dimensions>,
    mut exit_state: ResMut<ExitState>,
    mut game_state: ResMut<GameState>,
    mut pressed_triggers: ResMut<PressedTriggers>,
    assets: Res<GameObjectAssets>,
    fonts: Res<Fonts>,
    levels: Res<Levels>,
    menu_state: Res<MenuState>,
) {
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

        if let Some(entrance_position) = entrance_position {
            if let Some(players) = level.objects.get_mut(&ObjectType::Player) {
                for player in players {
                    player.position = entrance_position;
                }
            }
        }
    }

    let background_entity = background_query.single_mut();
    let mut background = commands.entity(background_entity);
    background.despawn_descendants();
    background.with_children(|cb| {
        spawn_level_objects(cb, level.objects, &assets, &fonts);
    });

    pressed_triggers.num_pressed_triggers = 0;

    *dimensions = level.dimensions;

    exit_state.next_level = None;

    background_events.send(if menu_state.is_in_hub_menu() {
        UpdateBackgroundTransform::Immediate
    } else {
        UpdateBackgroundTransform::LevelEntrance
    });
}

fn load_relative_level(
    trigger: Trigger<LoadRelativeLevel>,
    mut commands: Commands,
    game_state: Res<GameState>,
) {
    let LoadRelativeLevel(delta) = trigger.event();
    let new_level = game_state.current_level.saturating_add_signed(*delta);
    commands.trigger(LoadLevel(new_level));
}

fn reset_level(
    _trigger: Trigger<ResetLevel>,
    mut game_state: ResMut<GameState>,
    mut levels: ResMut<Levels>,
) {
    let level = game_state.current_level;
    levels.reset_level(level);
    game_state.set_current_level(level);
}

#[expect(clippy::type_complexity)]
fn save_level(
    trigger: Trigger<SaveLevel>,
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
                level: entrance.map(|entrance| entrance.0).or_else(|| {
                    openable.and_then(|openable| match openable {
                        Openable::Key => None,
                        Openable::LevelFinished(level) => Some(*level),
                        Openable::Trigger => None,
                    })
                }),
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
    cb: &mut ChildBuilder,
    objects: BTreeMap<ObjectType, Vec<InitialPositionAndMetadata>>,
    assets: &GameObjectAssets,
    fonts: &Fonts,
) {
    for (object_type, initial_positions) in objects {
        for initial_position in initial_positions {
            spawn_object_of_type(cb, assets, fonts, object_type, initial_position);
        }
    }
}

fn spawn_object(
    trigger: Trigger<SpawnObject>,
    mut commands: Commands,
    background_query: Query<Entity, With<Background>>,
    assets: Res<GameObjectAssets>,
    fonts: Res<Fonts>,
) {
    let SpawnObject {
        object_type,
        position,
    } = trigger.event();

    let background = background_query
        .get_single()
        .expect("there should be only one background");
    let mut background = commands.entity(background);
    background.with_children(|cb| {
        spawn_object_of_type(cb, &assets, &fonts, *object_type, position.clone());
    });
}
