use crate::utils::render;
use std::collections::BTreeMap;
use crate::Settings;
use macroquad::prelude::*;
use macroquad_platformer::World;
use crate::logic::player::Player;
use crate::scenes::levels::structs::{Level, LevelSceneData, Platform, PlatformTile, Triggers};
use crate::utils::debugger::draw_camera_collider;
use crate::utils::enums::{Scene, SceneTextureKey, TextureKey};
use crate::utils::texture::{get_platform_path, load_textures_from_tile_map};

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

    let mut world = level_scene_data.world.as_mut().unwrap();
    let player = level_scene_data.player.as_mut().unwrap();

    player.control(&mut world, settings).await;

    { // CameraCollider
        if is_key_down(KeyCode::Q) && is_key_down(KeyCode::C) && !level_scene_data.trigger_locks.get(&Triggers::ShowCameraColliders).unwrap_or(&false).to_owned() {
            let value = level_scene_data.triggers.get(&Triggers::ShowCameraColliders);
            level_scene_data.triggers.insert(Triggers::ShowCameraColliders, !value.unwrap_or(&false));
            level_scene_data.trigger_locks.insert(Triggers::ShowCameraColliders, true);
        }

        if is_key_released(KeyCode::Q) || is_key_released(KeyCode::C) && level_scene_data.trigger_locks.get(&Triggers::ShowCameraColliders).unwrap_or(&false).to_owned() {
            level_scene_data.trigger_locks.insert(Triggers::ShowCameraColliders, false);
        }

        if level_scene_data.triggers.get(&Triggers::ShowCameraColliders).unwrap_or(&false).to_owned() {
            draw_camera_collider(world, player, settings).await;
        }
    }

    if is_key_down(KeyCode::Escape) {
        *scene = Scene::LevelSelector(0);
    }

    render::render_level(level_scene_data, &textures, &settings, level_scene_data.world.as_ref().unwrap()).await;
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

    LevelSceneData {
        level: Some(Level::Level0),
        player: Some(Player::new(width, height, vec2(pos.x, nv2.y), 0, &mut world).await),
        platforms,
        collectible: vec![],
        world: Some(world),
        triggers: BTreeMap::new(),
        trigger_locks: BTreeMap::new()
    }
}

async fn load_textures(textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>) {
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
        let path = get_platform_path(TextureKey::Platform0).await;
        load_textures_from_tile_map(path).await
    };
    result.insert(TextureKey::Platform0, platform_0);

    // let coin

    // Insert result into the global texture map
    textures.insert(SceneTextureKey::Level0, result);
}