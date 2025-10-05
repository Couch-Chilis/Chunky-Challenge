use bevy::prelude::*;

use crate::{
    background::UpdateBackgroundTransform, constants::*, editor::ToggleEditor, fonts::Fonts, setup,
    LoadLevel, ResetLevel,
};

pub const MENU_WIDTH: f32 = 500.;
pub const MENU_HEIGHT: f32 = 400.;

const NUM_HUB_BUTTONS: usize = 4;
const NUM_LEVEL_BUTTONS: usize = 4;

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
            .add_observer(on_button_press)
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

fn setup_menus(
    mut commands: Commands,
    window_query: Query<&Window>,
    fonts: Res<Fonts>,
) -> Result<()> {
    let window = window_query.single()?;

    commands
        .spawn((
            Menu {
                kind: MenuKind::Hub,
            },
            BackgroundColor(GRAY_BACKGROUND),
            BorderColor::all(RED),
            GlobalZIndex(100),
            Node {
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
                ..default()
            },
        ))
        .with_children(|cb| {
            for kind in MenuButtonKind::hub_buttons() {
                cb.spawn(MenuButton::new(kind))
                    .with_children(|cb| MenuButton::populate(cb, kind.label(), &fonts));
            }
        });

    commands
        .spawn((
            Menu {
                kind: MenuKind::Level,
            },
            BackgroundColor(GRAY_BACKGROUND),
            BorderColor::all(RED),
            GlobalZIndex(100),
            Node {
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
                ..default()
            },
        ))
        .with_children(|cb| {
            for kind in MenuButtonKind::level_buttons() {
                cb.spawn(MenuButton::new(kind))
                    .with_children(|cb| MenuButton::populate(cb, kind.label(), &fonts));
            }
        });

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
                height: Val::Px(60.),
                width: Val::Px(300.),
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
    mut app_exit_events: MessageWriter<AppExit>,
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
                commands.trigger(ButtonPress);
                return;
            }
            Escape => {
                app_exit_events.write(AppExit::Success);
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
    mut menu_query: Query<&mut Node, With<Menu>>,
    window_query: Query<&Window, Changed<Window>>,
) {
    for window in &window_query {
        for mut node in &mut menu_query {
            node.margin.top = Val::Px(calculate_top_margin(window.size()));
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
        MenuButtonKind::Restart => {
            commands.trigger(ResetLevel);
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

fn calculate_top_margin(window_size: Vec2) -> f32 {
    // Add a small extra margin at the end so the written logo is revealed well.
    0.5 * (window_size.y - MENU_HEIGHT) + 50.
}
