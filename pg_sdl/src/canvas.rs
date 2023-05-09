use crate::prelude::*;

pub fn fill_background(canvas: &mut Canvas<Window>, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
}

pub fn draw_rect(canvas: &mut Canvas<Window>, rect: Rect, color: Color, radius: Option<u16>) {
    if let Some(radius) = radius {
        DrawRenderer::rounded_rectangle(
            canvas,
            rect.left() as i16,
            rect.top() as i16,
            rect.right() as i16,
            rect.bottom() as i16,
            radius as i16,
            color,
        )
    } else {
        canvas.set_draw_color(color);
        canvas.draw_rect(rect)
    }
    .expect("DrawRenderer failed");
}

pub fn fill_rect(canvas: &mut Canvas<Window>, rect: Rect, color: Color, radius: Option<u16>) {
    if let Some(radius) = radius {
        DrawRenderer::rounded_box(
            canvas,
            rect.left() as i16,
            rect.top() as i16,
            rect.right() as i16,
            rect.bottom() as i16,
            radius as i16,
            color,
        )
    } else {
        canvas.set_draw_color(color);
        canvas.fill_rect(rect)
    }
    .expect("DrawRenderer failed");
}
