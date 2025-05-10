use bevy::prelude::*;

use crate::colors::{BASE_80, CRUST, GREEN};

/// Create a popup windows with a message and a button.
pub fn popup_window(msg: &str, button_text: &str) -> impl Bundle + use<> {
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
            BorderColor(CRUST),
            BackgroundColor(BASE_80),
        ),
        children![Text::new(msg), button(button_text)],
    )
}

// Add a button to the popup window.
fn button(text: &str) -> impl Bundle + use<> {
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
            BorderColor(CRUST),
            BorderRadius::MAX,
            BackgroundColor(GREEN),
            children![(
                Text::new(text),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(CRUST),
            )]
        )],
    )
}
