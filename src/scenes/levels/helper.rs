use std::collections::BTreeMap;
use macroquad::prelude::Texture2D;
use crate::scenes::levels::structs::LevelSceneData;
use crate::Settings;
use crate::utils::enums::TextureKey;

pub async fn render_level(level_scene_data: &mut LevelSceneData, textures: &BTreeMap<TextureKey, Vec<Texture2D>>, settings: &Settings) {
    let world = &level_scene_data.world;
    let platforms = &level_scene_data.platforms;
    let collectibles = &mut level_scene_data.collectibles;
    let enemies = &level_scene_data.enemies;

    // Render Player
    level_scene_data.player.unwrap().render(&world, textures).await;

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

pub async fn tick_level(level_scene_data: &mut LevelSceneData, settings: &Settings) {
    {   // Tick collectibles
        let mut collectibles_to_remove = Vec::new();

        for (i, collectible) in level_scene_data.collectibles.iter_mut().enumerate() {
            collectible.check(level_scene_data.player.as_ref().unwrap()).await;
            if collectible.collected {
                collectibles_to_remove.push(i);
            }
        }

        for i in collectibles_to_remove {
            level_scene_data.collectibles.remove(i);
        }
    }
    { // Tick enemies
        let enemies = &mut level_scene_data.enemies;
        let world = &mut level_scene_data.world;
        let player = &mut level_scene_data.player.as_mut().unwrap();

        for enemy in enemies {
            enemy.tick(world, player, settings).await;
        }
    }
}
