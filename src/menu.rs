use bevy::prelude::*;

use crate::{constants::*, editor::ToggleEditor, fonts::Fonts, setup, GameEvent};

pub const MENU_WIDTH: f32 = 500.;
pub const MENU_HEIGHT: f32 = 400.;

const NUM_HUB_BUTTONS: usize = 4;
const NUM_LEVEL_BUTTONS: usize = 4;

#[derive(Component)]
pub struct Menu {
    kind: MenuKind,
}

#[derive(Resource)]
pub struct MenuState {
    open_menu: Option<MenuKind>,
    selected_button: MenuButtonKind,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            open_menu: Some(MenuKind::Hub),
            selected_button: MenuButtonKind::Start,
        }
    }
}

impl MenuState {
    pub fn is_open(&self) -> bool {
        self.open_menu.is_some()
    }

    pub fn is_in_hub_menu(&self) -> bool {
        self.open_menu == Some(MenuKind::Hub)
    }

    fn move_selected_button(&mut self, delta: isize) {
        let kinds = if self.is_in_hub_menu() {
            MenuButtonKind::hub_buttons()
        } else {
            MenuButtonKind::level_buttons()
        };

        let current_index = kinds
            .iter()
            .position(|kind| *kind == self.selected_button)
            .unwrap_or_default() as isize;
        let num_buttons = kinds.len() as isize;
        let new_index = (current_index + num_buttons + delta) % num_buttons;
        self.selected_button = kinds[new_index as usize];
    }

    pub fn set_open(&mut self, menu: MenuKind) {
        self.open_menu = Some(menu);
        self.selected_button = match menu {
            MenuKind::Hub => MenuButtonKind::Start,
            MenuKind::Level => MenuButtonKind::Restart,
        };
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuKind {
    Hub,
    Level,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_menus.after(setup))
            .init_resource::<MenuState>()
            .add_systems(Update, (on_menu_interaction_input, on_resize))
            .add_systems(Update, render_menu.after(on_menu_interaction_input));
    }
}

#[derive(Clone, Component, Copy, Eq, PartialEq)]
enum MenuButtonKind {
    Start,
    Restart,
    BackToHub,
    Editor,
    OtherGames,
    Quit,
}

impl MenuButtonKind {
    fn hub_buttons() -> [Self; NUM_HUB_BUTTONS] {
        [Self::Start, Self::Editor, Self::OtherGames, Self::Quit]
    }

    fn level_buttons() -> [Self; NUM_LEVEL_BUTTONS] {
        [Self::Restart, Self::BackToHub, Self::Editor, Self::Quit]
    }

    fn label(self) -> &'static str {
        match self {
            Self::Start => "Start",
            Self::Restart => "Restart Level",
            Self::BackToHub => "Exit Level",
            Self::Editor => "Level Editor",
            Self::OtherGames => "Other Games",
            Self::Quit => "Quit Game",
        }
    }
}

fn setup_menus(mut commands: Commands, window_query: Query<&Window>, fonts: Res<Fonts>) {
    let window = window_query
        .get_single()
        .expect("there should be only one window");

    commands
        .spawn((
            Menu {
                kind: MenuKind::Hub,
            },
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(MENU_WIDTH),
                    height: Val::Px(MENU_HEIGHT),
                    border: UiRect::all(Val::Px(2.)),
                    margin: UiRect::all(Val::Auto)
                        .with_top(Val::Px(calculate_top_margin(window.size()))),
                    padding: UiRect::all(Val::Auto),
                    row_gap: Val::Px(40.),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                background_color: GRAY_BACKGROUND.into(),
                border_color: RED.into(),
                z_index: ZIndex::Global(100),
                ..Default::default()
            },
        ))
        .with_children(|cb| {
            for kind in MenuButtonKind::hub_buttons() {
                cb.spawn(MenuButtonBundle::new(kind))
                    .with_children(|cb| MenuButtonBundle::populate(cb, kind.label(), &fonts));
            }
        });

    commands
        .spawn((
            Menu {
                kind: MenuKind::Level,
            },
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(MENU_WIDTH),
                    height: Val::Px(MENU_HEIGHT),
                    border: UiRect::all(Val::Px(2.)),
                    margin: UiRect::all(Val::Auto)
                        .with_top(Val::Px(calculate_top_margin(window.size()))),
                    padding: UiRect::all(Val::Auto),
                    row_gap: Val::Px(40.),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                background_color: GRAY_BACKGROUND.into(),
                border_color: RED.into(),
                z_index: ZIndex::Global(100),
                ..Default::default()
            },
        ))
        .with_children(|cb| {
            for kind in MenuButtonKind::level_buttons() {
                cb.spawn(MenuButtonBundle::new(kind))
                    .with_children(|cb| MenuButtonBundle::populate(cb, kind.label(), &fonts));
            }
        });
}

