use crate::{
    colors::{FLAMINGO, GREEN, RED},
    game::Game,
    popup::popup_window,
    states::GameState,
};

use bevy::prelude::*;

/// A helper functions that despawns all entities of a given type. This is
/// useful for cleaning up screens or UI elements that are no longer needed.
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        // Create the game over screen when the game is over.
        app.add_systems(
            OnEnter(GameState::GameOver),
            |mut commands: Commands, game: Res<Game>| {
                let message = match game.game_won {
                    true => "You won!",
                    false => "You lost!",
                };
                commands.spawn((GameOverPlugin, popup_window(message, "Play again!")));
            },
        )

        // Handle button presses on the game over screen.
        .add_systems(Update, button_press_system.run_if(in_state(GameState::GameOver)))

        // Despawn the game over screen when the game is reset. We despawn the game as well
        // because the game is reset when the button is pressed.
        .add_systems(
            OnExit(GameState::GameOver),
            (despawn_screen::<GameOverPlugin>, despawn_screen::<Sprite>),
        );
    }
}

pub fn button_press_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Change the color hove button interactions and reset the game when the button is pressed.
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = FLAMINGO.into();
                game.reset();
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = RED.into();
            }
            Interaction::None => {
                *color = GREEN.into();
            }
        }
    }
}
