use macroquad::camera::{set_camera, set_default_camera, Camera2D};
use macroquad::color::WHITE;
use macroquad::input::{is_key_pressed, mouse_wheel, KeyCode};
use macroquad::math::Rect;
use macroquad::time::get_frame_time;
use macroquad::window::{screen_height, screen_width};
use crate::utils::enums::Scene;
use crate::utils::structs::Settings;
use crate::utils::text::draw_text_centered;

pub async fn credits(scene: &mut Scene, settings: &Settings) {
    if is_key_pressed(KeyCode::Escape) {
        set_default_camera();
        *scene = Scene::MainMenu;
        return;
    }

    match scene {
        Scene::Credits(y_offset) => {
            *y_offset += mouse_wheel().1 * -2000.0 * settings.gui_scale * get_frame_time();
            set_camera(&Camera2D::from_display_rect(Rect::new(0.0, y_offset.to_owned() + screen_height(), screen_width(), -screen_height())));
        }
        _ => unreachable!(),
    }

    let font_size = 128.0 * settings.gui_scale;
    draw_text_centered("Credits (Scrolling)", screen_height() / 8.0, font_size, WHITE).await;

    let mut current_y = screen_height() / 8.0 + font_size + screen_height() / 16.0;
    let font_size = 96.0 * settings.gui_scale;

    draw_text_centered("Play testing - Gopiler", current_y, font_size, WHITE).await;
    current_y += font_size * 2.0;
    draw_text_centered("Player textures - Dinno", current_y, font_size, WHITE).await;
    current_y += font_size * 2.0;
    draw_text_centered("Fireball texture - Dinno", current_y, font_size, WHITE).await;
    current_y += font_size * 2.0;
    draw_text_centered("Enemy texture - Dinno", current_y, font_size, WHITE).await;
    current_y += font_size * 2.0;
    draw_text_centered("Level 2 Design - Kinglui2000 & Gopiler", current_y, font_size, WHITE).await;
    current_y += font_size * 2.0;
    draw_text_centered("Level 3 Design - Fossombrome", current_y, font_size, WHITE).await;

}