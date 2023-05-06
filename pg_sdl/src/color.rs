use itertools::{max, min};
use sdl2::pixels::Color;

pub struct Colors;

impl Colors {
    pub const WHITE: Color = Color::RGB(255, 255, 255);
    pub const LIGHT_GREY: Color = Color::RGB(191, 191, 191);
    pub const GREY: Color = Color::RGB(127, 127, 127);
    pub const DARK_GREY: Color = Color::RGB(63, 63, 63);
    pub const BLACK: Color = Color::RGB(0, 0, 0);

    pub const RED: Color = Color::RGB(255, 0, 0);
    pub const RED_ORANGE: Color = Color::RGB(255, 63, 0);
    pub const ORANGE: Color = Color::RGB(255, 127, 0);
    pub const AMBER: Color = Color::RGB(255, 191, 0);
    pub const YELLOW: Color = Color::RGB(255, 255, 0);

    pub const CHARTREUSE: Color = Color::RGB(127, 255, 0);
    pub const GREEN: Color = Color::RGB(0, 255, 0);
    pub const LIME: Color = Color::RGB(0, 255, 127);
    pub const CYAN: Color = Color::RGB(0, 255, 255);

    pub const AZURE: Color = Color::RGB(0, 127, 255);
    pub const BLUE: Color = Color::RGB(0, 0, 255);
    pub const VIOLET: Color = Color::RGB(127, 0, 255);
    pub const MAGENTA: Color = Color::RGB(255, 0, 255);
    pub const PINK: Color = Color::RGB(255, 0, 127);

    pub const SKY_BLUE: Color = Color::RGB(135, 206, 235);
    pub const ROYAL_BLUE: Color = Color::RGB(65, 105, 225);
    pub const BEIGE: Color = Color::RGB(255, 240, 200);
}

/// - hue (0-360)
/// - saturation (0.0 - 1.0)
/// - value (0.0 - 1.0)
pub fn hsv_color(hue: u16, saturation: f32, value: f32) -> Color {
    let c: f32 = saturation * value;
    let x: f32 = c * (1.0 - ((hue as f32 / 60.0) % 2.0 - 1.0).abs());
    let m: f32 = value - c;

    let (r, g, b): (f32, f32, f32) = if hue < 60 {
        (c, x, 0.0)
    } else if hue < 120 {
        (x, c, 0.0)
    } else if hue < 180 {
        (0.0, c, x)
    } else if hue < 240 {
        (0.0, x, c)
    } else if hue < 300 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color {
        r: ((m + r) * 255.0) as u8,
        g: ((m + g) * 255.0) as u8,
        b: ((m + b) * 255.0) as u8,
        a: 255,
    }
}

fn color_to_hsv(color: Color) -> (u16, f32, f32) {
    let (r, g, b) = color.rgb();
    let c_max = max([r, g, b]).unwrap();
    let c_min = min([r, g, b]).unwrap();

    let r2 = r as f32 / 255.0;
    let g2 = g as f32 / 255.0;
    let b2 = b as f32 / 255.0;

    let delta = (c_max - c_min) as f32 / 255.0;

    let hue = if delta == 0.0 {
        0.0
    } else if c_max == r {
        60.0 * (((g2 - b2) / delta).rem_euclid(6.0))
    } else if c_max == g {
        60.0 * (((b2 - r2) / delta) + 2.0)
    } else {
        60.0 * (((r2 - g2) / delta) + 4.0)
    };

    let saturation = if c_max == 0 {
        0.0
    } else {
        delta / (c_max as f32 / 255.0)
    };
    let value = c_max as f32 / 255.0;
    (hue as u16, saturation, value)
}

pub fn darker(color: Color, value_change: f32) -> Color {
    Color::RGB(
        (color.r as f32 * value_change) as u8,
        (color.g as f32 * value_change) as u8,
        (color.b as f32 * value_change) as u8,
    )
}

pub fn paler(color: Color, saturation_change: f32) -> Color {
    let (r, g, b) = color.rgb();
    let c_max = max([r, g, b]).unwrap() as f32 / 255.0;
    if c_max == 0.0 {
        return Color::BLACK;
    }
    let c_min = min([r, g, b]).unwrap() as f32 / 255.0;

    let saturation = c_max * (1.0 - saturation_change);

    let color = Color::RGB(
        r + ((c_max - r as f32 / 255.0) * saturation * 255.0) as u8,
        g + ((c_max - g as f32 / 255.0) * saturation * 255.0) as u8,
        b + ((c_max - b as f32 / 255.0) * saturation * 255.0) as u8,
    );

    color
}
