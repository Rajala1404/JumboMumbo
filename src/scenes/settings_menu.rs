use std::collections::BTreeMap;
use macroquad::color::{Color, WHITE};
use macroquad::input::{is_key_pressed, KeyCode, MouseButton};
use macroquad::math::vec2;
use macroquad::prelude::{measure_text, screen_height, Texture2D};
use macroquad::text::draw_text;
use macroquad::window::screen_width;
use stopwatch2::Stopwatch;
use crate::ui::buttons::Button;
use crate::utils::enums::{Scene, SceneTextureKey, TextureKey};
use crate::utils::mathemann::round;
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

    { // GUI Scale
        let size = vec2(64.0, 64.0) * settings.gui_scale;
        let border_size = vec2(16.0, 16.0) * settings.gui_scale;
        let y = screen_width() / 4.0;
        let plus_button = Button::new(
            vec2(screen_width() - size.x, y),
            size,
            border_size,
            "+".to_string(),
            64.0 * settings.gui_scale,
            TextureKey::Button0
        ).await;
        let minus_button = Button::new(
            vec2(screen_width() - size.x * 2.25, y),
            size,
            border_size,
            "-".to_string(),
            64.0 * settings.gui_scale,
            TextureKey::Button0
        ).await;

        plus_button.render(textures).await;
        minus_button.render(textures).await;

        if plus_button.is_released(MouseButton::Left).await && temp_settings.settings.gui_scale < 2.0 {
            temp_settings.settings.gui_scale += 0.1;
            temp_settings.settings.gui_scale = round(temp_settings.settings.gui_scale, 1).await
        }
        if minus_button.is_released(MouseButton::Left).await && temp_settings.settings.gui_scale > 0.2 {
            temp_settings.settings.gui_scale -= 0.1;
            temp_settings.settings.gui_scale = round(temp_settings.settings.gui_scale, 1).await
        }

        let text = format!("GUI Scale: {}", temp_settings.settings.gui_scale);
        let font_size = 64.0 * settings.gui_scale;
        let text_measures = measure_text(&text, None, font_size as _, 1.0);
        draw_text(
            &text,
            0.0,
            y + text_measures.offset_y,
            font_size as _,
            WHITE
        );
    }

    { // Apply Button
        let size = vec2(256.0, 128.0) * settings.gui_scale;
        let pos = vec2(screen_width(), screen_height()) - size;
        let button = Button::new(
            pos,
            size,
            vec2(32.0, 32.0) * settings.gui_scale,
            "Apply".to_string(),
            64.0 * settings.gui_scale,
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