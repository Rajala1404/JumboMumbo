use std::collections::BTreeMap;
use macroquad::color::WHITE;
use macroquad::prelude::{Texture2D, Vec2};
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad_platformer::{Solid, World};
use crate::Settings;
use crate::logic::player::Player;
use crate::scenes::levels::level_0::level_0;
use crate::utils::enums::{Scene, SceneTextureKey, TextureKey};

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

impl Platform {
    pub async fn new(collider: Solid, tile_size: Vec2, tiles: Vec<PlatformTile>, speed: Vec2) -> Self {
        Self { collider, tile_size, tiles, speed }
    }

    pub async fn render(&self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>, world: &World) {
        let pos = world.solid_pos(self.collider);

        for tile in &self.tiles {
            let texture = textures.get(&tile.texture_key).unwrap().get(tile.texture_index).unwrap();
            let pos =  pos + self.tile_size * tile.pos;
            draw_texture_ex(
                &texture,
                pos.x,
                pos.y,
                WHITE,
                DrawTextureParams{
                    dest_size: Some(self.tile_size),
                    ..Default::default()
                }
            )
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct PlatformTile {
    pub texture_key: TextureKey,
    /// The usize is the Index of the texture inside the TileMap. <br>
    /// For more info please see the json of the platform you're trying to render
    pub texture_index: usize,
    /// Contains the relative position of the Platform tile (e.g. vec2(1.0, 0.0))
    pub pos: Vec2,
}

impl PlatformTile {
    pub async fn new(texture_key: TextureKey, texture_index: usize, pos: Vec2) -> Self {
        Self {texture_key, texture_index, pos}
    }
}

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