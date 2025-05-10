use crate::{
    assets::{asset_path, asset_path_tile},
    game::{Game, Response},
    states::GameState,
};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct MinesweeperPlugin;

impl Plugin for MinesweeperPlugin {
    fn build(&self, app: &mut App) {
        // Add out game resource.
        app.insert_resource(Game::new(30, 16, 20))
            // Initialize the game state.
            .init_state::<GameState>()
            // Add the game creation and update.
            .add_systems(OnEnter(GameState::Playing), create_game)
            .add_systems(Update, update_game.run_if(in_state(GameState::Playing)));
    }
}

// Initialize the game by spawning a grid of tiles.
fn create_game(mut commands: Commands, game: Res<Game>, asset_server: Res<AssetServer>) {
    // For each tile in the game, spawn a sprite entity with the closed image.
    for row in 0..game.height {
        for column in 0..game.width {
            let tile_position = game.tile_position(column, row);
            commands.spawn((
                Name::new(format!("Cell ({}, {})", row, column)),
                Sprite::from_image(asset_server.load(asset_path("closed"))),
                Transform {
                    translation: Vec3::new(tile_position.x, tile_position.y, 1.0),
                    ..default()
                },
            ));
        }
    }
}

// Update the game state based on mouse input.
fn update_game(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut tile_query: Query<(&mut Sprite, &Transform), With<Sprite>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // We aren't interested in anything other than left or right mouse button presses.
    if !mouse_button_input.just_pressed(MouseButton::Right)
        && !mouse_button_input.just_pressed(MouseButton::Left)
    {
        return;
    }

    // Get the mouse position in window coordinates.
    let cursor_position = match window_query.single() {
        Ok(window) => match window.cursor_position() {
            Some(position) => position,
            None => return,
        },
        Err(_) => return,
    };

    // Convert window coordinates to world coordinates.
    let world_position = game.window_to_world(window_query.single().unwrap(), cursor_position);

    // Get the tile coordinates from the world coordinates.
    let (tile_x, tile_y) = match game.world_to_tile(world_position) {
        Some((x, y)) => (x, y),
        None => return,
    };
    debug!("Clicked on tile: ({}, {})", tile_x, tile_y);

    // Determine the action based on the mouse button pressed
    let action = match mouse_button_input.just_pressed(MouseButton::Left) {
        true => crate::game::Action::Reveal,
        false => crate::game::Action::Flag,
    };

    // Perform the action on the game resource.
    let result = match game.perform_action(tile_x, tile_y, action) {
        Ok(result) => result,
        Err(err) => {
            error!("Error performing action: {}", err);
            return;
        }
    };
    debug!("Action result: {:?}", result);

    // We want to loop through all the tile and see which ones need to change.
    for (mut sprite, transform) in tile_query.iter_mut() {
        // Get the tile position from the transform.
        let tile_position = match game.world_to_tile(transform.translation.truncate()) {
            Some(position) => position,
            None => continue, // Skip if the tile position is not valid
        };

        // Based on the result, we want may want to update the tile sprite.
        match result {
            // If the game is over, we reveal all the tiles and show the game over screen.
            Response::GameOver | Response::GameWon => {
                debug!("Game Over!");
                let tile = game.tile(tile_position.0, tile_position.1);
                if tile.bomb {
                    sprite.image = asset_server.load(asset_path("bomb"));
                } else {
                    sprite.image = asset_server.load(asset_path_tile(
                        game.tile_number(tile_position.0, tile_position.1),
                    ));
                }
                game_state.set(GameState::GameOver);
            }

            // In this case, we reveal all the tiles in the revealed list.
            Response::Reveal(ref revealed_tiles) => {
                if revealed_tiles.contains(&tile_position) {
                    trace!("Revealed tile: ({}, {})", tile_position.0, tile_position.1);
                    sprite.image = asset_server.load(asset_path_tile(
                        game.tile_number(tile_position.0, tile_position.1),
                    ));
                }
            }

            // In this case, we flag the tile.
            Response::Flag => {
                if tile_x == tile_position.0 && tile_y == tile_position.1 {
                    sprite.image = asset_server.load(asset_path("flag"));
                    break;
                }
            }

            // In this case, we unflag the tile.
            Response::Unflag => {
                if tile_x == tile_position.0 && tile_y == tile_position.1 {
                    sprite.image = asset_server.load(asset_path("closed"));
                    break;
                }
            }
        }
    }
}
