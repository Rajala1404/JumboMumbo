use std::collections::BTreeMap;
use macroquad::color::Color;
use macroquad::hash;
use macroquad::input::{is_key_pressed, KeyCode, MouseButton};
use macroquad::math::vec2;
use macroquad::prelude::{screen_height, Texture2D};
use macroquad::ui::{root_ui, widgets};
use macroquad::window::screen_width;
use stopwatch2::Stopwatch;
use crate::ui::buttons::Button;
use crate::utils::enums::{Scene, SceneTextureKey, TextureKey};
use crate::utils::structs::{Settings, TempSettings};
use crate::utils::text::draw_text_centered;
use crate::utils::texture::{get_texture_path, load_textures_from_tile_map};

pub async fn settings_menu(scene: &mut Scene, textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, settings: &mut Settings, temp_settings: &mut TempSettings) {
    if !textures.contains_key(&SceneTextureKey::SettingsMenu) {
        textures.insert(SceneTextureKey::SettingsMenu, load_textures().await);
    }

    draw_text_centered("JumboMumbo", screen_height() / 8.0, 150.0 * settings.gui_scale, Color::from_rgba(255, 255, 255, 255)).await;

    if is_key_pressed(KeyCode::Escape) {
        *scene = Scene::MainMenu;
        textures.remove(&SceneTextureKey::SettingsMenu);
        return;
    }

    let textures = textures.get(&SceneTextureKey::SettingsMenu).unwrap();

    let size = vec2(1000.0 * settings.gui_scale, 1000.0 * settings.gui_scale);
    let pos = vec2(screen_width() / 2.0 - size.x / 2.0, screen_height() / 2.0 - size.y / 2.0);
    root_ui().window(hash!(), pos, size, |ui| {
        widgets::Slider::new(
            hash!(),
            0.1..4.0
        ).ui(ui, &mut temp_settings.settings.gui_scale);
    });

    { // Apply Button
        let size = vec2(512.0, 256.0) * settings.gui_scale;
        let pos = vec2(screen_width(), screen_height()) - size;
        let button = Button::new(
            pos,
            size,
            vec2(64.0, 64.0) * settings.gui_scale,
            "Apply".to_string(),
            128.0 * settings.gui_scale,
            TextureKey::Button0,
        ).await;

        button.render(textures).await;

        if button.is_released(MouseButton::Left).await {
            *settings = temp_settings.clone().settings;
            settings.save().await;
        }
    }
}

async fn load_textures() -> BTreeMap<TextureKey, Vec<Texture2D>> {
    let mut stopwatch = Stopwatch::default();
    println!("Loading settings menu textures...");
    stopwatch.start();
    let mut result = BTreeMap::new();

    let button_textures = {
        let path = get_texture_path(TextureKey::Button0).await;
        load_textures_from_tile_map(path)
    };



    result.insert(TextureKey::Button0, button_textures.await);

    stopwatch.stop();
    println!("Loaded settings menu textures! Took {}ms", stopwatch.elapsed().as_millis());

    result
}