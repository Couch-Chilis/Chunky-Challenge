use bevy::prelude::*;

use crate::{
    game_object::{Direction, Player},
    game_state::GameState,
    ui::{UiAssets, menu::MenuKind},
};

use super::menu::MenuState;

#[derive(Component)]
pub struct ControlArrow;

#[derive(Component)]
pub struct MenuButton;

#[derive(Component)]
pub struct Overlay;

pub fn setup_overlays(mut commands: Commands, assets: Res<UiAssets>) {
    commands
        .spawn((
            Overlay,
            GlobalZIndex(100),
            Node {
                display: Display::Flex,
                width: Val::Vw(25.),
                height: Val::Vw(25.),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .with_child((
            MenuButton,
            Interaction::None,
            ImageNode {
                image: assets.menu.clone(),
                ..Default::default()
            },
        ));

    commands
        .spawn((
            Overlay,
            GlobalZIndex(100),
            Node {
                display: Display::Flex,
                width: Val::Vw(100.),
                height: Val::Vw(25.),
                bottom: Val::Px(0.),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .with_children(|spawner| {
            spawner
                .spawn(Node {
                    display: Display::Flex,
                    flex_grow: 1.,
                    width: Val::Vw(25.),
                    height: Val::Vw(25.),
                    ..default()
                })
                .with_child((
                    ControlArrow,
                    Direction::Left,
                    Interaction::None,
                    ImageNode {
                        image: assets.arrow_left.clone(),
                        ..Default::default()
                    },
                ));
            spawner
                .spawn(Node {
                    display: Display::Flex,
                    flex_grow: 1.,
                    width: Val::Vw(25.),
                    height: Val::Vw(25.),
                    ..default()
                })
                .with_child((
                    ControlArrow,
                    Direction::Up,
                    Interaction::None,
                    ImageNode {
                        image: assets.arrow_up.clone(),
                        ..Default::default()
                    },
                ));
            spawner
                .spawn(Node {
                    display: Display::Flex,
                    flex_grow: 1.,
                    width: Val::Vw(25.),
                    height: Val::Vw(25.),
                    ..default()
                })
                .with_child((
                    ControlArrow,
                    Direction::Down,
                    Interaction::None,
                    ImageNode {
                        image: assets.arrow_down.clone(),
                        ..Default::default()
                    },
                ));
            spawner
                .spawn(Node {
                    display: Display::Flex,
                    flex_grow: 1.,
                    width: Val::Vw(25.),
                    height: Val::Vw(25.),
                    ..default()
                })
                .with_child((
                    ControlArrow,
                    Direction::Right,
                    Interaction::None,
                    ImageNode {
                        image: assets.arrow_right.clone(),
                        ..Default::default()
                    },
                ));
        });
}

pub fn is_in_overlay(position: Vec2, window: &Window) -> bool {
    let quarter_width = 0.25 * window.width();

    let is_in_controls_overlay = position.y >= window.height() - quarter_width;
    let is_in_menu_overlay = position.y <= quarter_width && position.x <= quarter_width;

    is_in_controls_overlay || is_in_menu_overlay
}

pub fn check_for_menu_button_interaction(
    mut menu_state: ResMut<MenuState>,
    game_state: Res<GameState>,
    menu_button_query: Query<&Interaction, With<MenuButton>>,
) {
    for interaction in menu_button_query {
        if *interaction == Interaction::Pressed {
            menu_state.set_open(if game_state.is_in_hub() {
                MenuKind::Hub
            } else {
                MenuKind::Level
            });
        }
    }
}

pub fn check_for_overlay_visibility(
    mut overlay_query: Query<&mut Node, With<Overlay>>,
    player_query: Query<Entity, With<Player>>,
    menu_state: Res<MenuState>,
) {
    for mut overlay_style in &mut overlay_query {
        if player_query.single().is_ok() && !menu_state.is_open() {
            if overlay_style.display != Display::Flex {
                overlay_style.display = Display::Flex;
            }
        } else if overlay_style.display != Display::None {
            overlay_style.display = Display::None;
        }
    }
}
