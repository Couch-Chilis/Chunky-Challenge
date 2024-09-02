use bevy::prelude::*;

use crate::{constants::*, editor::Editor, fonts::Fonts, Player};

#[derive(Component)]
pub struct GameOver;

pub fn setup_gameover(commands: &mut Commands, fonts: &Fonts) {
    commands
        .spawn((
            GameOver,
            NodeBundle {
                style: Style {
                    display: Display::None,
                    width: Val::Px(300.),
                    height: Val::Px(80.),
                    border: UiRect::all(Val::Px(2.)),
                    margin: UiRect::all(Val::Auto),
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
            cb.spawn(TextBundle {
                text: Text::from_section(
                    "Game Over\n\nPress Enter to try again",
                    TextStyle {
                        font: fonts.poppins_light.clone(),
                        font_size: 20.,
                        color: WHITE,
                    },
                )
                .with_justify(JustifyText::Center),
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

pub fn check_for_game_over(
    mut game_over_query: Query<&mut Style, With<GameOver>>,
    editor_query: Query<Entity, With<Editor>>,
    player_query: Query<Entity, With<Player>>,
) {
    let mut game_over_style = game_over_query.get_single_mut().unwrap();

    if player_query.get_single().is_ok() || editor_query.get_single().is_ok() {
        if game_over_style.display != Display::None {
            game_over_style.display = Display::None;
        }
    } else if game_over_style.display != Display::Flex {
        game_over_style.display = Display::Flex;
    }
}
