use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::prelude::{screen_height, screen_width};
use macroquad::ui::root_ui;
use crate::Scene;
use crate::utils::text::draw_text_centered;

/// This function gets executed if the Main Menu is set
pub async fn main_menu(scene: &mut Scene) {
    draw_text_centered("MumboJumbo", screen_height() / 8.0, screen_height() / 8.0, Color::from_rgba(255, 255, 255, 255)).await;

    // Executes the code inside the brackets and sets the Scene to LevelSelector on page 0
    if root_ui().button(Some(Vec2 { x: screen_width() / 2.0, y: screen_height() / 2.0 }), "Select Level") {
        *scene = Scene::LevelSelector(0)
    }
}