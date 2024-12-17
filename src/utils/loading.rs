use macroquad::color::{Color, BLACK, WHITE};
use macroquad::prelude::{draw_line, screen_height, screen_width};
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame};
use crate::utils::mathemann::stretch_float_to;
use crate::utils::text::{draw_text_center, draw_text_centered};

pub async fn show_loading_screen(previous_progress: f32, progress: f32, text: &str) {
    let width = screen_width() / 4.0;
    let height = screen_height() / 1.5;

    let mut current_progress = previous_progress;

    clear_background(BLACK);
    draw_text_center(text, screen_height() / 8.0, Color::from_rgba(255, 255, 255, 255)).await;
    draw_text_centered("Loading...", screen_height() / 4.0, screen_height() / 16.0, Color::from_rgba(255, 255, 255, 255)).await;

    let length = width + stretch_float_to(previous_progress, 100.0, width * 2.0).await;

    draw_line(width, height, length, height, screen_height() / 32.0, WHITE);
    next_frame().await;

    clear_background(BLACK);
    draw_text_center(text, screen_height() / 8.0, Color::from_rgba(255, 255, 255, 255)).await;
    draw_text_centered("Loading...", screen_height() / 4.0, screen_height() / 16.0, Color::from_rgba(255, 255, 255, 255)).await;

    let length = width + stretch_float_to(previous_progress, 100.0, width * 2.0).await;

    draw_line(width, height, length, height, screen_height() / 32.0, WHITE);
    next_frame().await;

    while current_progress < progress {
        current_progress += 150.0 * get_frame_time();

        clear_background(BLACK);
        draw_text_center(text, screen_height() / 8.0, Color::from_rgba(255, 255, 255, 255)).await;
        draw_text_centered("Loading...", screen_height() / 4.0, screen_height() / 16.0, Color::from_rgba(255, 255, 255, 255)).await;

        let length = width + stretch_float_to(current_progress, 100.0, width * 2.0).await;

        draw_line(width, height, length, height, screen_height() / 32.0, WHITE);

        next_frame().await;
    }

}