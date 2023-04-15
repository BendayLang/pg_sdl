use crate::point;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn fill_circle(canvas: &mut Canvas<Window>, center: Point, radius: i32, color: Color) {
    let (mut offsetx, mut offsety, mut d) = (0, radius, radius - 1);
    canvas.set_draw_color(color);
    while offsety >= offsetx {
        canvas
            .draw_line(
                point!(center.x - offsetx, center.y + offsety),
                point!(center.x + offsetx, center.y + offsety),
            )
            .unwrap();

        canvas
            .draw_line(
                point!(center.x - offsetx, center.y - offsety),
                point!(center.x + offsetx, center.y - offsety),
            )
            .unwrap();
        canvas
            .draw_line(
                point!(center.x - offsety, center.y - offsetx),
                point!(center.x + offsety, center.y - offsetx),
            )
            .unwrap();
        canvas
            .draw_line(
                point!(center.x - offsety, center.y + offsetx),
                point!(center.x + offsety, center.y + offsetx),
            )
            .unwrap();

        if d >= 2 * offsetx {
            d -= 2 * offsetx + 1;
            offsetx += 1;
        } else if d < 2 * (radius - offsety) {
            d += 2 * offsety - 1;
            offsety -= 1;
        } else {
            d += 2 * (offsety - offsetx - 1);
            offsety -= 1;
            offsetx += 1;
        }
    }
}

pub fn draw_circle(canvas: &mut Canvas<Window>, center: Point, radius: i32, color: Color) {
    let (mut offsetx, mut offsety, mut d) = (0, radius, radius - 1);
    canvas.set_draw_color(color);
    while offsety >= offsetx {
        canvas
            .draw_point(point!(center.x + offsetx, center.y + offsety))
            .unwrap();
        canvas
            .draw_point(point!(center.x + offsety, center.y + offsetx))
            .unwrap();
        canvas
            .draw_point(point!(center.x - offsetx, center.y + offsety))
            .unwrap();
        canvas
            .draw_point(point!(center.x - offsety, center.y + offsetx))
            .unwrap();
        canvas
            .draw_point(point!(center.x + offsetx, center.y - offsety))
            .unwrap();
        canvas
            .draw_point(point!(center.x + offsety, center.y - offsetx))
            .unwrap();
        canvas
            .draw_point(point!(center.x - offsetx, center.y - offsety))
            .unwrap();
        canvas
            .draw_point(point!(center.x - offsety, center.y - offsetx))
            .unwrap();

        if d >= 2 * offsetx {
            d -= 2 * offsetx + 1;
            offsetx += 1;
        } else if d < 2 * (radius - offsety) {
            d += 2 * offsety - 1;
            offsety -= 1;
        } else {
            d += 2 * (offsety - offsetx - 1);
            offsety -= 1;
            offsetx += 1;
        }
    }
}
