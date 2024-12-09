use crate::scenes::levels::helper;
use std::collections::BTreeMap;
use crate::Settings;
use macroquad::prelude::*;
use macroquad_platformer::World;
use stopwatch2::Stopwatch;
use crate::logic::collider::Collider;
use crate::logic::enemy::Enemy;
use crate::logic::player::Player;
use crate::scenes::levels::structs::{Collectible, Level, LevelSceneData, Platform, PlatformTile};
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
    if level_scene_data.level != Some(Level::Level0) {
        *level_scene_data = layout(settings).await;
    }

    let mut world = &mut level_scene_data.world;
    let player = level_scene_data.player.as_mut().unwrap();
    let enemies = &level_scene_data.enemies;

    player.control(&mut world, enemies, settings).await;

    if is_key_down(KeyCode::Escape) {
        *scene = Scene::LevelSelector(0);
        *level_scene_data = LevelSceneData::empty().await;
        set_default_camera();
        return;
    }

    helper::tick_level(level_scene_data, settings).await;
    helper::render_level(level_scene_data, &textures, settings).await;

    debugger::check(&mut level_scene_data.triggers, &mut level_scene_data.trigger_locks).await;
    debugger::render(level_scene_data, settings).await;
}

async fn layout(settings: &Settings) -> LevelSceneData {
    let mut world = World::new();
    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;
    let width = 128.0 * settings.gui_scale;
    let height = 128.0 * settings.gui_scale;
    let size = vec2(width, height);
    let pos = vec2(x, y);
    let nv2 = vec2(0.0, 0.0);

    let mut platforms = vec![];
    let mut collectibles = vec![];
    let mut enemies = vec![];

    { // Base Platform (Collider)
        let pos = vec2(0.0 - screen_width(), screen_height());

        platforms.push(
            Platform {
                collider: world.add_solid(pos, screen_width() as i32 * 3, (screen_height() / 32.0) as i32 ),
                tile_size: size,
                tiles: vec![],
                speed: nv2
            }
        );
    }
    { // Base Platform 1
        let pos = vec2(-screen_width() / 2.0, screen_height() - size.y);

        let mut tiles = vec![
            PlatformTile {
                texture_key: TextureKey::Platform0,
                texture_index: 0,
                pos: vec2(0.0, 0.0),
            },
        ];

        for i in 1..40 {
            tiles.push(PlatformTile{
                texture_key: TextureKey::Platform0,
                texture_index: 1,
                pos: vec2(i as f32, 0.0),
            })
        }

        tiles.push(PlatformTile{
            texture_key: TextureKey::Platform0,
            texture_index: 2,
            pos: vec2(40.0, 0.0),
        });

        platforms.push(Platform{
            collider: world.add_solid(pos, (width * 41.0) as i32, height as i32),
            tile_size: size,
            tiles,
            speed: nv2
        });
    }

    platforms.push(Platform::floating(
        4,
        size,
        TextureKey::Platform0,
        vec2(size.x * 5.0, screen_height() - size.y * 3.0 - size.y / 4.0),
        &mut world
    ).await);

    platforms.push(Platform::floating(
        3,
        size,
        TextureKey::Platform0,
        vec2(size.x * 12.0, screen_height() - size.y * 5.0 - size.y / 4.0),
        &mut world
    ).await);

    { // Coin above Floating Platform
        let size = vec2(size.x, size.y);
        collectibles.push(Collectible {
            collected: false,
            collider: Collider::new_collectible(vec2(size.x * 13.5, screen_height() - size.y * 7.0), size.x, size.y, nv2).await,
            texture_key: TextureKey::Coin0,
            animation: Animation::new(AnimationType::Cycle(0, 5)),
            size,
            speed: nv2
        });
    }

    { // Enemy on Platform
        let size = vec2(size.x, size.y);
        let pos = vec2(size.x * 13.5, screen_height() - size.y * 7.0);
        let width = size.x * 2.0;
        let height = size.y;
        enemies.push(Enemy::new(
            pos,
            &mut world,
            vec2(height, height),
            TextureKey::Player // Player for now
        ).await);
    }

    LevelSceneData {
        level: Some(Level::Level0),
        player: Some(Player::new(width, height, vec2(pos.x, nv2.y), 0, &mut world).await),
        platforms,
        collectibles,
        enemies,
        world,
        triggers: BTreeMap::new(),
        trigger_locks: BTreeMap::new()
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

    // Insert result into the global texture map
    textures.insert(SceneTextureKey::Level0, result);

    stopwatch.stop();
    println!("Loaded textures! Took: {}ms", stopwatch.elapsed().as_millis());
}