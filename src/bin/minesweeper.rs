use bevy::log::LogPlugin;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use clap::Parser;
use minesweeper::game::{Action, Game, Response};
use minesweeper::popup::{button_system, popup_window};
use minesweeper::{GameState, despawn_screen};

#[derive(Parser, Debug)]
#[clap(author = "The Marshians", version = "0.1.0", about = "Play Minesweeper!", long_about = None)]
struct Args {
    #[clap(short, long)]
    /// Show debug information.
    debug: bool,

    #[clap(short, long)]
    /// Show trace information.
    trace: bool,

    #[clap(short, long)]
    /// Show the world inspector.
    inspector: bool,
}

fn main() {
    let args = Args::parse();

    let mut log_level = bevy::log::Level::INFO;
    if args.debug {
        log_level = bevy::log::Level::DEBUG;
    } else if args.trace {
        log_level = bevy::log::Level::TRACE;
    }

    let default_plugins = DefaultPlugins.set(LogPlugin {
        level: log_level,
        ..default()
    });

    let mut app = App::new();
    app.add_plugins(default_plugins)
        .init_state::<GameState>()
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((Name::new("Camera2d"), Camera2d));
        })
        .add_plugins((MinesweeperPlugin, GameWonScreen, GameLostScreen));

    if args.inspector {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

#[derive(Component)]
struct GameWonScreen;

impl Plugin for GameWonScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameWon), |mut commands: Commands| {
            commands.spawn((GameWonScreen, popup_window("You Won!")));
        })
        .add_systems(Update, button_system.run_if(in_state(GameState::GameWon)))
        .add_systems(
            OnExit(GameState::GameWon),
            (
                despawn_screen::<GameWonScreen>,
                despawn_screen::<Game>,
                despawn_screen::<Sprite>,
            ),
        );
    }
}

#[derive(Component)]
struct GameLostScreen;

impl Plugin for GameLostScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameLost), |mut commands: Commands| {
            commands.spawn((GameLostScreen, popup_window("You Lost!")));
        })
        .add_systems(Update, button_system.run_if(in_state(GameState::GameLost)))
        // next_state.set(GameState::Playing);
        .add_systems(
            OnExit(GameState::GameLost),
            (
                despawn_screen::<GameLostScreen>,
                despawn_screen::<Game>,
                despawn_screen::<Sprite>,
            ),
        );
    }
}

#[derive(Component)]
struct MinesweeperPlugin;

impl Plugin for MinesweeperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), create_game)
            .add_systems(Update, update_game.run_if(in_state(GameState::Playing)));
    }
}

fn create_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game = Game::new(30, 16, 20);
    commands.spawn((Name::new("Game"), game.clone()));

    for row in 0..game.board.height {
        for column in 0..game.board.width {
            let tile_position = game.tile_position(column, row);
            commands.spawn((
                Name::new(format!("Cell ({}, {})", row, column)),
                Sprite::from_image(asset_server.load("closed.png")),
                Transform {
                    translation: Vec3::new(tile_position.x, tile_position.y, 1.0),
                    ..default()
                },
            ));
        }
    }
}

fn update_game(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut game_query: Query<&mut Game, With<Game>>,
    mut tile_query: Query<(&mut Sprite, &Transform), With<Sprite>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Right)
        && !mouse_button_input.just_pressed(MouseButton::Left)
    {
        return;
    }

    let mut game = match game_query.single_mut() {
        Ok(game) => game,
        Err(_) => return,
    };

    // Get the mouse position in window coordinates.
    let cursor_position = match window_query.single() {
        Ok(window) => match window.cursor_position() {
            Some(position) => position,
            None => return,
        },
        Err(_) => return,
    };

    // Convert window coordinates to world coordinates
    let world_position = game.window_to_world(window_query.single().unwrap(), cursor_position);

    let (tile_x, tile_y) = match game.world_to_tile(world_position) {
        Some((x, y)) => (x, y),
        None => return,
    };
    debug!("Clicked on tile: ({}, {})", tile_x, tile_y);

    // Determine the action based on the mouse button pressed
    let action = match mouse_button_input.just_pressed(MouseButton::Left) {
        true => Action::Reveal,
        false => Action::Flag,
    };

    let result = match game.perform_action(tile_x, tile_y, action) {
        Ok(result) => result,
        Err(err) => {
            error!("Error performing action: {}", err);
            return;
        }
    };
    debug!("Action result: {:?}", result);

    match result {
        Response::GameOver => {
            debug!("Game Over! You hit a mine.");
            for (mut sprite, transform) in tile_query.iter_mut() {
                let tile_position = match game.world_to_tile(transform.translation.truncate()) {
                    Some(position) => position,
                    None => continue, // Skip if the tile position is not valid
                };
                let tile = game.tile(tile_position.0, tile_position.1);
                if tile.bomb {
                    sprite.image = asset_server.load("bomb.png");
                } else {
                    sprite.image = asset_server.load(format!(
                        "{}.png",
                        game.tile_number(tile_position.0, tile_position.1)
                    ));
                }
            }
            game_state.set(GameState::GameLost);
        }
        Response::GameWon => {
            debug!("Congratulations! You won the game.");
            for (mut sprite, transform) in tile_query.iter_mut() {
                let tile_position = match game.world_to_tile(transform.translation.truncate()) {
                    Some(position) => position,
                    None => continue, // Skip if the tile position is not valid
                };
                let tile = game.tile(tile_position.0, tile_position.1);
                if tile.bomb {
                    sprite.image = asset_server.load("bomb.png");
                } else {
                    sprite.image = asset_server.load(format!(
                        "{}.png",
                        game.tile_number(tile_position.0, tile_position.1)
                    ));
                }
            }
            game_state.set(GameState::GameWon);
        }
        Response::Reveal(tiles) => {
            for (mut sprite, transform) in tile_query.iter_mut() {
                let tile_position = match game.world_to_tile(transform.translation.truncate()) {
                    Some(position) => position,
                    None => continue, // Skip if the tile position is not valid
                };
                if tiles.contains(&tile_position) {
                    trace!("Revealed tile: ({}, {})", tile_position.0, tile_position.1);
                    sprite.image = asset_server.load(format!(
                        "{}.png",
                        game.tile_number(tile_position.0, tile_position.1)
                    ));
                }
            }
        }
        Response::Flag => {
            for (mut sprite, transform) in tile_query.iter_mut() {
                let tile_position = match game.world_to_tile(transform.translation.truncate()) {
                    Some(position) => position,
                    None => continue, // Skip if the tile position is not valid
                };
                if tile_x == tile_position.0 && tile_y == tile_position.1 {
                    // Change the color of the clicked tile to blue
                    sprite.image = asset_server.load("flag.png");
                    break;
                }
            }
        }
        Response::Unflag => {
            for (mut sprite, transform) in tile_query.iter_mut() {
                // Get the tile position from the transform
                let tile_position = match game.world_to_tile(transform.translation.truncate()) {
                    Some(position) => position,
                    None => continue, // Skip if the tile position is not valid
                };
                if tile_x == tile_position.0 && tile_y == tile_position.1 {
                    // Change the color of the clicked tile to blue
                    sprite.image = asset_server.load("closed.png");
                    break;
                }
            }
        }
    }
}
