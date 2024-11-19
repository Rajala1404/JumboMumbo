use std::collections::BTreeMap;
use macroquad::prelude::{Texture2D, Vec2};
use macroquad_platformer::{Solid, World};
use crate::Scene;
use crate::logic::player::Player;
use crate::scenes::levels::level_0::level_0;
use crate::utils::enums::{SceneTextureKey, TextureKey};

/// This enum defines all existing levels
#[derive(PartialEq, Clone)]
pub enum Level {
    Level0,
}

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd)]
pub enum Triggers {
    ShowCameraColliders,
}

/// Holds all data a level can possibly have
pub struct LevelSceneData {
    pub level: Option<Level>,
    pub player: Option<Player>,
    pub platforms: Vec<Platform>,
    pub world: Option<World>,
    /// Saves temporary triggers / settings
    pub triggers: BTreeMap<Triggers, bool>,
    pub trigger_locks: BTreeMap<Triggers, bool>,
}

#[derive(PartialEq, Clone)]
pub struct Platform {
    pub collider: Solid,
    pub tile_size: Vec2,
    pub tiles: Vec<PlatformTile>,
    pub speed: Vec2,
}

#[derive(PartialEq, Clone)]
pub struct PlatformTile {
    pub texture_key: TextureKey,
    pub pos: Vec2,
}

pub async fn start_level(mut scene: &mut Scene, mut textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, mut level_scene_data: &mut LevelSceneData) {
    match scene {
        Scene::MainMenu => {}
        Scene::SettingsMenu => {}
        Scene::LevelSelector(_) => {}
        // The cases above shouldn't be possible

        Scene::Level(level) => {
            match level {
                Level::Level0 => {
                    level_0(&mut scene, &mut textures, &mut level_scene_data).await;
                }
            }
        }
    }
}