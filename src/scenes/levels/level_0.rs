use crate::logic::level;
use std::collections::BTreeMap;
use crate::Settings;
use macroquad::prelude::*;
use macroquad_platformer::World;
use stopwatch2::Stopwatch;
use crate::logic::collider::Collider;
use crate::logic::enemy::Enemy;
use crate::logic::player::{Player, PlayerPowerUp, PowerUp};
use crate::scenes::levels::structs::{Collectible, Level, LevelData, LevelSceneData, Platform, PlatformTile};
use crate::utils::debugger;
use crate::utils::enums::{Animation, AnimationType, Scene, SceneTextureKey, TextureKey};
use crate::utils::texture::{get_texture_path, load_textures_from_tile_map};

pub async fn level_0(scene: &mut Scene, mut textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, level_scene_data: &mut LevelSceneData, settings: &Settings) {
    clear_background(DARKBLUE);

    // Load textures if not loaded already
    if textures.get(&SceneTextureKey::Level0).is_none() {
        load_textures(&mut textures).await;
    }
    let textures = textures.get(&SceneTextureKey::Level0).unwrap();

    // Load scene data for right level
    if level_scene_data.level_data.level != Some(Level::Level0) {
        *level_scene_data = layout(settings).await;
    }

    let mut level_data = level_scene_data.level_data.clone(); // Temporary level data
    let mut world = &mut level_scene_data.world;
    let mut player = level_data.player.clone().unwrap();

    player.control(&mut world, &mut level_data, settings).await;

    level_data.player = Some(player);
    level_scene_data.level_data = level_data;


    if is_key_down(KeyCode::Escape) {
        *scene = Scene::LevelSelector(0);
        *level_scene_data = LevelSceneData::empty().await;
        set_default_camera();
        return;
    }

    level::tick_level(level_scene_data, settings).await;
    level::render_level(level_scene_data, &textures, settings).await;

    debugger::check(&mut level_scene_data.level_data.triggers, &mut level_scene_data.level_data.trigger_locks).await;
    debugger::render(level_scene_data, settings).await;

    if level_scene_data.level_data.projectiles.iter().count() > 3000 { panic!() }
}

async fn layout(settings: &Settings) -> LevelSceneData {
    let mut world = World::new();
    let width = 128.0 * settings.gui_scale;
    let height = 128.0 * settings.gui_scale;
    let size = vec2(width, height);
    let nv2 = vec2(0.0, 0.0);

    let mut platforms = vec![];
    let mut collectibles = vec![];
    let mut enemies = vec![];

    { // Base Platform 1
        let pos = vec2(size.x * -20.0, screen_height() - size.y);

        let mut tiles = vec![
            PlatformTile {
                texture_key: TextureKey::Platform0,
                texture_index: 0,
                pos: vec2(0.0, 0.0),
            },
            PlatformTile {
                texture_key: TextureKey::Platform0,
                texture_index: 3,
                pos: vec2(0.0, 1.0),
            }
        ];

        for i in 1..40 {
            tiles.push(PlatformTile{
                texture_key: TextureKey::Platform0,
                texture_index: 1,
                pos: vec2(i as f32, 0.0),
            });
            tiles.push(PlatformTile{
                texture_key: TextureKey::Platform0,
                texture_index: 4,
                pos: vec2(i as f32, 1.0),
            })
        }

        tiles.push(PlatformTile{
            texture_key: TextureKey::Platform0,
            texture_index: 2,
            pos: vec2(40.0, 0.0),
        });
        tiles.push(PlatformTile {
            texture_key: TextureKey::Platform0,
            texture_index: 5,
            pos: vec2(40.0, 1.0),
        });

        platforms.push(Platform{
            collider: world.add_solid(pos, (width * 41.0) as i32, height as i32),
            collider_new: Collider::new_solid(pos,width * 41.0, height, vec2(0.0, 0.0)).await,
            tile_size: size,
            tiles,
            speed: nv2
        });
    }

    platforms.push(Platform::floating(
        4,
        size,
        TextureKey::Platform0,
        vec2(size.x * 5.0, screen_height() - (size.y * 3.0 + size.y / 4.0)),
        &mut world
    ).await);

    platforms.push(Platform::floating(
        3,
        size,
        TextureKey::Platform0,
        vec2(size.x * 12.0, screen_height() - (size.y * 5.0 + size.y / 4.0)),
        &mut world
    ).await);

    platforms.push(Platform::floating(
        3,
        size,
        TextureKey::Platform0,
        vec2(size.x * 18.0, screen_height() - (size.y * 8.0)),
        &mut world
    ).await);

    { // Coin above Floating Platform
        let size = vec2(size.x, size.y);
        collectibles.push(Collectible {
            collected: false,
            collider: Collider::new_collectible(vec2(size.x * 13.5, screen_height() - size.y * 7.0), size.x, size.y, nv2).await,
            texture_key: TextureKey::Coin0,
            animation: Animation::new(AnimationType::Cycle(0, 5, 0.1)),
            size,
            speed: nv2
        });
    }

    { // Enemy on Platform
        let size = vec2(size.x, size.y);
        let pos = vec2(size.x * 13.5, screen_height() - size.y * 7.0);
        let height = size.y;
        enemies.push(Enemy::new(
            pos,
            -50,
            &mut world,
            vec2(height, height),
            TextureKey::Player // Player for now
        ).await);
    }

    let mut power_ups = Vec::new();
    power_ups.push(PowerUp::new(
        PlayerPowerUp::JumpBoost,
        20.0,
        vec2(size.x * 18.0, screen_height() - (size.y * 10.0)),
        size,
        TextureKey::PowerUps0,
        (0, 17),
        0.1
    ).await);

    let pos = vec2(400.0 * settings.gui_scale, 0.0);
    LevelSceneData {
        level_data: LevelData {
            level: Some(Level::Level0),
            player: Some(Player::new(size.x, size.y, vec2(pos.x, nv2.y), 0, &mut world).await),
            platforms,
            collectibles,
            enemies,
            projectiles: Vec::new(),
            power_ups,
            triggers: BTreeMap::new(),
            trigger_locks: BTreeMap::new() },
        world
    }
}

async fn load_textures(textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>) {
    let mut stopwatch = Stopwatch::default();
    println!("Loading textures...");
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

    // Insert result into the global texture map
    textures.insert(SceneTextureKey::Level0, result);

    stopwatch.stop();
    println!("Loaded textures! Took: {}ms", stopwatch.elapsed().as_millis());
}