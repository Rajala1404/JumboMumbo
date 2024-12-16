use std::collections::BTreeMap;
use macroquad::color::Color;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{screen_height, screen_width, Texture2D};
use macroquad::ui::root_ui;
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

    draw_text_centered("JumboMumbo", screen_height() / 8.0, 150.0 * settings.gui_scale, Color::from_rgba(255, 255, 255, 255)).await;

    // Executes the code inside the brackets
    if root_ui().button(Some(Vec2 { x: screen_width() / 2.0, y: screen_height() / 2.0 }), "Select Level") {
        textures.remove(&SceneTextureKey::MainMenu);
        // Sets the Scene to LevelSelector on page 0
        *scene = Scene::LevelSelector(0);
        return;
    }

    let test_button = Button::new(
        vec2(0.0, 0.0),
        vec2(256.0, 128.0) * settings.gui_scale,
        vec2(64.0, 64.0) * settings.gui_scale,
        "Test".to_string(),
        64.0 * settings.gui_scale,
        TextureKey::Button0
    ).await;

    test_button.render(textures.get(&SceneTextureKey::MainMenu).unwrap()).await;

    if root_ui().button(Some(Vec2 { x: screen_width() / 2.0, y: screen_height() / 2.0 + screen_height() / 4.0 }), "Settings") {
        textures.remove(&SceneTextureKey::MainMenu);
        *scene = Scene::SettingsMenu;
        return;
    }
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