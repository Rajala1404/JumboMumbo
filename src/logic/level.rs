use std::collections::BTreeMap;
use macroquad::camera::set_default_camera;
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::Texture2D;
use macroquad::window::{clear_background, screen_height};
use crate::scenes::levels::structs::{CollectibleType, LevelSceneData, Trigger};
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
            let projectiles = &level_scene_data.level_data.projectiles;
            let power_ups = &mut level_scene_data.level_data.power_ups;

            // Render collectibles
            for collectible in collectibles {
                collectible.render(textures).await;
            }

            let platforms = async {
                // Render Platforms
                for platform in platforms {
                    platform.render(textures, world).await;
                }
            };

            let enemies = async {
                // Render enemies
                for enemy in enemies {
                    enemy.render(textures, settings).await;
                }
            };

            let projectiles = async {
                // Render projectiles
                for projectile in projectiles {
                    projectile.render(textures).await;
                }
            };

            // Render power ups
            for power_up in power_ups {
                power_up.render(textures).await;
            }

            platforms.await;
            enemies.await;
            projectiles.await;


            // Render Player
            level_scene_data.level_data.player.as_mut().unwrap().render(&world, textures, settings).await;
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
            if level_scene_data.level_data.collectibles.get(i).unwrap().collectible_type == CollectibleType::Coin {
                level_scene_data.level_data.player.as_mut().unwrap().coins += 1;
            }
            level_scene_data.level_data.collectibles.remove(i);
        }
    }
    { // Tick enemies
        let enemies = &mut level_scene_data.level_data.enemies;
        let projectiles = &mut level_scene_data.level_data.projectiles;
        let world = &mut level_scene_data.world;
        let player = &mut level_scene_data.level_data.player.as_mut().unwrap();

        let mut enemies_to_remove = Vec::new();

        for (i, enemy) in enemies.iter_mut().enumerate() {
            if enemy.deletable {
                enemies_to_remove.push(i);
                continue;
            }
            enemy.tick(world, player, projectiles, settings).await;
        }

        for enemy in enemies_to_remove {
            enemies.remove(enemy);
        }
    }
    { // Tick projectiles
        let mut level_data = level_scene_data.level_data.clone();
        let projectiles = &mut level_scene_data.level_data.projectiles;

        let mut projectiles_to_remove = Vec::new();

        for (i, projectile) in projectiles.iter_mut().enumerate() {
            projectile.tick(&level_data).await;
            level_data.projectiles.remove(i);
            level_data.projectiles.insert(i, projectile.clone());

            if projectile.deletable {
                projectiles_to_remove.push(i);
            }
        }
        level_scene_data.level_data = level_data;

        for projectile in projectiles_to_remove {
            if projectile < level_scene_data.level_data.projectiles.len() {
                level_scene_data.level_data.projectiles.remove(projectile);
            }
        }
    }
    { // Tick power ups
        let player = level_scene_data.level_data.player.as_mut().unwrap();

        let mut power_ups_to_remove = Vec::new();

        for (i, power_up) in level_scene_data.level_data.power_ups.iter_mut().enumerate() {
            power_up.tick(player).await;
            if power_up.collected {
                power_ups_to_remove.push(i);
            }
        }

        for power_up in power_ups_to_remove {
            level_scene_data.level_data.power_ups.remove(power_up);
        }
    }
}
