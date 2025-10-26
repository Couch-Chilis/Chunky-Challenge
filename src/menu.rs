use bevy::prelude::*;

use crate::{
    LoadLevel, LoadRelativeLevel, background::UpdateBackgroundTransform, constants::*,
    editor::ToggleEditor, fonts::Fonts, setup,
};

pub const MENU_WIDTH: f32 = 500.;
pub const MENU_BUTTON_GAP: f32 = 40.;
pub const MENU_BUTTON_HEIGHT: f32 = 60.;
pub const MENU_BUTTON_WIDTH: f32 = 300.;

#[derive(Component)]
pub struct Menu {
    kind: MenuKind,
}

#[derive(Event)]
struct ButtonPress;

#[derive(Resource)]
pub struct MenuState {
    open_menu: Option<MenuKind>,
    selected_button: MenuButtonKind,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            open_menu: Some(MenuKind::Start),
            selected_button: MenuButtonKind::Start,
        }
    }
}

impl MenuState {
    pub fn is_open(&self) -> bool {
        self.open_menu.is_some()
    }

    pub fn is_in_start_menu(&self) -> bool {
        self.open_menu == Some(MenuKind::Start)
    }

    fn move_selected_button(&mut self, menu_kind: MenuKind, delta: isize) {
        let buttons = MenuButtonKind::buttons_for_menu(menu_kind);

        let current_index = buttons
            .iter()
            .position(|kind| *kind == self.selected_button)
            .unwrap_or_default() as isize;
        let num_buttons = buttons.len() as isize;
        let new_index = (current_index + num_buttons + delta) % num_buttons;
        self.selected_button = buttons[new_index as usize];
    }

    pub fn set_open(&mut self, menu: MenuKind) {
        self.open_menu = Some(menu);
        self.selected_button = match menu {
            MenuKind::Start => MenuButtonKind::Start,
            MenuKind::Hub | MenuKind::Level => MenuButtonKind::Continue,
        };
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuKind {
    Start,
    Hub,
    Level,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_menus.after(setup))
            .init_resource::<MenuState>()
            .add_observer(on_button_press)
            .add_systems(Update, (on_menu_interaction_input, on_resize))
            .add_systems(Update, render_menu.after(on_menu_interaction_input));
    }
}

#[derive(Clone, Component, Copy, Eq, PartialEq)]
enum MenuButtonKind {
    Start,
    Continue,
    Restart,
    RestartLevel,
    BackToHub,
    Editor,
    OtherGames,
    Quit,
}

impl MenuButtonKind {
    fn buttons_for_menu(menu_kind: MenuKind) -> &'static [Self] {
        static START_BUTTONS: &[MenuButtonKind] = &[
            MenuButtonKind::Start,
            MenuButtonKind::Editor,
            MenuButtonKind::OtherGames,
            MenuButtonKind::Quit,
        ];

        static HUB_BUTTONS: &[MenuButtonKind] = &[
            MenuButtonKind::Continue,
            MenuButtonKind::Restart,
            MenuButtonKind::Editor,
            MenuButtonKind::OtherGames,
            MenuButtonKind::Quit,
        ];

        static LEVEL_BUTTONS: &[MenuButtonKind] = &[
            MenuButtonKind::Continue,
            MenuButtonKind::RestartLevel,
            MenuButtonKind::BackToHub,
            MenuButtonKind::Editor,
            MenuButtonKind::Quit,
        ];

        match menu_kind {
            MenuKind::Start => START_BUTTONS,
            MenuKind::Hub => HUB_BUTTONS,
            MenuKind::Level => LEVEL_BUTTONS,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Start => "Start",
            Self::Continue => "Continue",
            Self::Restart => "Restart",
            Self::RestartLevel => "Restart Level",
            Self::BackToHub => "Exit Level",
            Self::Editor => "Level Editor",
            Self::OtherGames => "Other Games",
            Self::Quit => "Quit Game",
        }
    }
}

fn setup_menus(
    mut commands: Commands,
    window_query: Query<&Window>,
    fonts: Res<Fonts>,
) -> Result<()> {
    let window = window_query.single()?;

    let menu_node = |num_buttons: usize| Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Px(MENU_WIDTH),
        height: Val::Px(menu_height(num_buttons)),
        border: UiRect::all(Val::Px(2.)),
        margin: UiRect::all(Val::Auto)
            .with_top(Val::Px(calculate_top_margin(window.size(), num_buttons))),
        padding: UiRect::all(Val::Auto),
        row_gap: Val::Px(40.),
        position_type: PositionType::Absolute,
        ..default()
    };

    for kind in [MenuKind::Start, MenuKind::Hub, MenuKind::Level] {
        let buttons = MenuButtonKind::buttons_for_menu(kind);

        commands
            .spawn((
                Menu { kind },
                BackgroundColor(GRAY_BACKGROUND),
                BorderColor::all(RED),
                GlobalZIndex(100),
                menu_node(buttons.len()),
            ))
            .with_children(|cb| {
                for kind in buttons {
                    cb.spawn(MenuButton::new(*kind))
                        .with_children(|cb| MenuButton::populate(cb, kind.label(), &fonts));
                }
            });
    }

    Ok(())
}

