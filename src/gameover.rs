use bevy::prelude::*;

use crate::{constants::*, editor::EditorState, fonts::Fonts, Player};

#[derive(Component)]
pub struct GameOver;

pub fn setup_gameover(commands: &mut Commands, fonts: &Fonts) {
    commands
        .spawn((
            GameOver,
            BackgroundColor(GRAY_BACKGROUND),
            BorderColor::all(RED),
            GlobalZIndex(100),
            Node {
                display: Display::None,
                width: Val::Px(300.),
                height: Val::Px(80.),
                border: UiRect::all(Val::Px(2.)),
                margin: UiRect::all(Val::Auto),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .with_children(|cb| {
            cb.spawn((
                Text::new("Game Over\n\nPress Enter to try again"),
                TextColor(WHITE),
                TextFont::from(fonts.poppins_light.clone()).with_font_size(20.),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
            ));
        });
}

pub fn check_for_game_over(
    mut game_over_query: Query<&mut Node, With<GameOver>>,
    player_query: Query<Entity, With<Player>>,
    editor: Res<EditorState>,
) -> Result<()> {
    let mut game_over_style = game_over_query.single_mut()?;

    if player_query.single().is_ok() || editor.is_open {
        if game_over_style.display != Display::None {
            game_over_style.display = Display::None;
        }
    } else if game_over_style.display != Display::Flex {
        game_over_style.display = Display::Flex;
    }

    Ok(())
}
