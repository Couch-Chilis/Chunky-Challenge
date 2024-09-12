#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod background;
mod constants;
mod editor;
mod errors;
mod fonts;
mod game_object;
mod gameover;
mod level;
mod menu;
mod timers;
mod utils;

use std::{borrow::Cow, collections::BTreeMap, fs};

use background::{Background, BackgroundPlugin, UpdateBackgroundTransform};
use bevy::{
    prelude::*,
    window::{WindowMode, WindowResized, WindowResolution},
    winit::WinitWindows,
};
use constants::*;
use editor::{spawn_selected_object, Editor, EditorPlugin, ToggleEditor};
use fonts::Fonts;
use game_object::{
    behaviors::*, spawn_object_of_type, Direction, Entrance, GameObjectAssets, ObjectType, Player,
    Position, Weight, PLAYER_ASSET,
};
use gameover::{check_for_game_over, setup_gameover};
use level::{Dimensions, InitialPositionAndMetadata, Level, LEVELS};
use menu::{MenuPlugin, MenuState};
use timers::{AnimationTimer, MovementTimer, TemporaryTimer, TransporterTimer};
use utils::get_level_filename;
use winit::window::Icon;

pub const INITIAL_HUB_FOCUS: (i16, i16) = (33, 26);

#[derive(Resource)]
struct Levels {
    current_level: u16,
    previous_level: Option<u16>,
    levels: BTreeMap<u16, Cow<'static, str>>,
}

impl Default for Levels {
    fn default() -> Self {
        Self {
            current_level: 0,
            previous_level: None,
            levels: LEVELS
                .iter()
                .map(|(level_num, data)| (*level_num, (*data).into()))
                .collect(),
        }
    }
}

impl Levels {
    fn set_current_level(&mut self, level: u16) {
        self.previous_level = Some(self.current_level);
        self.current_level = level;
    }
}

#[derive(Default, Resource)]
struct PressedTriggers {
    num_pressed_triggers: usize,
}

#[derive(Resource)]
struct Zoom {
    factor: f32,
}

impl Zoom {
    fn hub_default() -> Self {
        Self { factor: 0.32768 }
    }

    fn level_default() -> Self {
        Self { factor: 1. }
    }
}

#[derive(Event)]
enum GameEvent {
    ChangeWidth(i16),
    ChangeHeight(i16),
    ChangeZoom(f32),
    LoadLevel(u16),
    LoadRelativeLevel(i16),
    MovePlayer(i16, i16),
    ToggleEditor,
    Exit,
}

#[derive(Event)]
struct MoveAllObjects {
    dx: i16,
    dy: i16,
}

#[derive(Event)]
struct SaveLevel {
    save_to_disk: bool,
    next_level: Option<u16>,
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
        .init_resource::<Fonts>()
        .init_resource::<GameObjectAssets>()
        .init_resource::<Levels>()
        .init_resource::<MovementTimer>()
        .init_resource::<PressedTriggers>()
        .init_resource::<TemporaryTimer>()
        .init_resource::<TransporterTimer>()
        .insert_resource(Zoom::hub_default())
        .add_event::<GameEvent>()
        .add_event::<MoveAllObjects>()
        .add_event::<SaveLevel>()
        .add_systems(Startup, (set_window_icon, setup))
        .add_systems(Update, (on_keyboard_input, on_resize))
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
                check_for_transform_on_push,
                despawn_volatile_objects,
                move_objects,
                on_game_event,
            )
                .after(on_keyboard_input),
        )
        .add_systems(
            Update,
            check_for_triggers
                .after(on_keyboard_input)
                .after(move_objects),
        )
        .add_systems(Update, load_level.after(on_game_event))
        .add_systems(
            Update,
            (position_entities, update_entity_directions)
                .after(load_level)
                .after(check_for_explosive)
                .after(check_for_liquid)
                .after(move_objects)
                .after(spawn_selected_object),
        )
        .observe(move_all_objects)
        .observe(save_level)
        .run();
}

