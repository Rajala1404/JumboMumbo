//! Contains structs (including there implementation) & enums for levels

use macroquad::math::{vec2, Vec2};
use macroquad_platformer::{Actor, Solid, World};
use std::collections::BTreeMap;
use macroquad::prelude::{draw_texture_ex, get_frame_time, DrawTextureParams, Texture2D};
use macroquad::color::WHITE;
use crate::logic::collider::Collider;
use crate::logic::player::Player;
use crate::Settings;
use crate::utils::enums::{Animation, AnimationType, Direction, TextureKey};

/// This enum defines all existing levels
#[derive(PartialEq, Clone)]
pub enum Level {
    Level0,
}

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Debug)]
pub enum Trigger {
    ShowCameraColliders,
    ShowColliders,
}

/// Holds all data a level can possibly have
pub struct LevelSceneData {
    pub level: Option<Level>,
    pub player: Option<Player>,
    pub platforms: Vec<Platform>,
    pub collectibles: Vec<Collectible>,
    pub enemies: Vec<Enemy>,
    pub world: World,
    /// Saves temporary triggers / settings
    pub triggers: BTreeMap<Trigger, bool>,
    pub trigger_locks: BTreeMap<Trigger, bool>,
}

impl LevelSceneData {
    pub async fn empty() -> Self {
        Self {
            level: None,
            player: None,
            platforms: Vec::new(),
            collectibles: Vec::new(),
            enemies: Vec::new(),
            world: World::new(),
            triggers: BTreeMap::new(),
            trigger_locks: BTreeMap::new()
        }
    }
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

#[derive(PartialEq, Clone)]
pub struct Enemy {
    pub size: Vec2,
    pub texture_key: TextureKey,
    pub pos: Vec2,
    pub start_pos: Vec2,
    pub trigger_right: Collider,
    pub trigger_left: Collider,
    pub collider: Collider,
    pub world_collider: Actor,
    pub state: EnemyState,
    pub waiters: BTreeMap<EnemyWaiter, bool>,
    pub behavior: Vec<EnemyBehavior>,
    pub speed: Vec2,
}

#[derive(PartialEq, Clone)]
pub enum EnemyState {
    Idling,
    Attacking,

}

/// Contains random things that an enemy may do or is needed to execute some bs
#[derive(PartialEq, Clone, Ord, Eq, PartialOrd)]
pub enum EnemyWaiter {
    /// `true` = Right
    /// `false`= Left
    IdlingDirection
}

#[derive(PartialEq, Clone, Ord, Eq, PartialOrd)]
pub enum EnemyBehavior {
    Move(Direction)
}

impl Enemy {
    pub async fn new(pos: Vec2, x_range: f32, y_range: f32, world: &mut World, size: Vec2, texture_key: TextureKey) -> Self {
        let width = size.x;
        let height = size.y;

        Self {
            size,
            texture_key,
            pos: pos + vec2(1.0, 0.0),
            start_pos: pos,
            trigger_right: Collider::new_trigger(pos + size.x, x_range, y_range).await,
            trigger_left: Collider::new_trigger(pos - width - x_range, x_range, y_range).await,
            collider: Collider::new_enemy(pos, width, height).await,
            world_collider: world.add_actor(pos, width as i32, height as i32),
            state: EnemyState::Idling,
            behavior: Vec::new(),
            waiters: BTreeMap::new(),
            speed: vec2(0.0, 0.0),
        }
    }

    pub async fn tick(&mut self, world: &mut World, player: &mut Player, settings: &Settings) {

        // The same as for the player
        // SP Start
        let pos = world.actor_pos(self.world_collider);
        self.pos = pos;
        let on_ground = world.collide_check(self.world_collider, pos + vec2(0.0, 1.0));
        let sealing_hit = world.collide_check(self.world_collider, pos + vec2(0.0, -1.0));

        if sealing_hit {
            self.speed.y = (100.0 * settings.gui_scale) * get_frame_time()
        }

        if !on_ground {
            self.speed.y += (4800.0 * settings.gui_scale) * get_frame_time();
        } else {
            self.speed.y = 0.0;
        }
        // SP End

        // "AI"
        match self.state {
            EnemyState::Attacking => {

            },
            EnemyState::Idling => {
                let touched_right = self.trigger_right.touching_player(player);
                let touched_left = self.trigger_left.touching_player(player);
                if touched_right.await || touched_left.await {
                    self.state = EnemyState::Attacking;
                    self.behavior.clear();
                } else {
                    if *self.waiters.get(&EnemyWaiter::IdlingDirection).unwrap_or(&true) {
                        // Why the fuck does this function check so wierd
                        if world.collide_check(self.world_collider, pos + vec2(self.size.x + 1.0, 1.0)) {
                            self.waiters.insert(EnemyWaiter::IdlingDirection, true);
                            self.behavior.push(EnemyBehavior::Move(Direction::Right));
                        } else {
                            self.waiters.insert(EnemyWaiter::IdlingDirection, false);
                        }
                    } else {
                        // Same here
                        if world.collide_check(self.world_collider, pos + vec2(-self.size.x - 1.0, 1.0)) {
                            self.waiters.insert(EnemyWaiter::IdlingDirection, false);
                            self.behavior.push(EnemyBehavior::Move(Direction::Left));
                        } else {
                            self.waiters.insert(EnemyWaiter::IdlingDirection, true);
                        }
                    }
                }
            },
        }
        self.speed.x = 0.0;
        for behavior in &self.behavior {
            match behavior {
                EnemyBehavior::Move(direction) => {
                    match direction {
                        Direction::Right => {
                            self.speed.x = 500.0 * settings.gui_scale;
                        }
                        Direction::Left => {
                            self.speed.x = -500.0 * settings.gui_scale;
                        }
                        _ => unimplemented!()
                    }
                }
            }
        }
        self.behavior.clear();

        // Set positions using the previously defined speeds
        world.move_h(self.world_collider, self.speed.x * get_frame_time());
        world.move_v(self.world_collider, self.speed.y * get_frame_time());

        let pos = world.actor_pos(self.world_collider);
        self.pos = pos;
        self.update_pos().await;
    }

    async fn update_pos(&mut self) {
        self.trigger_left.change_pos(self.pos - vec2(self.trigger_left.rect.w, 0.0)).await;
        self.trigger_right.change_pos(self.pos + vec2(self.size.x, 0.0)).await;
        self.collider.change_pos(self.pos).await;
    }

    pub async fn render(&self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>) {
        let texture = textures.get(&self.texture_key).unwrap().get(0).unwrap();
        draw_texture_ex(
            texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size),
                ..Default::default()
            },
        );
    }
}