use bevy::prelude::*;

use crate::{
    game_object::{Direction, Player},
    ui::UiAssets,
};

use super::menu::MenuState;

#[derive(Component)]
pub struct ControlsOverlay;

#[derive(Component)]
pub struct ControlArrow;

pub fn setup_controls_overlay(mut commands: Commands, assets: Res<UiAssets>) {
    commands
        .spawn((
            ControlsOverlay,
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

pub fn check_for_controls_visibility(
    mut overlay_query: Query<&mut Node, With<ControlsOverlay>>,
    player_query: Query<Entity, With<Player>>,
    menu_state: Res<MenuState>,
) -> Result<()> {
    let mut overlay_style = overlay_query.single_mut()?;

    if player_query.single().is_ok() && !menu_state.is_open() {
        if overlay_style.display != Display::Flex {
            overlay_style.display = Display::Flex;
        }
    } else if overlay_style.display != Display::None {
        overlay_style.display = Display::None;
    }

    Ok(())
}
