use std::collections::BTreeMap;
use macroquad::prelude::Texture2D;
use crate::logic::level::{Level, LevelSceneData, PersistentLevelData};
use crate::utils::structs::Settings;
use crate::scenes::levels::level_0::level_0;
use crate::scenes::levels::level_1::level_1;
use crate::scenes::levels::level_2::level_2;
use crate::scenes::levels::level_3::level_3;
use crate::utils::enums::{Scene, SceneTextureKey, TextureKey};

pub async fn start_level(scene: &mut Scene, textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, level_scene_data: &mut LevelSceneData, persistent_level_data: &mut PersistentLevelData, settings: &Settings) {
    match scene {
        Scene::Level(level) => {
            match level {
                Level::Level0 => {
                    level_0(scene, textures, level_scene_data, persistent_level_data, &settings).await;
                }
                Level::Level1 => level_1(scene, textures, level_scene_data, persistent_level_data, &settings).await,
                Level::Level2 => level_2(scene, textures, level_scene_data, persistent_level_data, &settings).await,
                Level::Level3 => level_3(scene, textures, level_scene_data, persistent_level_data, &settings).await,
            }
        }
        _ => return,
    }
}