fn render_menu(
    mut menu_query: Query<(&mut Node, &Menu)>,
    mut button_query: Query<(&MenuButtonKind, &mut BackgroundColor)>,
    menu_state: Res<MenuState>,
) {
    if !menu_state.is_changed() {
        return;
    }

    for (mut menu_node, menu) in &mut menu_query {
        menu_node.display = if menu_state.open_menu.is_some_and(|kind| kind == menu.kind) {
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

struct MenuButton;

impl MenuButton {
    #[expect(clippy::new_ret_no_self)]
    pub fn new(marker: impl Bundle) -> impl Bundle {
        (
            marker,
            Button,
            BackgroundColor(BLUE),
            Node {
                height: Val::Px(MENU_BUTTON_HEIGHT),
                width: Val::Px(MENU_BUTTON_WIDTH),
                align_content: AlignContent::Center,
                ..default()
            },
        )
    }

    pub fn populate(spawner: &mut ChildSpawnerCommands, text: impl Into<String>, fonts: &Fonts) {
        spawner.spawn((
            Text::new(text),
            TextColor(WHITE),
            TextFont::from(fonts.poppins_light.clone()).with_font_size(36.),
            Node {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
        ));
    }
}

pub fn on_menu_keyboard_input(
    mut commands: Commands,
    mut menu_state: ResMut<MenuState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Some(menu_kind) = menu_state.open_menu else {
        return;
    };

    for key in keys.get_just_pressed() {
        use KeyCode::*;
        match key {
            ArrowUp => menu_state.move_selected_button(menu_kind, -1),
            ArrowDown => menu_state.move_selected_button(menu_kind, 1),
            Enter | Space => {
                commands.trigger(ButtonPress);
                return;
            }
            Escape => {
                menu_state.open_menu = None;
            }

            _ => continue,
        };
    }
}

fn on_menu_interaction_input(
    mut commands: Commands,
    button_query: Query<(&Interaction, &MenuButtonKind), Changed<Interaction>>,
    mut menu_state: ResMut<MenuState>,
) {
    for (interaction, menu_button) in &button_query {
        match *interaction {
            Interaction::Pressed => {
                menu_state.selected_button = *menu_button;
                commands.trigger(ButtonPress);
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
    mut menu_query: Query<(&mut Node, &Menu)>,
    window_query: Query<&Window, Changed<Window>>,
) {
    for window in &window_query {
        for (mut node, menu) in &mut menu_query {
            let num_buttons = MenuButtonKind::buttons_for_menu(menu.kind).len();
            node.margin.top = Val::Px(calculate_top_margin(window.size(), num_buttons));
        }
    }
}

fn on_button_press(
    _trigger: On<ButtonPress>,
    mut commands: Commands,
    mut app_exit_events: MessageWriter<AppExit>,
    mut background_events: MessageWriter<UpdateBackgroundTransform>,
    mut menu_state: ResMut<MenuState>,
) {
    match menu_state.selected_button {
        MenuButtonKind::Start => {
            background_events.write(UpdateBackgroundTransform::HubIntro);
            menu_state.open_menu = None;
        }
        MenuButtonKind::Continue => {
            menu_state.open_menu = None;
        }
        MenuButtonKind::Restart | MenuButtonKind::RestartLevel => {
            commands.trigger(LoadRelativeLevel(0));
            menu_state.open_menu = None;
        }
        MenuButtonKind::BackToHub => {
            commands.trigger(LoadLevel(0));
            menu_state.open_menu = None;
        }
        MenuButtonKind::Editor => {
            commands.trigger(ToggleEditor);
            menu_state.open_menu = None;
        }
        MenuButtonKind::OtherGames => { /* TODO */ }
        MenuButtonKind::Quit => {
            app_exit_events.write(AppExit::Success);
        }
    }
}

fn calculate_top_margin(window_size: Vec2, num_buttons: usize) -> f32 {
    // Add a small extra margin at the end so the written logo is revealed well.
    0.5 * (window_size.y - menu_height(num_buttons)) + 50.
}

fn menu_height(num_buttons: usize) -> f32 {
    num_buttons as f32 * (MENU_BUTTON_HEIGHT + MENU_BUTTON_GAP)
}
