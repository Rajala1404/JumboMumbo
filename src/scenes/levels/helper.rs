use std::collections::BTreeMap;
use macroquad::camera::set_default_camera;
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::Texture2D;
use macroquad::window::{clear_background, screen_height};
use crate::scenes::levels::structs::{LevelSceneData, Trigger};
use crate::utils::structs::Settings;
use crate::utils::enums::TextureKey;
use crate::utils::text::{draw_text_center, draw_text_centered};

pub async fn render_level(level_scene_data: &mut LevelSceneData, textures: &BTreeMap<TextureKey, Vec<Texture2D>>, settings: &Settings) {
    match level_scene_data.level_data.triggers.get(&Trigger::GameOver).unwrap_or(&false) {
        true => {
            set_default_camera();
            clear_background(BLACK);
            draw_text_center("GAME OVER", 300.0 * settings.gui_scale, WHITE).await;
            draw_text_centered("Press ESC to go back", screen_height() / 2.0 + 250.0 * settings.gui_scale, 100.0 * settings.gui_scale, WHITE).await;
        }
        false => {
            let world = &level_scene_data.world;
            let platforms = &level_scene_data.level_data.platforms;
            let collectibles = &mut level_scene_data.level_data.collectibles;
            let enemies = &level_scene_data.level_data.enemies;

            // Render Player
            level_scene_data.level_data.player.as_mut().unwrap().render(&world, textures, settings).await;

            // Render Platforms
            for platform in platforms {
                platform.render(textures, world).await;
            }

            // Render collectibles
            for collectible in collectibles {
                collectible.render(textures).await;
            }

            // Render enemies
            for enemy in enemies {
                enemy.render(textures).await;
            }
        }
    }
}

pub async fn tick_level(level_scene_data: &mut LevelSceneData, settings: &Settings) {
    {   // Tick collectibles
        let mut collectibles_to_remove = Vec::new();

        for (i, collectible) in level_scene_data.level_data.collectibles.iter_mut().enumerate() {
            collectible.check(level_scene_data.level_data.player.as_ref().unwrap()).await;
            if collectible.collected {
                collectibles_to_remove.push(i);
            }
        }

        for i in collectibles_to_remove {
            level_scene_data.level_data.collectibles.remove(i);
        }
    }
    { // Tick enemies
        let enemies = &mut level_scene_data.level_data.enemies;
        let world = &mut level_scene_data.world;
        let player = &mut level_scene_data.level_data.player.as_mut().unwrap();

        for enemy in enemies {
            enemy.tick(world, player, settings).await;
        }
    }
}
