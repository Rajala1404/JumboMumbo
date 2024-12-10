use macroquad::color::Color;
use macroquad::hash;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::screen_height;
use macroquad::ui::{root_ui, widgets};
use macroquad::window::screen_width;
use crate::utils::enums::Scene;
use crate::utils::structs::{Settings, TempSettings};
use crate::utils::text::draw_text_centered;

pub async fn settings_menu(scene: &mut Scene, settings: &mut Settings, temp_settings: &mut TempSettings) {
    draw_text_centered("MumboJumbo", screen_height() / 8.0, 150.0 * settings.gui_scale, Color::from_rgba(255, 255, 255, 255)).await;

    if root_ui().button(Some(Vec2 {x: 0.0, y: 0.0}), "X") {
        *scene = Scene::MainMenu
    }

    let size = vec2(1000.0 * settings.gui_scale, 1000.0 * settings.gui_scale);
    let pos = vec2(screen_width() / 2.0 - size.x / 2.0, screen_height() / 2.0 - size.y / 2.0);
    root_ui().window(hash!(), pos, size, |ui| {
        widgets::Slider::new(
            hash!(),
            0.1..4.0
        ).ui(ui, &mut temp_settings.settings.gui_scale);
    });

    if root_ui().button(vec2(screen_width() / 2.0, (size.y - pos.y) + screen_height() / 4.0), "Apply") {
        *settings = temp_settings.clone().settings
    }
}