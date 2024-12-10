use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::prelude::{screen_height, screen_width};
use macroquad::ui::root_ui;
use crate::utils::structs::Settings;
use crate::utils::enums::Scene;
use crate::utils::text::draw_text_centered;

/// This function gets executed if the Main Menu is set
pub async fn main_menu(scene: &mut Scene, settings: &Settings) {
    draw_text_centered("JumboMumbo", screen_height() / 8.0, 150.0 * settings.gui_scale, Color::from_rgba(255, 255, 255, 255)).await;

    // Executes the code inside the brackets
    if root_ui().button(Some(Vec2 { x: screen_width() / 2.0, y: screen_height() / 2.0 }), "Select Level") {
        // Sets the Scene to LevelSelector on page 0
        *scene = Scene::LevelSelector(0)
    }

    if root_ui().button(Some(Vec2 { x: screen_width() / 2.0, y: screen_height() / 2.0 + screen_height() / 4.0 }), "Settings") {
        *scene = Scene::SettingsMenu
    }
}