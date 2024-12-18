use std::collections::BTreeMap;
use macroquad::camera::set_default_camera;
use macroquad::color::{BLACK, DARKBLUE, WHITE};
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::math::vec2;
use macroquad::text::draw_text;
use macroquad::texture::Texture2D;
use macroquad::window::{clear_background, screen_height};
use macroquad_platformer::World;
use crate::logic::collectible::{Collectible, CollectibleType};
use crate::logic::enemy::Enemy;
use crate::logic::level;
use crate::logic::level::{Level, LevelData, LevelSceneData, PersistentLevelData, Trigger};
use crate::logic::platform::Platform;
use crate::logic::player::{Player, PlayerPowerUp, PowerUp};
use crate::utils::debugger;
use crate::utils::enums::{Animation, AnimationType, Scene, SceneTextureKey, TextureKey};
use crate::utils::structs::Settings;
use crate::utils::text::{draw_text_center, draw_text_centered};
use crate::utils::texture::load_level_textures;

pub async fn level_1(scene: &mut Scene, textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, level_scene_data: &mut LevelSceneData, persistent_level_data: &mut PersistentLevelData, settings: &Settings) {
    clear_background(DARKBLUE);

    // Load textures if not loaded already
    if textures.get(&SceneTextureKey::Level1).is_none() {
        let keys = [
            TextureKey::Platform0,
            TextureKey::PowerUps0,
            TextureKey::Player,
            TextureKey::Icons0,
            TextureKey::Coin0,
            TextureKey::Enemy0,
            TextureKey::Projectile0
        ].to_vec();
        textures.insert(SceneTextureKey::Level1, load_level_textures("Level 1", keys).await);
    }

    // Load scene data for right level
    if level_scene_data.level_data.level != Some(Level::Level1) {
        *level_scene_data = layout(settings).await;
    }

    if is_key_pressed(KeyCode::Escape) {
        *scene = Scene::LevelSelector(0);
        level_scene_data.level_data.save(persistent_level_data, settings).await;
        *level_scene_data = LevelSceneData::empty().await;
        textures.remove(&SceneTextureKey::Level1);
        set_default_camera();
        return;
    }

    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::R) {
        level_scene_data.level_data.save(persistent_level_data, settings).await;
        *level_scene_data = layout(settings).await;
    }

    let textures = textures.get(&SceneTextureKey::Level1).unwrap();

    let mut level_data = level_scene_data.level_data.clone(); // Temporary level data
    let mut world = &mut level_scene_data.world;
    let mut player = level_data.player.clone().unwrap();

    player.control(&mut world, &mut level_data, settings).await;

    level_data.player = Some(player);
    level_scene_data.level_data = level_data;

    let won = level_scene_data.level_data.enemies.is_empty();
    let game_over = level_scene_data.level_data.triggers.get(&Trigger::GameOver).unwrap_or(&false).to_owned();

    if !game_over && !won { level::tick_level(level_scene_data, settings).await; }
    level::render_level(level_scene_data, &textures, settings).await;

    if !game_over && !won {
        debugger::check(&mut level_scene_data.level_data.triggers, &mut level_scene_data.level_data.trigger_locks).await;
        debugger::render(level_scene_data, settings).await;
    }

    { // Tutorial shoot text
        let pos = vec2(354.0 * (128.0 * settings.gui_scale), -7.0 * (128.0 * settings.gui_scale));
        draw_text("Shoot with Q and E or left click", pos.x, pos.y, 48.0 * settings.gui_scale, WHITE);
    }

    if level_scene_data.level_data.player.as_ref().unwrap().pos.y > 0.0 * (128.0 * settings.gui_scale) {
        level_scene_data.level_data.player.as_mut().unwrap().health = 0;
    }

    if won {
        set_default_camera();
        clear_background(BLACK);
        draw_text_center("Congratulations!", 300.0 * settings.gui_scale, WHITE).await;
        draw_text_centered("You completed Level 1! Press ESC to go back", screen_height() / 2.0 + 250.0 * settings.gui_scale, 100.0 * settings.gui_scale, WHITE).await;
    }
}

