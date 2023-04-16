/**
Returns a vector of fonts.
The fonts are loaded from the system at compile time (using the `include_bytes!` macro).
*/
pub fn fonts_init() -> Vec<fontdue::Font> {
    macros::init_fonts!(
        "/usr/share/fonts/TTF",
        "Vera.ttf",
        "VeraBd.ttf",
        "VeraIt.ttf",
    )
}
