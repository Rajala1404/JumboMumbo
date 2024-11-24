use crate::utils::render;
use std::collections::BTreeMap;
use crate::Settings;
use macroquad::prelude::*;
use macroquad_platformer::World;
use crate::logic::player::Player;
use crate::scenes::levels::levels::{Level, LevelSceneData, Platform, PlatformTile, Triggers};
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

        if is_key_released(KeyCode::Q) || is_key_released(KeyCode::C) {
            level_scene_data.trigger_locks.insert(Triggers::ShowCameraColliders, false);
        }

        if level_scene_data.triggers.get(&Triggers::ShowCameraColliders).unwrap_or(&false).to_owned() {
            draw_camera_collider(world, player, settings).await;
        }
    }

    draw_line(screen_width() / 2.0, screen_height() / 2.0, screen_width() / 2.0 + 100.0, screen_height() / 2.0 + 100.0, 10.0, WHITE);

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

    { // Base Platform
        let pos = vec2(0.0 - screen_width(), screen_height());
        let size = vec2(width, height);

        platforms.push(
            Platform {
                collider: world.add_solid(pos, screen_width() as i32 * 3, (screen_height() / 32.0) as i32 ),
                tile_size: size,
                tiles: vec![],
                speed: nv2
            }
        );
    }
    { // Test Platform
        let pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let width = width * 3.0;
        let height = height * 3.0;

        let mut tiles = vec![];
        // Just all keys
        for i in 0..9 {
            let pos = {
                match i {
                    0 => vec2(0.0, 0.0),
                    1 => vec2(1.0, 0.0),
                    2 => vec2(2.0, 0.0),
                    3 => vec2(0.0, 1.0),
                    4 => vec2(1.0, 1.0),
                    5 => vec2(2.0, 1.0),
                    6 => vec2(0.0, 2.0),
                    7 => vec2(1.0, 2.0),
                    8 => vec2(2.0, 2.0),
                    _ => unimplemented!()
                }
            };
            tiles.push(PlatformTile::new(TextureKey::Platform0, i, pos).await)
        }

        let platform = Platform::new(
            world.add_solid(pos, width as i32, height as i32),
            size,
            tiles,
            nv2
        ).await;

        platforms.push(platform);
    }
    { // Test Platform 2
        let pos = vec2(screen_width() / 4.0, screen_height() - screen_height() / 4.0);

        let width = width * 3.0;
        let height = height * 3.0;

        let mut tiles = vec![];
        // Just all keys
        for i in 0..9 {
            let pos = {
                match i {
                    0 => vec2(0.0, 0.0),
                    1 => vec2(1.0, 0.0),
                    2 => vec2(2.0, 0.0),
                    3 => vec2(0.0, 1.0),
                    4 => vec2(1.0, 1.0),
                    5 => vec2(2.0, 1.0),
                    6 => vec2(0.0, 2.0),
                    7 => vec2(1.0, 2.0),
                    8 => vec2(2.0, 2.0),
                    _ => unimplemented!()
                }
            };
            tiles.push(PlatformTile::new(TextureKey::Platform0, i, pos).await)
        }

        let platform = Platform::new(
            world.add_solid(pos, width as i32, height as i32),
            size,
            tiles,
            nv2
        ).await;

        platforms.push(platform);
    }

    LevelSceneData {
        level: Some(Level::Level0),
        player: Some(Player::new(width, height, vec2(pos.x, nv2.y), 0, &mut world).await),
        platforms,
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



    // Insert result into the global texture map
    textures.insert(SceneTextureKey::Level0, result);
}