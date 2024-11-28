use std::collections::BTreeMap;
use macroquad::prelude::Texture2D;
use macroquad_platformer::World;
use crate::scenes::levels::structs::LevelSceneData;
use crate::Settings;
use crate::utils::enums::TextureKey;

pub async fn render_level(level_scene_data: &LevelSceneData, textures: &BTreeMap<TextureKey, Vec<Texture2D>>, settings: &Settings, world: &World) {
    let world = level_scene_data.world.as_ref().unwrap();
    let platforms = &level_scene_data.platforms;

    // Render Player
    level_scene_data.player.unwrap().render(&world, textures).await;

    // Render Platforms
    for platform in platforms {
        platform.render(textures, world).await;
    }
}