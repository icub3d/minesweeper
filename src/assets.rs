use bevy::app::Plugin;
use bevy::asset::embedded_asset;

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
