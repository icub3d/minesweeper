use bevy::color::Color;
use catppuccin::PALETTE;

/// A helper function that converts a catppuccin color to a Bevy color.
const fn color_convert(original: catppuccin::Color, alpha: f32) -> Color {
    Color::hsla(
        original.hsl.h as f32,
        original.hsl.s as f32,
        original.hsl.l as f32,
        alpha,
    )
}

/// Colors from the Catppuccin Mocha palette.
pub const GREEN: Color = color_convert(PALETTE.mocha.colors.green, 1.0);
pub const RED: Color = color_convert(PALETTE.mocha.colors.red, 1.0);
pub const FLAMINGO: Color = color_convert(PALETTE.mocha.colors.flamingo, 1.0);
pub const CRUST: Color = color_convert(PALETTE.mocha.colors.crust, 1.0);
pub const BASE_80: Color = color_convert(PALETTE.mocha.colors.base, 0.8);