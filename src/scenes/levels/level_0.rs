use crate::logic::level;
use std::collections::BTreeMap;
use crate::Settings;
use macroquad::prelude::*;
use macroquad_platformer::World;
use stopwatch2::Stopwatch;
use crate::logic::cannon::Cannon;
use crate::logic::collectible::{Collectible, CollectibleType};
use crate::logic::collider::Collider;
use crate::logic::enemy::Enemy;
use crate::logic::level::{Level, LevelData, LevelSceneData, PersistentLevelData, Trigger};
use crate::logic::platform::{Platform, PlatformTile};
use crate::logic::player::{Player, PlayerPowerUp, PowerUp};
use crate::utils::debugger;
use crate::utils::enums::{Animation, AnimationType, Direction, Scene, SceneTextureKey, TextureKey};
use crate::utils::texture::{get_texture_path, load_textures_from_tile_map};

pub async fn level_0(scene: &mut Scene, mut textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, level_scene_data: &mut LevelSceneData, persistent_level_data: &mut PersistentLevelData, settings: &Settings) {
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
        level_scene_data.level_data.save(persistent_level_data, settings).await;
        *level_scene_data = LevelSceneData::empty().await;
        set_default_camera();
        return;
    }

    let game_over = level_scene_data.level_data.triggers.get(&Trigger::GameOver).unwrap_or(&false).to_owned();

    if !game_over { level::tick_level(level_scene_data, settings).await; }
    level::render_level(level_scene_data, &textures, settings).await;

    if !game_over {
        debugger::check(&mut level_scene_data.level_data.triggers, &mut level_scene_data.level_data.trigger_locks).await;
        debugger::render(level_scene_data, settings).await;
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
    let mut cannons = Vec::new();
    let mut power_ups = Vec::new();

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

    power_ups.push(PowerUp::new(
        PlayerPowerUp::SpeedBoost,
        120.0,
        vec2(size.x * 7.0, screen_height() - (size.y * 5.0)),
        size,
        TextureKey::PowerUps0,
        (18, 40),
        0.1
    ).await);

    { // Floating Platform
        let pos = vec2(size.x * 12.0, screen_height() - (size.y * 5.0 + size.y / 4.0));

        platforms.push(Platform::floating(
            3,
            size,
            TextureKey::Platform0,
            pos,
            &mut world,
        ).await);

        { // Enemy on Platform
            let size = vec2(size.x, size.y);
            let pos = vec2(size.x * 13.5, screen_height() - size.y * 7.0);
            let height = size.y;
            enemies.push(Enemy::new(
                pos,
                -50,
                &mut world,
                vec2(height, height),
                TextureKey::Player, // Player for now
            ).await);
        }
        { // Coin above Floating Platform
            let size = vec2(size.x, size.y);
            collectibles.push(Collectible::new(
                CollectibleType::Coin,
                vec2(size.x * 13.5, screen_height() - size.y * 7.0),
                size,
                TextureKey::Coin0,
                Animation::new(AnimationType::Cycle(0, 5, 0.1)),
                nv2,
            ).await)
        }
        // Left cannon below the platform
        cannons.push(Cannon::new(
            pos + size,
            size,
            2.0,
            0.0,
            Direction::Down,
            1500.0 * settings.gui_scale,
            4.0,
            TextureKey::Cannon0,
            TextureKey::Coin0,
            -100,
            &mut world

        ).await);

        // Right cannon below the platform
        cannons.push(Cannon::new(
            pos + vec2(width * 2.0, height),
            size,
            2.0,
            0.1,
            Direction::Down,
            1500.0 * settings.gui_scale,
            4.0,
            TextureKey::Cannon0,
            TextureKey::Coin0,
            -100,
            &mut world

        ).await);

        // Powerup in middle of cannons
        power_ups.push(PowerUp::new(
            PlayerPowerUp::Coins2x,
            20.0,
            pos + vec2(width * 1.5, height * 2.5),
            size,
            TextureKey::PowerUps0,
            (41, 63),
            0.1
        ).await);

        // Powerup in the Air above Platform
        power_ups.push(PowerUp::new(
            PlayerPowerUp::Damage2x,
            20.0,
            pos + vec2(width * -8.0, height * -8.0),
            size,
            TextureKey::PowerUps0,
            (64, 83),
            0.1
        ).await)
    }

    platforms.push(Platform::floating(
        3,
        size,
        TextureKey::Platform0,
        vec2(size.x * 18.0, screen_height() - (size.y * 8.0)),
        &mut world
    ).await);

    power_ups.push(PowerUp::new(
        PlayerPowerUp::JumpBoost,
        120.0,
        vec2(size.x * 18.0, screen_height() - (size.y * 10.0)),
        size,
        TextureKey::PowerUps0,
        (0, 17),
        0.1
    ).await);

    let pos = vec2(300.0 * settings.gui_scale, 0.0);
    LevelSceneData {
        level_data: LevelData {
            start_time: get_time(),

            level: Some(Level::Level0),
            player: Some(Player::new(size.x, size.y, vec2(pos.x, nv2.y), 0, &mut world).await),
            platforms,
            collectibles,
            enemies,
            cannons,
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
    textures.insert(SceneTextureKey::Level0, result);

    stopwatch.stop();
    println!("Loaded textures! Took: {}ms", stopwatch.elapsed().as_millis());
}