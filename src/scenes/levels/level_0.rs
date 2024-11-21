use crate::utils::render;
use std::collections::BTreeMap;
use crate::{Scene, Settings};
use macroquad::prelude::*;
use macroquad_platformer::World;
use crate::logic::player::Player;
use crate::scenes::levels::levels::{Level, LevelSceneData, Platform, Triggers};
use crate::utils::debugger::draw_camera_collider;
use crate::utils::enums::{SceneTextureKey, TextureKey};

pub async fn level_0(scene: &mut Scene, mut textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, level_scene_data: &mut LevelSceneData, settings: &Settings) {
    clear_background(DARKBLUE);

    // Load textures if not loaded already
    if textures.get(&SceneTextureKey::Level0).is_none() {
        load_textures(&mut textures).await;
    }
    let textures = textures.get(&SceneTextureKey::Level0).unwrap();

    // Load scene data for right level
    if level_scene_data.level != Some(Level::Level0) {
        *level_scene_data = layout().await;
    }

    let mut world = level_scene_data.world.as_mut().unwrap();
    let player = level_scene_data.player.as_mut().unwrap();

    player.control(&mut world).await;
    render::render(&level_scene_data, &textures, &settings).await;

    { // CameraCollider
        if is_key_down(KeyCode::Q) && is_key_down(KeyCode::C) && !level_scene_data.trigger_locks.get(&Triggers::ShowCameraColliders).unwrap_or(&false).to_owned() {
            println!("Executed!");
            let value = level_scene_data.triggers.get(&Triggers::ShowCameraColliders);
            level_scene_data.triggers.insert(Triggers::ShowCameraColliders, !value.unwrap_or(&false));
            level_scene_data.trigger_locks.insert(Triggers::ShowCameraColliders, true);
        }

        if is_key_released(KeyCode::Q) || is_key_released(KeyCode::C) {
            level_scene_data.trigger_locks.insert(Triggers::ShowCameraColliders, false);
        }

        if level_scene_data.triggers.get(&Triggers::ShowCameraColliders).unwrap_or(&false).to_owned() {
            draw_camera_collider(world, player).await;
        }
    }

    draw_line(screen_width() / 2.0, screen_height() / 2.0, screen_width() / 2.0 + 100.0, screen_height() / 2.0 + 100.0, 10.0, WHITE);
}

async fn layout() -> LevelSceneData {

    let mut world = World::new();
    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;
    let width = screen_height() / 12.0;
    let height = screen_height() / 12.0;
    let pos = vec2(x, y);
    let nv2 = vec2(0.0, 0.0);

    let mut platforms = vec![];

    { // Base Platform
        let pos = vec2(0.0 - screen_width(), screen_height());
        let size = vec2(width, height);

        platforms.push(
            Platform {
                collider: world.add_solid(pos, screen_width()  as i32 * 3, (screen_height() / 32.0) as i32 ),
                tile_size: size,
                tiles: vec![],
                speed: nv2
            }
        );
    }



    LevelSceneData {
        level: Some(Level::Level0),
        player: Some(Player::new(width, height, pos, 0, &mut world).await),
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



    // Insert result into the global texture map
    textures.insert(SceneTextureKey::Level0, result);
}