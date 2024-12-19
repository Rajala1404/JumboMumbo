use macroquad::prelude::*;

/// Draws text in screen center
pub async fn draw_text_center(text: &str, font_size: f32, color: Color) {
    let size = measure_text(text, None, font_size as _, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - size.width / 2.0,
        screen_height() / 2.0 + size.offset_y / 2.0,
        font_size,
        color
    );
}

/// Draws text in the centered x
pub async fn draw_text_centered(text: &str, y: f32, font_size: f32, color: Color) {
    let size = measure_text(text, None, font_size as _, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - size.width / 2.0,
        y + size.offset_y,
        font_size,
        color
    );
}