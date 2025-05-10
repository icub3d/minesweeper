mod assets;
mod game;
mod game_over_plugin;
mod states;
mod minesweeper_plugin;
mod popup;
mod colors;

use assets::EmbeddedAssetsPlugin;
use game_over_plugin::GameOverPlugin;
use minesweeper_plugin::MinesweeperPlugin;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use clap::Parser;

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

    // Setup console logging for debugging.
    let log_level = match (args.debug, args.trace) {
        (_, true) => Level::TRACE,
        (true, _) => Level::DEBUG,
        _ => Level::INFO,
    };

    // Initialize our app.
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: log_level,
        ..default()
    }))
    .add_systems(Startup, |mut commands: Commands| {
        commands.spawn((Name::new("Camera2d"), Camera2d));
    })
    .add_plugins((EmbeddedAssetsPlugin, MinesweeperPlugin, GameOverPlugin));

    // Optionally add the inspector.
    if args.inspector {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new());
    }

    // Run the app.
    app.run();
}