use bevy::prelude::*;
use catppuccin::PALETTE;

use crate::helpers::{GameState, color_convert};

const NORMAL_BUTTON: Color = color_convert(PALETTE.mocha.colors.green, 1.0);
const HOVERED_BUTTON: Color = color_convert(PALETTE.mocha.colors.red, 1.0);
const PRESSED_BUTTON: Color = color_convert(PALETTE.mocha.colors.flamingo, 1.0);


pub fn popup_window(msg: &str) -> impl Bundle + use<> {
    (
        (
            Name::new("Popup"),
            Node {
                top: Val::Percent(32.5),
                left: Val::Percent(30.0),
                width: Val::Percent(40.0),
                height: Val::Percent(35.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(5.0)),
                margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
            BorderColor(color_convert(PALETTE.mocha.colors.crust, 1.0)),
            BackgroundColor(color_convert(PALETTE.mocha.colors.base, 0.8)),
        ),
        children![Text::new(msg), button()],
    )
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn button() -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, -5.0),
            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(color_convert(PALETTE.mocha.colors.crust, 1.0)),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Play Again"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(color_convert(PALETTE.mocha.colors.crust, 1.0)),
            )]
        )],
    )
}
