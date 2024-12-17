use std::collections::BTreeMap;
use macroquad::camera::set_default_camera;
use macroquad::color::{BLACK, DARKBLUE, WHITE};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::math::vec2;
use macroquad::prelude::{clear_background, screen_height, Texture2D};
use macroquad_platformer::World;
use crate::logic::level;
use crate::logic::level::{Level, LevelData, LevelSceneData, PersistentLevelData, Trigger};
use crate::utils::debugger;
use crate::utils::enums::{Scene, SceneTextureKey, TextureKey};
use crate::utils::mapper::{level_map_from_image, level_map_image_path};
use crate::utils::structs::Settings;
use crate::utils::text::{draw_text_center, draw_text_centered};
use crate::utils::texture::load_level_textures;

pub async fn level_2(scene: &mut Scene, textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, level_scene_data: &mut LevelSceneData, persistent_level_data: &mut PersistentLevelData, settings: &Settings) {
    clear_background(DARKBLUE);

    // Load textures if not loaded already
    if textures.get(&SceneTextureKey::Level2).is_none() {
        let keys = [
            TextureKey::Platform0,
            TextureKey::Player,
            TextureKey::Projectile0,
            TextureKey::Cannon0,
            TextureKey::Icons0,
            TextureKey::Coin0
        ].to_vec();
        textures.insert(SceneTextureKey::Level2, load_level_textures("Level 2", keys).await);
    }

    // Load scene data for right level
    if level_scene_data.level_data.level != Some(Level::Level2) {
        *level_scene_data = layout(settings).await;
    }

    if is_key_down(KeyCode::Escape) {
        *scene = Scene::LevelSelector(0);
        level_scene_data.level_data.save(persistent_level_data, settings).await;
        *level_scene_data = LevelSceneData::empty().await;
        textures.remove(&SceneTextureKey::Level2);
        set_default_camera();
        return;
    }

    let textures = textures.get(&SceneTextureKey::Level2).unwrap();

    let mut level_data = level_scene_data.level_data.clone(); // Temporary level data
    let mut world = &mut level_scene_data.world;
    let mut player = level_data.player.clone().unwrap();

    player.control(&mut world, &mut level_data, settings).await;

    level_data.player = Some(player);
    level_scene_data.level_data = level_data;

    let won = !level_scene_data.level_data.enemies.is_empty();
    let game_over = level_scene_data.level_data.triggers.get(&Trigger::GameOver).unwrap_or(&false).to_owned();

    if !game_over && !won { level::tick_level(level_scene_data, settings).await; }
    level::render_level(level_scene_data, &textures, settings).await;

    if !game_over && !won {
        debugger::check(&mut level_scene_data.level_data.triggers, &mut level_scene_data.level_data.trigger_locks).await;
        debugger::render(level_scene_data, settings).await;
    }

    if level_scene_data.level_data.player.as_ref().unwrap().pos.y > 2.0 * (128.0 * settings.gui_scale) {
        level_scene_data.level_data.player.as_mut().unwrap().health = 0;
    }

    if won {
        set_default_camera();
        clear_background(BLACK);
        draw_text_center("Congratulations!", 300.0 * settings.gui_scale, WHITE).await;
        draw_text_centered("You completed Level 2! Press ESC to go back", screen_height() / 2.0 + 250.0 * settings.gui_scale, 100.0 * settings.gui_scale, WHITE).await;
    }
}

async fn layout(settings: &Settings) -> LevelSceneData {
    let mut world = World::new();
    let width = 128.0 * settings.gui_scale;
    let height = 128.0 * settings.gui_scale;
    let size = vec2(width, height);

    let (player, platforms, collectibles, enemies, cannons, power_ups) = level_map_from_image(
        level_map_image_path(Level::Level2).await,
        size,
        settings,
        &mut world,
        TextureKey::Platform0,
        TextureKey::Coin0,
        TextureKey::Enemy0,
        TextureKey::Cannon0,
        TextureKey::Projectile0,
        TextureKey::PowerUps0
    ).await;

    LevelSceneData::new(
        LevelData::new(
            Level::Level2,
            player,
            platforms,
            collectibles,
            enemies,
            cannons,
            power_ups
        ).await,
        world
    ).await
}