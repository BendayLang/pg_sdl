use crate::{draw_circle, point, rect};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub fn fill_background(canvas: &mut Canvas<Window>, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
}

/// take optional rounded corners from 0 to 1
pub fn fill_rect(canvas: &mut Canvas<Window>, rect: Rect, color: Color, rounded: Option<f32>) {
    canvas.set_draw_color(color);
    if let Some(radius) = rounded {
        let wird_shift = 2;
        let radius = (radius * (rect.h as f32) / 2.) as i32 - 4;
        // first draw the rounded corners
        let corner = rect.top_left();
        draw_circle::fill_circle(
            canvas,
            point!(corner.x + radius, corner.y + radius),
            radius,
            color,
        );
        let corner = rect.top_right();
        draw_circle::fill_circle(
            canvas,
            point!(corner.x - radius - wird_shift, corner.y + radius),
            radius,
            color,
        );
        let corner = rect.bottom_left();
        draw_circle::fill_circle(
            canvas,
            point!(corner.x + radius, corner.y - radius - wird_shift),
            radius,
            color,
        );
        let corner = rect.bottom_right();
        draw_circle::fill_circle(
            canvas,
            point!(
                corner.x - radius - wird_shift,
                corner.y - radius - wird_shift
            ),
            radius,
            color,
        );

        // then draw the rectangles
        canvas
            .fill_rect(rect!(rect.x + radius, rect.y, rect.w - radius * 2, rect.h))
            .unwrap();
        canvas
            .fill_rect(rect!(rect.x, rect.y + radius, rect.w, rect.h - radius * 2))
            .unwrap();
    } else {
        canvas.fill_rect(rect).unwrap();
    }
}