fn render_menu(
    mut menu_query: Query<(&mut Style, &Menu)>,
    mut button_query: Query<(&MenuButtonKind, &mut BackgroundColor)>,
    menu_state: Res<MenuState>,
) {
    if !menu_state.is_changed() {
        return;
    }

    for (mut menu_style, menu) in &mut menu_query {
        menu_style.display = if menu_state.open_menu.is_some_and(|kind| kind == menu.kind) {
            Display::Flex
        } else {
            Display::None
        };
    }

    for (menu_button, mut background_color) in &mut button_query {
        *background_color = if menu_button == &menu_state.selected_button {
            RED
        } else {
            BLUE
        }
        .into();
    }
}

#[derive(Bundle)]
struct MenuButtonBundle {
    button: ButtonBundle,
}

impl MenuButtonBundle {
    #[expect(clippy::new_ret_no_self)]
    pub fn new(marker: impl Bundle) -> impl Bundle {
        (
            marker,
            Self {
                button: ButtonBundle {
                    background_color: BLUE.into(),
                    style: Style {
                        height: Val::Px(60.),
                        width: Val::Px(300.),
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            },
        )
    }

    pub fn populate(cb: &mut ChildBuilder, text: impl Into<String>, fonts: &Fonts) {
        cb.spawn(TextBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font: fonts.poppins_light.clone(),
                    font_size: 40.,
                    color: WHITE,
                },
            ),
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

pub fn on_menu_keyboard_input(
    commands: Commands,
    mut app_exit_events: EventWriter<AppExit>,
    game_events: EventWriter<GameEvent>,
    mut menu_state: ResMut<MenuState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if menu_state.open_menu.is_none() {
        return;
    }

    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            ArrowUp => menu_state.move_selected_button(-1),
            ArrowDown => menu_state.move_selected_button(1),
            Enter | Space => {
                handle_button_press(commands, app_exit_events, game_events, menu_state);
                return;
            }
            Escape => {
                app_exit_events.send(AppExit::Success);
            }

            _ => continue,
        };
    }
}

fn on_menu_interaction_input(
    commands: Commands,
    app_exit_events: EventWriter<AppExit>,
    game_events: EventWriter<GameEvent>,
    button_query: Query<(&Interaction, &MenuButtonKind), Changed<Interaction>>,
    mut menu_state: ResMut<MenuState>,
) {
    for (interaction, menu_button) in &button_query {
        match *interaction {
            Interaction::Pressed => {
                menu_state.selected_button = *menu_button;
                handle_button_press(commands, app_exit_events, game_events, menu_state);
                return;
            }
            Interaction::Hovered => {
                menu_state.selected_button = *menu_button;
            }
            Interaction::None => {}
        }
    }
}

fn on_resize(
    mut menu_query: Query<&mut Style, With<Menu>>,
    window_query: Query<&Window, Changed<Window>>,
) {
    for window in &window_query {
        for mut style in &mut menu_query {
            style.margin.top = Val::Px(calculate_top_margin(window.size()));
        }
    }
}

fn handle_button_press(
    mut commands: Commands,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_events: EventWriter<GameEvent>,
    mut menu_state: ResMut<MenuState>,
) {
    match menu_state.selected_button {
        MenuButtonKind::Start | MenuButtonKind::Restart => {
            game_events.send(GameEvent::LoadRelativeLevel(0));
            menu_state.open_menu = None;
        }
        MenuButtonKind::BackToHub => {
            game_events.send(GameEvent::LoadLevel(0));
            menu_state.open_menu = None;
        }
        MenuButtonKind::Editor => {
            game_events.send(GameEvent::LoadRelativeLevel(0));
            commands.trigger(ToggleEditor);
            menu_state.open_menu = None;
        }
        MenuButtonKind::OtherGames => { /* TODO */ }
        MenuButtonKind::Quit => {
            app_exit_events.send(AppExit::Success);
        }
    }
}

fn calculate_top_margin(window_size: Vec2) -> f32 {
    // Add a small extra margin at the end so the written logo is revealed well.
    0.5 * (window_size.y - MENU_HEIGHT) + 50.
}
