use std::collections::BTreeMap;
use std::process::exit;
use macroquad::color::Color;
use macroquad::input::{is_key_pressed, KeyCode, MouseButton};
use macroquad::math::vec2;
use macroquad::prelude::{screen_height, screen_width, Texture2D};
use macroquad::text::measure_text;
use stopwatch2::Stopwatch;
use crate::ui::buttons::Button;
use crate::utils::structs::Settings;
use crate::utils::enums::{Scene, SceneTextureKey, TextureKey};
use crate::utils::text::draw_text_centered;
use crate::utils::texture::{get_texture_path, load_textures_from_tile_map};

/// This function gets executed if the Main Menu is set
pub async fn main_menu(scene: &mut Scene, textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, settings: &Settings) {
    if !textures.contains_key(&SceneTextureKey::MainMenu) {
        textures.insert(SceneTextureKey::MainMenu, load_textures().await);
    }

    let title_measurements = measure_text("JumboMumbo", None, (150.0 * settings.gui_scale) as u16, 1.0);
    draw_text_centered("JumboMumbo", screen_height() / 16.0 + title_measurements.offset_y, 150.0 * settings.gui_scale, Color::from_rgba(255, 255, 255, 255)).await;

    {
        let y_offset = screen_height() / 16.0 + title_measurements.height;
        let size = vec2(400.0, 200.0) * settings.gui_scale;
        let border_size = vec2(64.0, 64.0) * settings.gui_scale;
        let font_size = 64.0 * settings.gui_scale;
        let gap = screen_height() / 64.0;


        let level_selector_button = Button::new(
            vec2(screen_width() / 2.0 - size.x / 2.0, screen_height() / 2.0 - size.y / 2.0 - size.y - gap + y_offset),
            size,
            border_size,
            "Levels".to_string(),
            font_size,
            TextureKey::Button0,
        ).await;
        level_selector_button.render(textures.get(&SceneTextureKey::MainMenu).unwrap()).await;
        if level_selector_button.is_released(MouseButton::Left).await || is_key_pressed(KeyCode::L) {
            *scene = Scene::LevelSelector(0);
            textures.remove(&SceneTextureKey::MainMenu);
            return;
        }

        let settings_button = Button::new(
            vec2(screen_width() / 2.0 - size.x / 2.0, screen_height() / 2.0 - size.y / 2.0 + y_offset),
            size,
            border_size,
            "Settings".to_string(),
            font_size,
            TextureKey::Button0,
        ).await;
        settings_button.render(textures.get(&SceneTextureKey::MainMenu).unwrap()).await;
        if settings_button.is_released(MouseButton::Left).await || is_key_pressed(KeyCode::S){
            *scene = Scene::SettingsMenu;
            textures.remove(&SceneTextureKey::MainMenu);
            return;
        }

        let credits_button = Button::new(
            vec2(screen_width() / 2.0 - size.x / 2.0, screen_height() / 2.0 - size.y / 2.0 + size.y + gap + y_offset),
            size,
            border_size,
            "Credits".to_string(),
            font_size,
            TextureKey::Button0,
        ).await;
        credits_button.render(textures.get(&SceneTextureKey::MainMenu).unwrap()).await;
        if credits_button.is_released(MouseButton::Left).await || is_key_pressed(KeyCode::C) {
            *scene = Scene::Credits(0.0);
            textures.remove(&SceneTextureKey::MainMenu);
            return;
        }
    }

    let exit_button = Button::new(
        vec2(0.0, 0.0),
        vec2(256.0, 128.0) * settings.gui_scale,
        vec2(48.0, 48.0) * settings.gui_scale,
        "Exit".to_string(),
        64.0 * settings.gui_scale,
        TextureKey::Button0
    ).await;

    exit_button.render(textures.get(&SceneTextureKey::MainMenu).unwrap()).await;
    if exit_button.is_released(MouseButton::Left).await { exit(0) }
}

async fn load_textures() -> BTreeMap<TextureKey, Vec<Texture2D>> {
    let mut stopwatch = Stopwatch::default();
    println!("Loading main menu textures...");
    stopwatch.start();
    let mut result = BTreeMap::new();

    let button_textures = {
        let path = get_texture_path(TextureKey::Button0).await;
        load_textures_from_tile_map(path)
    };

    result.insert(TextureKey::Button0, button_textures.await);

    stopwatch.stop();
    println!("Loaded main menu textures! Took {}ms", stopwatch.elapsed().as_millis());

    result
}