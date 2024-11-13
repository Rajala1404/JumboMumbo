use std::collections::BTreeMap;
use macroquad::camera::Camera2D;
use macroquad::prelude::{Texture2D, Vec2};
use macroquad_platformer::{Solid, World};
use crate::{Scene, SceneTextureKey, TextureKey};
use crate::logic::player::Player;
use crate::scenes::levels::level_0::level_0;

/// This enum defines all existing levels
#[derive(PartialEq, Copy, Clone)]
pub enum Level {
    Level0,
}

/// Holds all data a level can possibly have
pub struct LevelSceneData {
    pub level: Option<Level>,
    pub player: Option<Player>,
    pub platforms: Vec<Platform>,
    pub world: Option<World>,
}

#[derive(PartialEq, Copy, Clone)]
pub struct Platform {
    pub collider: Solid,
    pub speed: Vec2,
}

pub async fn start_level(mut scene: &mut Scene, mut camera: &mut Camera2D, mut textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, mut level_scene_data: &mut LevelSceneData) {
    match scene {
        Scene::MainMenu => {}
        Scene::SettingsMenu => {}
        Scene::LevelSelector(_) => {}
        // The cases above shouldn't be possible

        Scene::Level(level) => {
            match level {
                Level::Level0 => {
                    level_0(&mut scene, &mut camera, &mut textures, &mut level_scene_data).await;
                }
            }
        }
    }
}