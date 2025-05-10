
use bevy::state::state::States;

/// The `GameState` enum represents the different states of the game.
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    /// The game is currently being played.
    #[default]
    Playing,

    /// The game is over, either won or lost.
    GameOver,
}