fn get_initial_window_mode() -> WindowMode {
    if cfg!(target_os = "ios") || std::env::var_os("SteamTenfoot").is_some() {
        WindowMode::BorderlessFullscreen
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
    mut events: EventWriter<GameEvent>,
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

    commands.spawn(Camera2dBundle::default());

    setup_gameover(&mut commands, &fonts);

    events.send(GameEvent::LoadRelativeLevel(0));
}

fn on_keyboard_input(
    mut events: EventWriter<GameEvent>,
    player_query: Query<Entity, With<Player>>,
    menu_state: Res<MenuState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if menu_state.is_open {
        return;
    }

    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            ArrowUp => events.send(GameEvent::MovePlayer(0, -1)),
            ArrowRight => events.send(GameEvent::MovePlayer(1, 0)),
            ArrowDown => events.send(GameEvent::MovePlayer(0, 1)),
            ArrowLeft => events.send(GameEvent::MovePlayer(-1, 0)),
            Enter if player_query.get_single().is_err() => {
                events.send(GameEvent::LoadRelativeLevel(0))
            }
            Equal => events.send(GameEvent::ChangeZoom(1.25)),
            Minus => events.send(GameEvent::ChangeZoom(0.8)),
            BracketRight => events.send(GameEvent::LoadRelativeLevel(1)),
            BracketLeft => events.send(GameEvent::LoadRelativeLevel(-1)),
            KeyE => events.send(GameEvent::ToggleEditor),
            KeyR => events.send(GameEvent::LoadRelativeLevel(0)),
            Escape => events.send(GameEvent::Exit),

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

fn update_entity_directions(mut query: Query<(&Direction, &mut TextureAtlas), Changed<Direction>>) {
    for (direction, mut atlas) in &mut query {
        atlas.index = *direction as usize;
    }
}

type PlayerComponents<'a> = (
    Entity,
    &'a mut Position,
    Option<&'a mut Direction>,
    Option<&'a Weight>,
);

#[allow(clippy::too_many_arguments)]
fn on_game_event(
    mut commands: Commands,
    mut app_exit_events: EventWriter<AppExit>,
    mut collision_objects_query: Query<CollisionObject, Without<Player>>,
    mut dimensions: ResMut<Dimensions>,
    mut level_events: EventReader<GameEvent>,
    mut levels: ResMut<Levels>,
    mut player_query: Query<PlayerComponents, With<Player>>,
    mut menu_state: ResMut<MenuState>,
    mut zoom: ResMut<Zoom>,
    editor_query: Query<Entity, With<Editor>>,
) {
    for event in level_events.read() {
        match event {
            GameEvent::ChangeHeight(delta) => {
                if dimensions.height + delta > 0 {
                    dimensions.height += delta;

                    if delta.abs() > 1 {
                        commands.trigger(MoveAllObjects {
                            dx: 0,
                            dy: delta / 2,
                        });
                    }
                }
            }
            GameEvent::ChangeWidth(delta) => {
                if dimensions.width + delta > 0 {
                    dimensions.width += delta;

                    if delta.abs() > 1 {
                        commands.trigger(MoveAllObjects {
                            dx: delta / 2,
                            dy: 0,
                        });
                    }
                }
            }
            GameEvent::ChangeZoom(factor) => {
                zoom.factor *= factor;
                commands.trigger(UpdateBackgroundTransform);
            }
            GameEvent::LoadLevel(level) => {
                levels.set_current_level(*level);
            }
            GameEvent::LoadRelativeLevel(delta) => {
                let new_level = levels.current_level.saturating_add_signed(*delta);
                levels.set_current_level(new_level);
            }
            GameEvent::MovePlayer(dx, dy) => {
                if let Ok((player, mut position, player_direction, weight)) =
                    player_query.get_single_mut()
                {
                    if move_object(
                        &mut position,
                        (*dx, *dy),
                        &dimensions,
                        collision_objects_query.iter_mut(),
                        weight.copied().unwrap_or_default(),
                    ) {
                        if let Ok(direction) = Direction::try_from((*dx, *dy)) {
                            if let Some(mut player_direction) = player_direction {
                                *player_direction = direction;
                            } else {
                                commands.entity(player).insert(direction);
                            }
                        }
                        commands.trigger(UpdateBackgroundTransform);
                    }
                }
            }
            GameEvent::ToggleEditor => {
                commands.trigger(ToggleEditor);
            }
            GameEvent::Exit => {
                if editor_query.get_single().is_ok() {
                    commands.trigger(ToggleEditor);
                }

                if menu_state.is_open {
                    app_exit_events.send(AppExit::Success);
                } else {
                    menu_state.is_open = true;
                }
            }
        }
    }
}

fn on_resize(mut commands: Commands, mut resize_reader: EventReader<WindowResized>) {
    if resize_reader.read().last().is_some() {
        commands.trigger(UpdateBackgroundTransform);
    }
}

#[allow(clippy::too_many_arguments)]
fn load_level(
    mut commands: Commands,
    mut background_query: Query<Entity, With<Background>>,
    mut dimensions: ResMut<Dimensions>,
    mut pressed_triggers: ResMut<PressedTriggers>,
    mut zoom: ResMut<Zoom>,
    assets: Res<GameObjectAssets>,
    fonts: Res<Fonts>,
    levels: Res<Levels>,
) {
    if !levels.is_changed() {
        return;
    }

    let Some(level_data) = levels.levels.get(&levels.current_level) else {
        return;
    };

    let mut level = Level::load(level_data);

    // If we come from a previous level, we check if the new level has an
    // entrance to the previous level. If it does, it will be the player's
    // starting position instead of the one specified by the level.
    if let Some(previous_level) = levels.previous_level {
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

    let background_entity = background_query
        .get_single_mut()
        .expect("there should be only one background");

    let mut background = commands.entity(background_entity);
    background.despawn_descendants();
    background.with_children(|cb| {
        spawn_level_objects(cb, level.objects, &assets, &fonts);
    });

    pressed_triggers.num_pressed_triggers = 0;

    *dimensions = level.dimensions;

    *zoom = if levels.current_level == 0 && levels.previous_level.is_none() {
        Zoom::hub_default()
    } else {
        Zoom::level_default()
    };
}

fn move_all_objects(trigger: Trigger<MoveAllObjects>, mut query: Query<&mut Position>) {
    let MoveAllObjects { dx, dy } = trigger.event();

    for mut position in &mut query {
        position.x += *dx;
        position.y += *dy;
    }
}

fn save_level(
    trigger: Trigger<SaveLevel>,
    mut levels: ResMut<Levels>,
    dimensions: Res<Dimensions>,
    objects_query: Query<(
        &ObjectType,
        &Position,
        Option<&Direction>,
        Option<&Entrance>,
    )>,
) {
    let SaveLevel {
        save_to_disk,
        next_level,
    } = trigger.event();

    let mut objects = BTreeMap::new();
    for (object_type, position, direction, entrance) in &objects_query {
        if position.x > 0
            && position.x <= dimensions.width
            && position.y > 0
            && position.y <= dimensions.height
        {
            let positions = objects.entry(*object_type).or_insert(Vec::new());
            positions.push(InitialPositionAndMetadata {
                position: *position,
                direction: direction.copied(),
                level: entrance.map(|entrance| entrance.0),
            });
        }
    }

    if !objects
        .get(&ObjectType::Player)
        .is_some_and(|player_locations| player_locations.len() == 1)
    {
        return; // Only save levels with exactly one player.
    }

    let level = Level {
        dimensions: *dimensions,
        objects,
    };
    let content = level.save();
    let current_level = levels.current_level;

    if *save_to_disk {
        if let Err(error) = fs::write(get_level_filename(current_level), &content) {
            println!("Could not save level: {error}");
        }
    }

    levels.levels.insert(current_level, content.into());

    if let Some(next_level) = next_level {
        levels.set_current_level(*next_level);
    }
}

fn spawn_level_objects(
    commands: &mut ChildBuilder,
    objects: BTreeMap<ObjectType, Vec<InitialPositionAndMetadata>>,
    assets: &GameObjectAssets,
    fonts: &Fonts,
) {
    for (object_type, initial_positions) in objects {
        for initial_position in initial_positions {
            spawn_object_of_type(commands, assets, fonts, object_type, initial_position);
        }
    }
}
