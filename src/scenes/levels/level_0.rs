use crate::logic::level;
use std::collections::BTreeMap;
use crate::Settings;
use macroquad::prelude::*;
use macroquad_platformer::World;
use crate::logic::collectible::{Collectible, CollectibleType};
use crate::logic::collider::Collider;
use crate::logic::level::{Level, LevelData, LevelSceneData, PersistentLevelData, Trigger};
use crate::logic::platform::{Platform, PlatformTile};
use crate::logic::player::{Player, PlayerUIElementType};
use crate::utils::debugger;
use crate::utils::enums::{Animation, AnimationType, Scene, SceneTextureKey, TextureKey};
use crate::utils::text::{draw_text_center, draw_text_centered};
use crate::utils::texture::load_level_textures;

pub async fn level_0(scene: &mut Scene, textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, level_scene_data: &mut LevelSceneData, persistent_level_data: &mut PersistentLevelData, settings: &Settings) {
    clear_background(DARKBLUE);

    // Load textures if not loaded already
    if textures.get(&SceneTextureKey::Level0).is_none() {
        let keys = [
            TextureKey::Platform0,
            TextureKey::Player,
            TextureKey::Coin0,
            TextureKey::Icons0,
            TextureKey::Projectile0
        ].to_vec();
        textures.insert(SceneTextureKey::Level0, load_level_textures("Tutorial", keys).await);
    }

    if is_key_down(KeyCode::Escape) {
        *scene = Scene::LevelSelector(0);
        level_scene_data.level_data.save(persistent_level_data, settings).await;
        *level_scene_data = LevelSceneData::empty().await;
        textures.remove(&SceneTextureKey::Level0);
        set_default_camera();
        return;
    }

    let textures = textures.get(&SceneTextureKey::Level0).unwrap();

    // Load scene data for right level
    if level_scene_data.level_data.level != Some(Level::Level0) {
        *level_scene_data = layout(settings).await;
    }

    let width = 128.0 * settings.gui_scale;
    let height = 128.0 * settings.gui_scale;
    let size = vec2(width, height);

    { // Tutorial Text
        let level_data = &mut level_scene_data.level_data;
        let triggers = &mut level_data.triggers;
        let triggers_exec = &mut level_data.triggers_exec;

        let walked = triggers.get(&Trigger::TutorialWalking).unwrap_or(&false).to_owned();

        if (is_key_down(KeyCode::A) || is_key_down(KeyCode::D)) && !walked { triggers.insert(Trigger::TutorialWalking, true); triggers_exec.insert(Trigger::TutorialWalking, get_time()); }

        if !walked {
            draw_text("Use A & D or the Arrow keys to move Right and Left", size.x * -18.0, screen_height() - (size.y * 5.0), 64.0 * settings.gui_scale, WHITE);
        } else if triggers_exec.get(&Trigger::TutorialWalking).unwrap_or(&0.0) + 3.0 > get_time() {
            draw_text("Great!", size.x * -18.0, screen_height() - (size.y * 5.0), 64.0 * settings.gui_scale, WHITE);
        }

        let jumped = triggers.get(&Trigger::TutorialSpace).unwrap_or(&false).to_owned();
        let platform = level_data.platforms.get(1).unwrap();

        if platform.collider_new.touching_player(level_data.player.as_ref().unwrap()).await && !jumped {
            triggers.insert(Trigger::TutorialSpace, true);
            triggers_exec.insert(Trigger::TutorialSpace, get_time());
        }

        if !jumped && walked {
            draw_text("Use Space to jump on this Platform", platform.collider_new.rect.x - platform.collider_new.rect.w / 2.0, screen_height() - (size.y * 5.0), 64.0 * settings.gui_scale, WHITE);
        } else if triggers_exec.get(&Trigger::TutorialSpace).unwrap_or(&0.0) + 3.0 > get_time() {
            draw_text("Amazing!", platform.collider_new.rect.x , screen_height() - (size.y * 5.0), 64.0 * settings.gui_scale, WHITE);
        }

        let collected_one_coin = level_data.player.as_ref().unwrap().coins >= 1;

        if collected_one_coin && !triggers.get(&Trigger::TutorialCoins).unwrap_or(&false).to_owned() {
            triggers.insert(Trigger::TutorialCoins, true);
            triggers_exec.insert(Trigger::TutorialCoins, get_time());
        }

        if !collected_one_coin && jumped {
            draw_text("Collect all coins!", size.x * 12.0, screen_height() - (size.y * 7.5), 64.0 * settings.gui_scale, WHITE);
        } else if triggers_exec.get(&Trigger::TutorialCoins).unwrap_or(&0.0) + 3.0 > get_time() {
            let coins_ui = level_data.player.as_ref().unwrap().ui_elements.get(&PlayerUIElementType::Coins).unwrap();
            draw_rectangle(coins_ui.pos.x + coins_ui.texture_size.x / 8.0, coins_ui.pos.y, coins_ui.texture_size.x, coins_ui.texture_size.y, RED);
            draw_text("You can see your collected coins in the top left corner of the screen", size.x * 10.0, screen_height() - (size.y * 7.0), 32.0 * settings.gui_scale, WHITE);
        }
    }

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
        draw_text_centered("You completed the Tutorial! Press ESC to go back", screen_height() / 2.0 + 250.0 * settings.gui_scale, 100.0 * settings.gui_scale, WHITE).await;
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
            collider_new: Collider::new_solid(pos,width * 41.0, height * 2.0, vec2(0.0, 0.0)).await,
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

    { // Floating Platform
        let pos = vec2(size.x * 12.0, screen_height() - (size.y * 5.0 + size.y / 4.0));

        platforms.push(Platform::floating(
            3,
            size,
            TextureKey::Platform0,
            pos,
            &mut world,
        ).await);

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
    }

    platforms.push(Platform::floating(
        3,
        size,
        TextureKey::Platform0,
        vec2(size.x * 18.0, screen_height() - (size.y * 8.0)),
        &mut world
    ).await);

    collectibles.push(Collectible::new(
        CollectibleType::Coin,
        vec2(size.x * 19.5, screen_height() - size.y * 10.0),
        size,
        TextureKey::Coin0,
        Animation::new(AnimationType::Cycle(0, 5, 0.1)),
        nv2,
    ).await);

    let pos = vec2(size.x * -17.0, 0.0);
    LevelSceneData {
        level_data: LevelData {
            start_time: get_time(),

            zero: vec2(0.0, 0.0),

            level: Some(Level::Level0),
            player: Some(Player::new(size.x, size.y, vec2(pos.x, nv2.y), 0, &mut world).await),
            platforms,
            collectibles,
            enemies,
            cannons,
            projectiles: Vec::new(),
            power_ups,
            triggers: BTreeMap::new(),
            triggers_exec: BTreeMap::new(),
            trigger_locks: BTreeMap::new() },
        world
    }
}