async fn layout(settings: &Settings) -> LevelSceneData {
    let mut world = World::new();
    let width = 128.0 * settings.gui_scale;
    let height = 128.0 * settings.gui_scale;
    let size = vec2(width, height);
    let nv2 = vec2(0.0, 0.0);

    let mut platforms = Vec::new();
    let mut collectibles = Vec::new();
    let mut enemies = Vec::new();
    let cannons = Vec::new();
    let mut power_ups = Vec::new();

    platforms.push(Platform::floating(
        3,
        size,
        TextureKey::Platform0,
        vec2(width * -1.5, 0.0),
        &mut world
    ).await);

    platforms.push(Platform::floating(
        3,
        size,
        TextureKey::Platform0,
        vec2(size.x * 5.0, size.y * -2.5),
        &mut world
    ).await);

    power_ups.push(
        PowerUp::new(
            PlayerPowerUp::Coins2x,
            120.0,
            vec2(size.x *  6.5, size.y * -4.5),
            size,
            TextureKey::PowerUps0,
            (41, 63),
            0.1
        ).await
    );

    platforms.push(Platform::floating(
        2,
        size,
        TextureKey::Platform0,
        vec2(size.x * 12.0, size.y * -4.5),
        &mut world
    ).await);

    collectibles.push(Collectible::new(
        CollectibleType::Coin,
        vec2(size.x * 13.0, size.y * -6.5),
        size,
        TextureKey::Coin0,
        Animation::new(AnimationType::Cycle(0, 5, 0.1)),
        nv2
    ).await);

    platforms.push(Platform::floating(
        2,
        size,
        TextureKey::Platform0,
        vec2(size.x * 21.0, size.y * -2.5),
        &mut world
    ).await);

    collectibles.push(Collectible::new(
        CollectibleType::Coin,
        vec2(size.x * 22.0, size.y * -4.5),
        size,
        TextureKey::Coin0,
        Animation::new(AnimationType::Cycle(0, 5, 0.1)),
        nv2
    ).await);

    platforms.push(Platform::floating(
        4,
        size,
        TextureKey::Platform0,
        vec2(size.x * 28.0, size.y * -4.0),
        &mut world
    ).await);

    collectibles.push(Collectible::new(
        CollectibleType::Coin,
        vec2(size.x * 28.0, size.y * -6.0),
        size,
        TextureKey::Coin0,
        Animation::new(AnimationType::Cycle(0, 5, 0.1)),
        nv2
    ).await);

    power_ups.push(PowerUp::new(
        PlayerPowerUp::SpeedBoost,
        30.0,
        vec2(size.x * 30.0, size.y * -6.0),
        size,
        TextureKey::PowerUps0,
        (18, 40),
        0.1
    ).await);

    power_ups.push(PowerUp::new(
        PlayerPowerUp::JumpBoost,
        30.0,
        vec2(size.x * 32.0, size.y * -6.0),
        size,
        TextureKey::PowerUps0,
        (0, 17),
        0.1
    ).await);

    for i in  (0..=306).step_by(18) {
        let pos = vec2(size.x * (i + 40) as f32, size.y * -6.0);
        platforms.push(Platform::floating(
            2,
            size,
            TextureKey::Platform0,
            pos,
            &mut world
        ).await);
        collectibles.push(Collectible::new(
            CollectibleType::Coin,
            pos + vec2(size.x * 1.0, size.y * -2.0),
            size,
            TextureKey::Coin0,
            Animation::new(AnimationType::Cycle(0, 5, 0.1)),
            nv2
        ).await)
    }

    platforms.push(Platform::floating(
        4,
        size,
        TextureKey::Platform0,
        vec2(size.x * 354.0, size.y * -4.5),
        &mut world
    ).await);

    power_ups.push(PowerUp::new(
        PlayerPowerUp::Damage2x,
        60.0,
        vec2(size.x * 356.0, size.y * -6.5),
        size,
        TextureKey::PowerUps0,
        (64, 83),
        0.1
    ).await);

    platforms.push(Platform::floating(
        8,
        size,
        TextureKey::Platform0,
        vec2(size.x * 364.0, size.y * -6.0),
        &mut world
    ).await);

    enemies.push(Enemy::new(
        vec2(size.x * 364.0, size.y * -7.5),
        -50,
        &mut world,
        size,
        TextureKey::Enemy0
    ).await);

    let pos = vec2(0.0, height * -10.0);
    LevelSceneData::new(
        LevelData::new(
            Level::Level1,
            Player::new(size.x, size.y, pos, 0, &mut world).await,
            platforms,
            collectibles,
            enemies,
            cannons,
            power_ups
        ).await,
        world
    ).await
}