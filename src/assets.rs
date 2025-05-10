use bevy::app::Plugin;
use bevy::asset::embedded_asset;

pub const ASSET_PREFIX: &str = "embedded://minesweeper/assets/";

/// The `EmbeddedAssetsPlugin` plugin is used to load embedded assets into the Bevy application.
pub struct EmbeddedAssetsPlugin;

impl Plugin for EmbeddedAssetsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        embedded_asset!(app, "", "assets/0.png");
        embedded_asset!(app, "", "assets/1.png");
        embedded_asset!(app, "", "assets/2.png");
        embedded_asset!(app, "", "assets/3.png");
        embedded_asset!(app, "", "assets/4.png");
        embedded_asset!(app, "", "assets/5.png");
        embedded_asset!(app, "", "assets/6.png");
        embedded_asset!(app, "", "assets/7.png");
        embedded_asset!(app, "", "assets/8.png");
        embedded_asset!(app, "", "assets/flag.png");
        embedded_asset!(app, "", "assets/closed.png");
        embedded_asset!(app, "", "assets/bomb.png");
    }
}

/// Returns the path to the asset with the given file name.
/// The file name should not include the `.png` extension.
pub fn asset_path(file_name: &str) -> String {
    format!("{}/{}.png", ASSET_PREFIX, file_name)
}

/// Returns the path to the tile asset with the given number.
/// The number should be between 0 and 8, inclusive.
pub fn asset_path_tile(number: u8) -> String {
    format!("{}/{}.png", ASSET_PREFIX, number)
}
