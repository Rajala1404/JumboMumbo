//! Contains structs (including there implementation) & enums for levels

use macroquad::math::{vec2, Vec2};
use macroquad_platformer::{Solid, World};
use std::collections::BTreeMap;
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};
use macroquad::color::WHITE;
use macroquad::time::{get_frame_time, get_time};
use crate::logic::collider::Collider;
use crate::logic::enemy::Enemy;
use crate::logic::player::Player;
use crate::utils::enums::{Animation, AnimationType, TextureKey};
/// This enum defines all existing levels
#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Debug)]
pub enum Level {
    Level0,
}

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Debug)]
pub enum Trigger {
    ShowCameraColliders,
    ShowColliders,
    ShowFPS,

    GameOver,
}

#[derive(Clone)]
pub struct LevelData {
    pub level: Option<Level>,
    pub player: Option<Player>,
    pub platforms: Vec<Platform>,
    pub collectibles: Vec<Collectible>,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    /// Saves temporary triggers / settings
    pub triggers: BTreeMap<Trigger, bool>,
    pub trigger_locks: BTreeMap<Trigger, bool>
}

/// Holds all data a level can possibly have
pub struct LevelSceneData {
    pub level_data: LevelData,
    pub world: World,
}

impl LevelSceneData {
    pub async fn empty() -> Self {
        let level_data = LevelData {
            level: None,
            player: None,
            platforms: Vec::new(),
            collectibles: Vec::new(),
            enemies: Vec::new(),
            projectiles: Vec::new(),
            triggers: BTreeMap::new(),
            trigger_locks: BTreeMap::new()
        };

        Self {
            level_data,
            world: World::new(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Platform {
    pub collider: Solid,
    pub collider_new: Collider,
    pub tile_size: Vec2,
    pub tiles: Vec<PlatformTile>,
    pub speed: Vec2,
}

impl Platform {
    pub async fn new(collider: Solid, pos: Vec2, size: Vec2, tile_size: Vec2, tiles: Vec<PlatformTile>, speed: Vec2) -> Self {
        let collider_new = Collider::new_solid(pos, size.x, size.y, vec2(0.0, 0.0)).await;
        Self { collider, collider_new, tile_size, tiles, speed }
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

        let width = tile_size.x * (length + 1) as f32;

        Self::new(
            world.add_solid(pos, width as i32, tile_size.y as i32),
            pos,
            vec2(width, tile_size.y),
            tile_size,
            tiles,
            vec2(0.0, 0.0)
        ).await
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

#[derive(PartialEq, Clone, Debug)]
pub struct Collectible {
    pub collected: bool,
    pub collider: Collider,
    pub texture_key: TextureKey,
    pub animation: Animation,
    pub size: Vec2,
    pub speed: Vec2,
}

impl Collectible {
    /// Runs all checks that may get called onto a collectible
    pub async fn check(&mut self, player: &Player) {
        // Check if the collectible collides with another thing
        if self.collider.touching_player(player).await {
            self.collected = true;
        }
    }

    pub async fn render(&mut self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>) {
        let pos = self.collider.pos().await;

        match self.animation.animation_type {
            AnimationType::Cycle(_, _) => {
                self.animation.animate().await;
                let texture = textures.get(&self.texture_key).unwrap().get(self.animation.index as usize).unwrap();
                draw_texture_ex(
                    texture,
                    pos.x,
                    pos.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(self.size),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Projectile {
    pub active: bool,
    pub deletable: bool,
    pub pos: Vec2,
    pub size: Vec2,
    pub start_time: f64,
    pub max_time: f64,
    pub collider: Collider,
    pub damage: i16,
    pub texture_key: TextureKey,
    pub origin: ProjectileOrigin,
    pub speed: Vec2,
}

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Debug)]
pub enum ProjectileOrigin {
    Player,
}

impl Projectile {
    pub async fn new(pos: Vec2, size: Vec2, damage: i16, max_time: f64, texture_key: TextureKey, origin: ProjectileOrigin, speed: Vec2) -> Self {
        let start_time = get_time();
        let collider = Collider::new_projectile(pos, size.x, size.y, vec2(0.0, 0.0)).await;

        Self {
            active: true,
            deletable: false,
            pos,
            size,
            start_time,
            max_time,
            collider,
            damage,
            texture_key,
            origin,
            speed,
        }
    }

    pub async fn tick(&mut self, level_data: &LevelData) {
        if !self.collider.collide_check_platform(&level_data.platforms, vec2(0.0, 0.0)).await.is_empty() || self.start_time + self.max_time < get_time(){
            self.active = false;
            self.deletable = true;
        } else {
            self.perform_move().await;
        }
    }

    async fn perform_move(&mut self) {
        let new_pos = self.pos + self.speed * get_frame_time();
        self.pos = new_pos;
        self.collider.change_pos(self.pos).await;
    }

    // TODO: Implement Animation
    pub async fn render(&self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>) {
        draw_texture_ex(
            &textures.get(&self.texture_key).unwrap().get(0).unwrap(), self.pos.x, self.pos.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.size.x, self.size.y)),
                ..Default::default()
            },
        );
    }
}