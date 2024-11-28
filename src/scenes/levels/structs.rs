//! Contains structs (including there implementation) & enums for levels

use macroquad::math::{vec2, Vec2};
use macroquad_platformer::{Actor, Solid, World};
use std::collections::BTreeMap;
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};
use macroquad::color::WHITE;
use crate::logic::player::Player;
use crate::utils::enums::{Animation, TextureKey};

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
    pub collectible: Vec<Collectible>,
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

    /// Basic Floating platform
    /// `length` are the tiles between the start and end
    pub async fn floating(length: i32, tile_size: Vec2, texture_key: TextureKey, pos: Vec2, world: &mut World) -> Self {
        let mut tiles = vec![
            PlatformTile {
                texture_key: TextureKey::Platform0,
                texture_index: 0,
                pos: vec2(0.0, 0.0),
            },
        ];

        for i in 1..length {
            tiles.push(PlatformTile{
                texture_key,
                texture_index: 1,
                pos: vec2(i as f32, 0.0),
            })
        }

        tiles.push(PlatformTile{
            texture_key: TextureKey::Platform0,
            texture_index: 2,
            pos: vec2(length as f32, 0.0),
        });

        Platform{
            collider: world.add_solid(pos, (tile_size.x * (length + 1) as f32) as i32 , tile_size.y as i32),
            tile_size,
            tiles,
            speed: vec2(0.0, 0.0),
        }
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

#[derive(PartialEq, Clone)]
pub struct Collectible {
    pub collected: bool,
    pub collider: Actor,
    pub texture_index: i32,
    pub animation: Animation,
    pub speed: Vec2,
}

impl Collectible {
    /// Runs all checks that may get called onto a collectible
    pub async fn check(&mut self, world: &World) {
        let pos = world.actor_pos(self.collider);
        // Check if the collectible collides with another thing
        let touched = world.collide_check(self.collider, pos);
        if touched {
            self.collected = true;
        }
    }

    pub async fn render(&self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>, world: &World) {

    }
}