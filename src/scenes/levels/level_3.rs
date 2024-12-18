use std::collections::BTreeMap;
use macroquad::camera::set_default_camera;
use macroquad::color::DARKBLUE;
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::math::vec2;
use macroquad::prelude::{clear_background, Texture2D};
use macroquad_platformer::World;
use crate::logic::level;
use crate::logic::level::{Level, LevelData, LevelSceneData, PersistentLevelData, Trigger};
use crate::utils::debugger;
use crate::utils::enums::{Scene, SceneTextureKey, TextureKey};
use crate::utils::mapper::{level_map_from_image, level_map_image_path};
use crate::utils::structs::Settings;
use crate::utils::texture::load_level_textures;

pub async fn level_3(scene: &mut Scene, textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, level_scene_data: &mut LevelSceneData, persistent_level_data: &mut PersistentLevelData, settings: &Settings) {
    clear_background(DARKBLUE);

    // Load textures if not loaded already
    if textures.get(&SceneTextureKey::Level3).is_none() {
        let keys = [
            TextureKey::Platform0,
            TextureKey::Player,
            TextureKey::Coin0,
            TextureKey::Icons0,
            TextureKey::Enemy0,
            TextureKey::PowerUps0,
            TextureKey::Cannon0,
            TextureKey::Projectile0
        ].to_vec();
        textures.insert(SceneTextureKey::Level3, load_level_textures("Level 3", keys).await);
    }

    // Load scene data for right level
    if level_scene_data.level_data.level != Some(Level::Level3) {
        *level_scene_data = layout(settings).await;
    }

    if is_key_pressed(KeyCode::Escape) {
        *scene = Scene::LevelSelector(0);
        level_scene_data.level_data.save(persistent_level_data, settings).await;
        *level_scene_data = LevelSceneData::empty().await;
        textures.remove(&SceneTextureKey::Level3);
        set_default_camera();
        return;
    }

    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::R) {
        level_scene_data.level_data.save(persistent_level_data, settings).await;
        *level_scene_data = layout(settings).await;
    }

    let textures = textures.get(&SceneTextureKey::Level3).unwrap();
    let width = 128.0 * settings.gui_scale;
    let height = 128.0 * settings.gui_scale;
    let size = vec2(width, height);

    let mut level_data = level_scene_data.level_data.clone(); // Temporary level data
    let mut world = &mut level_scene_data.world;
    let mut player = level_data.player.clone().unwrap();

    player.control(&mut world, &mut level_data, settings).await;

    level_data.player = Some(player);
    level_scene_data.level_data = level_data;

    if level_scene_data.level_data.player.as_ref().unwrap().pos.x > -0.0 * width { level_scene_data.level_data.triggers.insert(Trigger::LevelCompleted, true); }
    let won = *level_scene_data.level_data.triggers.get(&Trigger::LevelCompleted).unwrap_or(&false);

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
}

async fn layout(settings: &Settings) -> LevelSceneData {
    let mut world = World::new();
    let width = 128.0 * settings.gui_scale;
    let height = 128.0 * settings.gui_scale;
    let size = vec2(width, height);

    let (player, platforms, collectibles, enemies, cannons, power_ups) = level_map_from_image(
        level_map_image_path(Level::Level3).await,
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
            Level::Level3,
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