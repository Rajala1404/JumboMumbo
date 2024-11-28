use std::collections::BTreeMap;
use macroquad::prelude::Texture2D;
use crate::Settings;
use crate::scenes::levels::level_0::level_0;
use crate::scenes::levels::structs::{Level, LevelSceneData};
use crate::utils::enums::{Scene, SceneTextureKey, TextureKey};

pub async fn start_level(mut scene: &mut Scene, mut textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, mut level_scene_data: &mut LevelSceneData, settings: &Settings) {
    match scene {
        Scene::MainMenu => {}
        Scene::SettingsMenu => {}
        Scene::LevelSelector(_) => {}
        // The cases above shouldn't be possible

        Scene::Level(level) => {
            match level {
                Level::Level0 => {
                    level_0(&mut scene, &mut textures, &mut level_scene_data, &settings).await;
                }
            }
        }
    }
}