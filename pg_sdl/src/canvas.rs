use crate::{point, rect};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub fn fill_background(canvas: &mut Canvas<Window>, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
}

pub fn draw_rect(canvas: &mut Canvas<Window>, rect: Rect) {
    canvas.draw_rect(rect).unwrap();
}

pub fn draw_rounded_rect(canvas: &mut Canvas<Window>, rect: Rect, radius: u32) {
    assert!(
        rect.width().min(rect.height()) / 2 > radius,
        "Radius ({}) is too big for rounded rectangle with size ({}, {}) !",
        radius,
        rect.width(),
        rect.height()
    );

    canvas.draw_rect(rect).unwrap();
}

pub fn fill_rect(canvas: &mut Canvas<Window>, rect: Rect, radius: Option<u32>) {
    if let Some(mut radius) = radius {
        if 2 * radius + 1 >= rect.width() && 2 * radius >= rect.height() {
            radius = rect.width().min(rect.height()) / 2 - 1;
        }

        if 2 * radius + 1 >= rect.width() {
            let r = (rect.w / 2) as i32;

            canvas
                .fill_rect(rect!(rect.x, rect.y + rect.w / 2, rect.w, rect.h - rect.w))
                .unwrap();

            let top = rect.top_left() + point!(r, r);
            let bottom = rect.bottom_left() + point!(r, -(r + 1));
            let (mut x, mut y, mut d) = (0, r, r - 1);

            while y >= x {
                for (start_point, end_point) in [
                    (top + point!(-x, -y), top + point!(x - 1, -y)),
                    (top + point!(-y, -x), top + point!(y - 1, -x)),
                    (bottom + point!(-x, y), bottom + point!(x - 1, y)),
                    (bottom + point!(-y, x), bottom + point!(y - 1, x)),
                ] {
                    canvas.draw_line(start_point, end_point).unwrap();
                }

                if d >= 2 * x {
                    d -= 2 * x + 1;
                    x += 1;
                } else if d < 2 * (r - y) {
                    d += 2 * y - 1;
                    y -= 1;
                } else {
                    d += 2 * (y - x - 1);
                    y -= 1;
                    x += 1;
                }
            }
        } else if 2 * radius + 1 >= rect.height() {
            let r = (rect.h / 2 - 1) as i32;

            canvas
                .fill_rect(rect!(rect.x + rect.h / 2, rect.y, rect.w - rect.h, rect.h))
                .unwrap();

            let left = rect.top_left() + point!(r, r);
            let right = rect.top_right() + point!(-(r + 1), r);
            let (mut x, mut y, mut d) = (0, r, r - 1);

            while y >= x {
                for (start_point, end_point) in [
                    (left + point!(-x, y + 1), left + point!(-x, -y)),
                    (left + point!(-y, x + 1), left + point!(-y, -x)),
                    (right + point!(x, y + 1), right + point!(x, -y)),
                    (right + point!(y, x + 1), right + point!(y, -x)),
                ] {
                    canvas.draw_line(start_point, end_point).unwrap();
                }

                if d >= 2 * x {
                    d -= 2 * x + 1;
                    x += 1;
                } else if d < 2 * (r - y) {
                    d += 2 * y - 1;
                    y -= 1;
                } else {
                    d += 2 * (y - x - 1);
                    y -= 1;
                    x += 1;
                }
            }
        } else {
            let r = radius as i32;

            canvas
                .fill_rect(rect!(rect.x, rect.y + r + 1, rect.w, rect.h - 2 * r - 2))
                .unwrap();

            canvas
                .fill_rect(rect!(rect.x + r + 1, rect.y, rect.w - 2 * r - 2, r + 1))
                .unwrap();
            canvas
                .fill_rect(rect!(
                    rect.x + r + 1,
                    rect.y + rect.h - r - 1,
                    rect.w - 2 * r - 2,
                    r + 1
                ))
                .unwrap();

            let top_left = rect.top_left() + point!(r, r);
            let top_right = rect.top_right() + point!(-(r + 1), r);
            let bottom_left = rect.bottom_left() + point!(r, -(r + 1));
            let bottom_right = rect.bottom_right() + point!(-(r + 1), -(r + 1));
            let (mut x, mut y, mut d) = (0, r, r - 1);

            while y >= x {
                for (start_point, end_point) in [
                    (top_left + point!(-x, -y), top_left + point!(0, -y)),
                    (top_left + point!(-y, -x), top_left + point!(0, -x)),
                    (top_right + point!(0, -y), top_right + point!(x, -y)),
                    (top_right + point!(0, -x), top_right + point!(y, -x)),
                    (bottom_left + point!(-x, y), bottom_left + point!(0, y)),
                    (bottom_left + point!(-y, x), bottom_left + point!(0, x)),
                    (bottom_right + point!(0, y), bottom_right + point!(x, y)),
                    (bottom_right + point!(0, x), bottom_right + point!(y, x)),
                ] {
                    canvas.draw_line(start_point, end_point).unwrap();
                }

                if d >= 2 * x {
                    d -= 2 * x + 1;
                    x += 1;
                } else if d < 2 * (r - y) {
                    d += 2 * y - 1;
                    y -= 1;
                } else {
                    d += 2 * (y - x - 1);
                    y -= 1;
                    x += 1;
                }
            }
        }
    } else {
        canvas.fill_rect(rect).unwrap()
    }
}
