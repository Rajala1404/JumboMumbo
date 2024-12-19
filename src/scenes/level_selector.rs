use std::collections::BTreeMap;
use macroquad::color::WHITE;
use macroquad::input::MouseButton;
use macroquad::math::vec2;
use macroquad::prelude::{is_key_pressed, screen_height, screen_width, KeyCode, Texture2D};
use macroquad::text::measure_text;
use crate::logic::level::{Level, PersistentLevelData};
use crate::utils::enums::{Scene, SceneTextureKey, TextureKey};
use crate::ui::buttons::Button;
use crate::utils::structs::Settings;
use crate::utils::text::draw_text_centered;
use crate::utils::texture::load_textures;

pub async fn level_selector(scene: &mut Scene, textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, settings: &Settings, persistent_level_data: &PersistentLevelData) {
    if !textures.contains_key(&SceneTextureKey::LevelSelector) {
        textures.insert(SceneTextureKey::LevelSelector, load_textures("Level Selector", [TextureKey::Button0].to_vec()).await);
    }

    if is_key_pressed(KeyCode::Escape) {
        *scene = Scene::MainMenu;
        textures.remove(&SceneTextureKey::LevelSelector);
        return;
    }
    let scene_textures = textures.get(&SceneTextureKey::LevelSelector).unwrap();

    { // Draw page switch buttons
        match scene {
            Scene::LevelSelector(page) => {
                let min_page = 0;
                let max_page = 3;

                let size = vec2(128.0, 129.0) * settings.gui_scale;
                let border_size = vec2(32.0, 32.0) * settings.gui_scale;

                let left_button = Button::new(
                    vec2(0.0, screen_height() / 2.0 - size.y / 2.0),
                    size,
                    border_size,
                    "<".to_string(),
                    64.0 * settings.gui_scale,
                    TextureKey::Button0,
                ).await;

                let right_button = Button::new(
                    vec2(screen_width() - size.x, screen_height() / 2.0 - size.y / 2.0),
                    size,
                    border_size,
                    ">".to_string(),
                    64.0 * settings.gui_scale,
                    TextureKey::Button0,
                ).await;

                if page.to_owned() != min_page { left_button.render(scene_textures).await; }
                if page.to_owned() != max_page { right_button.render(scene_textures).await; }

                if (is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) || left_button.is_released(MouseButton::Left).await) && page.to_owned() != min_page {
                    *page -= 1;
                }
                if (is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) || right_button.is_released(MouseButton::Left).await) && page.to_owned() !=  max_page {
                    *page += 1;
                }

            },
            _ => unreachable!()
        }
    }

    let mut new_scene = scene.clone();

    new_scene = match scene {
        Scene::LevelSelector(page) => {
            match *page {
                0 => level_page(Level::Level0, TextureKey::Button0, scene_textures, settings, persistent_level_data).await,
                1 => level_page(Level::Level1, TextureKey::Button0, scene_textures, settings, persistent_level_data).await,
                2 => level_page(Level::Level2, TextureKey::Button0, scene_textures, settings, persistent_level_data).await,
                3 => level_page(Level::Level3, TextureKey::Button0, scene_textures, settings, persistent_level_data).await,
                _ => unimplemented!()
            }
        },
        _ => unreachable!()
    };

    match new_scene {
        Scene::LevelSelector(_) => {},
        _ => {
            *scene = new_scene;
            textures.remove(&SceneTextureKey::LevelSelector);
            return;
        }
    }
}

async fn level_page(level: Level, button_texture_key: TextureKey, textures: &BTreeMap<TextureKey, Vec<Texture2D>>, settings: &Settings, persistent_level_data: &PersistentLevelData) -> Scene {
    let mut scene = Scene::LevelSelector(1);

    draw_text_centered(
        level.name(),
        screen_height() / 8.0,
        128.0 * settings.gui_scale,
        WHITE
    ).await;

    { // Stats
        let (plays_text, total_deaths_text, high_coins_text, high_kills_text) = match persistent_level_data.stats.get(&level) {
            Some(stats) => {
                let plays_text = format!("Plays: {}", stats.plays);
                let total_deaths_text = format!("Deaths: {}", stats.deaths);
                let high_coins_text = format!("Max. collected Coins: {}", stats.coins_high);
                let high_kills_text = format!("Max. killed Enemies: {}", stats.kills_high);

                (plays_text, total_deaths_text, high_coins_text, high_kills_text)
            }
            None => {
                let plays_text = "Plays: None".to_string();
                let total_deaths_text = "Deaths: None".to_string();
                let high_coins_text = "Max. collected Coins: None".to_string();
                let high_kills_text = "Max. killed Enemies: None".to_string();

                (plays_text, total_deaths_text, high_coins_text, high_kills_text)
            }
        };

        let font_size = (64.0 * settings.gui_scale) as _;
        let plays_text_mes = measure_text(&plays_text, None, font_size, 1.0);
        let total_deaths_text_mes = measure_text(&total_deaths_text, None, font_size, 1.0);
        let high_coins_text_mes = measure_text(&high_coins_text, None, font_size, 1.0);
        let high_kills_text_mes = measure_text(&high_kills_text, None, font_size, 1.0);

        let mut y = {
            let total_height = plays_text_mes.height + total_deaths_text_mes.height + high_coins_text_mes.height + high_kills_text_mes.height + screen_height() / 64.0 * 3.0;
            screen_height() / 2.0 - total_height / 2.0
        };

        draw_text_centered(&plays_text, y + plays_text_mes.offset_y, font_size as f32, WHITE).await;
        y += plays_text_mes.height + screen_height() / 64.0;
        draw_text_centered(&total_deaths_text, y + total_deaths_text_mes.offset_y, font_size as f32, WHITE).await;
        y += total_deaths_text_mes.height + screen_height() / 64.0;
        draw_text_centered(&high_coins_text, y + high_coins_text_mes.offset_y, font_size as f32, WHITE).await;
        y += high_coins_text_mes.height + screen_height() / 64.0;
        draw_text_centered(&high_kills_text, y + high_kills_text_mes.offset_y, font_size as f32, WHITE).await;
    }

    let size = vec2(400.0, 150.0) * settings.gui_scale;
    let border_size = vec2(64.0, 64.0) * settings.gui_scale;
    let button_pos = vec2(screen_width() / 2.0 - size.x / 2.0, screen_height() - screen_height() / 8.0 - size.y);

    let button = Button::new(
        button_pos,
        size,
        border_size,
        "Play".to_string(),
        64.0 * settings.gui_scale,
        button_texture_key,
    ).await;

    button.render(textures).await;

    if button.is_released(MouseButton::Left).await || is_key_pressed(KeyCode::Space) {
        scene = Scene::Level(level);
    }

    scene
}