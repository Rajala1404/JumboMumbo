use std::collections::BTreeMap;
use macroquad::prelude::Texture2D;
use crate::scenes::levels::levels::LevelSceneData;
use crate::Settings;
use crate::utils::enums::TextureKey;

pub async fn render(level_scene_data: &LevelSceneData, textures: &BTreeMap<TextureKey, Vec<Texture2D>>, settings: &Settings) {
    let world = level_scene_data.world.as_ref().unwrap();
    level_scene_data.player.unwrap().render(&world, textures).await;

}