use std::collections::BTreeMap;
use macroquad::camera::set_default_camera;
use macroquad::color::{BLACK, DARKBLUE, WHITE};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::math::vec2;
use macroquad::texture::{load_texture, FilterMode, Texture2D};
use macroquad::window::{clear_background, screen_height};
use macroquad_platformer::World;
use stopwatch2::Stopwatch;
use crate::logic::level;
use crate::logic::level::{Level, LevelData, LevelSceneData, PersistentLevelData, Trigger};
use crate::logic::platform::Platform;
use crate::logic::player::Player;
use crate::utils::debugger;
use crate::utils::enums::{Scene, SceneTextureKey, TextureKey};
use crate::utils::structs::Settings;
use crate::utils::text::{draw_text_center, draw_text_centered};
use crate::utils::texture::{get_texture_path, load_textures_from_tile_map};

pub async fn level_1(scene: &mut Scene, mut textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, level_scene_data: &mut LevelSceneData, persistent_level_data: &mut PersistentLevelData, settings: &Settings) {
    clear_background(DARKBLUE);

    // Load textures if not loaded already
    if textures.get(&SceneTextureKey::Level1).is_none() {
        load_textures(&mut textures).await;
    }

    // Load scene data for right level
    if level_scene_data.level_data.level != Some(Level::Level1) {
        *level_scene_data = layout(settings).await;
    }

    if is_key_down(KeyCode::Escape) {
        *scene = Scene::LevelSelector(0);
        level_scene_data.level_data.save(persistent_level_data, settings).await;
        *level_scene_data = LevelSceneData::empty().await;
        textures.remove(&SceneTextureKey::Level1);
        set_default_camera();
        return;
    }

    let textures = textures.get(&SceneTextureKey::Level1).unwrap();

    let mut level_data = level_scene_data.level_data.clone(); // Temporary level data
    let mut world = &mut level_scene_data.world;
    let mut player = level_data.player.clone().unwrap();

    player.control(&mut world, &mut level_data, settings).await;

    level_data.player = Some(player);
    level_scene_data.level_data = level_data;

    let won = level_scene_data.level_data.player.as_ref().unwrap().coins >= 2;
    let game_over = level_scene_data.level_data.triggers.get(&Trigger::GameOver).unwrap_or(&false).to_owned();

    if !game_over && !won { level::tick_level(level_scene_data, settings).await; }
    level::render_level(level_scene_data, &textures, settings).await;

    if !game_over && !won {
        debugger::check(&mut level_scene_data.level_data.triggers, &mut level_scene_data.level_data.trigger_locks).await;
        debugger::render(level_scene_data, settings).await;
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
    let enemies = Vec::new();
    let cannons = Vec::new();
    let power_ups = Vec::new();

    platforms.push(Platform::full(
        5,
        2,
        size,
        TextureKey::Platform0,
        vec2(0.0, height * 4.0),
        &mut world
    ).await);

    let pos = vec2(0.0, 0.0);
    LevelSceneData::new(
        LevelData::new(
            Level::Level1,
            Player::new(size.x, size.y, vec2(pos.x, nv2.y), 0, &mut world).await,
            platforms,
            collectibles,
            enemies,
            cannons,
            power_ups
        ).await,
        world
    ).await
}
async fn load_textures(textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>) {
    let mut stopwatch = Stopwatch::default();
    println!("Loading textures for Level 1...");
    stopwatch.start();

    let mut result = BTreeMap::new();
    // Load player textures
    let player = {
        let mut result = Vec::new();

        let player_walk_left = load_texture("res/textures/player/player_walk_left.png").await.unwrap();
        // FilterMode is set to Nearest so it doesn't pixelate when scaling
        player_walk_left.set_filter(FilterMode::Nearest);

        let player_walk_right = load_texture("res/textures/player/player_walk_right.png").await.unwrap();
        player_walk_right.set_filter(FilterMode::Nearest);

        result.push(player_walk_left);
        result.push(player_walk_right);

        result
    };
    result.insert(TextureKey::Player, player);


    let platform_0 = {
        let path = get_texture_path(TextureKey::Platform0).await;
        load_textures_from_tile_map(path).await
    };
    result.insert(TextureKey::Platform0, platform_0);

    let coin_0 = {
        let path = get_texture_path(TextureKey::Coin0).await;
        load_textures_from_tile_map(path).await
    };
    result.insert(TextureKey::Coin0, coin_0);

    let power_ups_0 = {
        let path = get_texture_path(TextureKey::PowerUps0).await;
        load_textures_from_tile_map(path).await
    };
    result.insert(TextureKey::PowerUps0, power_ups_0);

    let icons_0 =  {
        let path = get_texture_path(TextureKey::Icons0).await;
        load_textures_from_tile_map(path).await
    };
    result.insert(TextureKey::Icons0, icons_0);

    let cannons_0 = {
        let path = get_texture_path(TextureKey::Cannon0).await;
        load_textures_from_tile_map(path).await
    };
    result.insert(TextureKey::Cannon0, cannons_0);

    // Insert result into the global texture map
    textures.insert(SceneTextureKey::Level1, result);

    stopwatch.stop();
    println!("Loaded textures for Level 1! Took: {}ms", stopwatch.elapsed().as_millis());
}