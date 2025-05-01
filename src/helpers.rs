
use bevy::ecs::component::Component;
use bevy::ecs::query::With;
use bevy::prelude::{Color, Commands, Entity, Query};
use bevy::state::state::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
    GameWon,
    GameLost,
}

/// A helper functions that despawns all entities of a given type. This is useful for cleaning up
/// screens or UI elements that are no longer needed.
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

/// A helper function that converts a catppuccin color to a Bevy color.
pub const fn color_convert(original: catppuccin::Color, alpha: f32) -> Color {
    Color::hsla(
        original.hsl.h as f32,
        original.hsl.s as f32,
        original.hsl.l as f32,
        alpha,
    )
}
