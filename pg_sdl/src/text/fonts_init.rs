/**
Returns a vector of fonts.
The fonts are loaded from the system at compile time (using the `include_bytes!` macro).
*/
#[cfg(windows)]
pub fn fonts_init() -> Vec<fontdue::Font> {
    macros::init_fonts!(
        "C:/Users/arnol/PycharmProjects/LibTests/venv/Lib/site-packages/kivy/data/fonts",
        "DejaVuSans.ttf",
        "DejaVuSans.ttf",
    )
}

#[cfg(unix)]
pub fn fonts_init() -> Vec<fontdue::Font> {
    macros::init_fonts!(
        "/usr/share/fonts/TTF",
        "Vera.ttf",
        "VeraBd.ttf",
        "VeraIt.ttf",
    )
}
