#[macro_export]
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[macro_export]
macro_rules! point(
    ($x:expr, $y:expr) => (
        sdl2::rect::Point::new($x as i32, $y as i32)
    )
);
