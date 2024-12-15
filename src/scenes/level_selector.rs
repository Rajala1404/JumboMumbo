use macroquad::color::Color;
use macroquad::input::is_key_down;
use macroquad::math::Vec2;
use macroquad::prelude::{screen_height, screen_width, KeyCode};
use macroquad::ui::root_ui;
use crate::utils::enums::Scene;
use crate::logic::level::Level;
use crate::utils::text::draw_text_centered;

pub async fn level_selector(scene: &mut Scene) {
    draw_text_centered("Level Selector", screen_height() / 8.0, screen_height() / 8.0, Color::from_rgba(255, 255, 255, 255)).await;

    // Executes the code inside the brackets and sets the Scene to Level with id 0
    if root_ui().button(Some(Vec2 { x: screen_width() / 2.0, y: screen_height() / 2.0 }), "Tutorial") {
        *scene = Scene::Level(Level::Level0)
    }

    if is_key_down(KeyCode::Escape) {
        *scene = Scene::MainMenu
    }
}