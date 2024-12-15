use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use macroquad::math::{f32, vec2, Vec2};
use macroquad_platformer::{Solid, World};
use macroquad::prelude::{draw_texture_ex, get_frame_time, get_time, DrawTextureParams, Texture2D};
use macroquad::color::WHITE;
use stopwatch2::Stopwatch;
use crate::logic::collider::Collider;
use crate::logic::enemy::Enemy;
use crate::logic::player::{Player, PowerUp};
use crate::utils::enums::{Animation, AnimationType, TextureKey};

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub async fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    /// Converts a [Rect] into a [macroquad::math::Rect]
    pub async fn to_macro_rect(&self) -> macroquad::math::Rect {
        macroquad::math::Rect::new(
            self.x,
            self.y,
            self.w,
            self.h,
        )
    }

    /// Checks whether the `Rect` overlaps another `Rect`
    pub async fn overlaps(&self, other: &Rect) -> bool {
        self.left().await <= other.right().await
            && self.right().await >= other.left().await
            && self.top().await <= other.bottom().await
            && self.bottom().await >= other.top().await
    }

    /// Returns the left edge
    pub async fn left(&self) -> f32 {
        self.x
    }

    /// Returns the right edge
    pub async fn right(&self) -> f32 {
        self.x + self.w
    }

    /// Returns the top edge
    pub async fn top(&self) -> f32 {
        self.y
    }

    /// Returns the bottom edge
    pub async fn bottom(&self) -> f32 {
        self.y + self.h
    }
}

/// Basically the same as a `Vec2` just with `i32` instead of `f32`
#[derive(PartialEq, Clone, Debug)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub fn vec2i(x: i32, y: i32) -> Vec2i {
    Vec2i { x, y }
}

impl From<macroquad::math::Vec2> for Vec2i {
    fn from(vec: macroquad::math::Vec2) -> Self {
        Vec2i {
            x: vec.x.round() as i32,
            y: vec.y.round() as i32,
        }
    }
}


#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    /// The Path of the Game's config directory
    pub path: String,
    pub level_data_path: String,
    pub gui_scale: f32,
}

/// Temporary Saves settings before they get applied
#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct TempSettings {
    pub settings: Settings,
}

impl Settings {
    pub async fn new(path: String, level_data_path: String) -> Settings {
        Settings {
            path,
            level_data_path,
            gui_scale: 1.0,
        }
    }
}

/// A 2D Matrix
#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Matrix<T> {
    pub data: BTreeMap<(i32, i32), T>
}

impl<T> Matrix<T> {
    /// Creates a new empty [Matrix]
    pub fn new() -> Self {
        Self { data: BTreeMap::new() }
    }

    /// Insert a value at a specific position <br>
    /// If a value already exists at that Position then that value will be overwritten
    pub fn insert(&mut self, row: i32, col: i32, value: T) {
        self.data.insert((row, col), value);
    }

    /// Get a value from a specific position
    pub fn get(&self, row: i32, col: i32) -> Option<&T> {
        self.data.get(&(row, col))
    }

    /// Get the lowest and highes colum/row <br>
    /// `0` is min <br>
    /// `1` is max
    pub async fn bounds(&self) -> [Vec2i; 2] {
        let min_row = self.data.keys().map(|(row, _)| row).min().unwrap_or(&0).to_owned();
        let min_col = self.data.keys().map(|(_, colum)| colum).min().unwrap_or(&0).to_owned();
        let max_row = self.data.keys().map(|(row, _)| row).max().unwrap_or(&0).to_owned();
        let max_col = self.data.keys().map(|(_, colum)| colum).max().unwrap_or(&0).to_owned();
        [
            vec2i(min_row, min_col),
            vec2i(max_row, max_col)
        ]
    }
}

impl<T> Into<Vec<T>> for Matrix<T> where T: Clone {
    fn into(self) -> Vec<T> {
        self.data.iter().map(| (_, t)| (*t).clone()).collect()
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = ((i32, i32), T);
    type IntoIter = std::collections::btree_map::IntoIter<(i32, i32), T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = (&'a (i32, i32), &'a T);
    type IntoIter = std::collections::btree_map::Iter<'a,(i32, i32), T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix<T> {
    type Item = (&'a (i32, i32), &'a mut T);
    type IntoIter = std::collections::btree_map::IterMut<'a,(i32, i32), T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
/// Contains ALL data that may be saved across multiple sessions of levels
pub struct PersistentLevelData {
    pub stats: BTreeMap<Level, LevelStat>,
    pub scores: BTreeMap<Level, Vec<LevelScore>>
}

impl PersistentLevelData {
    pub fn new() -> Self {
        let stats = BTreeMap::new();
        let scores= BTreeMap::new();

        Self { stats, scores }
    }

    pub async fn save(&self, settings: &Settings) {
        let mut file = fs::File::create(&settings.level_data_path).unwrap();

        let s_persistent_level_data = serde_json::to_string_pretty(&self).expect("Couldn't serialize level data");
        file.write_all(s_persistent_level_data.as_bytes()).expect("Couldn't write level data file");
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct LevelStat {
    pub level: Level,
    /// The total amount of tries to play the level
    pub plays: u32,
    /// The total amount of all deaths
    pub deaths: u32,
}

impl LevelStat {
    pub fn new(level: Level) -> Self {
        Self { level, plays: 0, deaths: 0 }
    }

    pub fn update(&mut self, deaths: u32) {
        self.plays += 1;
        self.deaths += deaths;
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct LevelScore {
    /// The time this was created
    pub time: SystemTime,
    /// The total playtime of the level
    pub playtime: f64,
    /// The total amount of collected coins
    pub coins: u32,
    /// The total amount of kills
    pub kills: u32,
    /// The total mount of damage that the player has done
    pub total_damage: u32,
    /// The total amount of damage that the player received
    pub total_damage_received: u32
}

impl LevelScore {
    pub fn new(playtime: f64, coins: u32, kills: u32, total_damage: u32, total_damage_received: u32) -> LevelScore {
        let time = SystemTime::now();
        Self { time, playtime, coins, kills, total_damage, total_damage_received }
    }
}

/// This enum defines all existing levels
#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Serialize, Deserialize, Debug)]
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
    pub start_time: f64,

    pub level: Option<Level>,
    pub player: Option<Player>,
    pub platforms: Vec<Platform>,
    pub collectibles: Vec<Collectible>,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    pub power_ups: Vec<PowerUp>,
    /// Saves temporary triggers / settings
    pub triggers: BTreeMap<Trigger, bool>,
    pub trigger_locks: BTreeMap<Trigger, bool>
}

impl LevelData {
    pub async fn save(&self, persistent_level_data: &mut PersistentLevelData, settings: &Settings) {
        let mut stopwatch = Stopwatch::default();
        println!("Saving level score and updating stats...");
        stopwatch.start();
        let playtime = get_time() - self.start_time;
        let player = self.player.as_ref().unwrap();
        let level = self.level.as_ref().unwrap();

        let score = LevelScore::new(
            playtime,
            player.coins,
            player.kills,
            player.total_damage,
            player.total_damage_received
        );

        if persistent_level_data.scores.get(level).is_none() {
            persistent_level_data.scores.insert(level.to_owned(), Vec::new());
        }

        if persistent_level_data.stats.get(level).is_none() {
            persistent_level_data.stats.insert(level.to_owned(), LevelStat::new(level.to_owned()));
        }
        let stats_ref = persistent_level_data.stats.get_mut(level).unwrap();
        let deaths = {
            if *self.triggers.get(&Trigger::GameOver).unwrap_or(&false) {
                1
            } else {
                0
            }
        };
        stats_ref.update(deaths);

        let score_space = persistent_level_data.scores.get_mut(level).unwrap();
        score_space.push(score);

        persistent_level_data.save(settings).await;

        stopwatch.stop();
        println!("Saved level level score and updated stats! Took {}ms", stopwatch.elapsed().as_millis());
    }
}

/// Holds all data a level can possibly have
pub struct LevelSceneData {
    pub level_data: LevelData,
    pub world: World,
}

impl LevelSceneData {
    pub async fn empty() -> Self {
        let level_data = LevelData {
            start_time: 0.0,

            level: None,
            player: None,
            platforms: Vec::new(),
            collectibles: Vec::new(),
            enemies: Vec::new(),
            projectiles: Vec::new(),
            power_ups: Vec::new(),
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
    pub collectible_type: CollectibleType,
    pub collected: bool,
    pub collider: Collider,
    pub texture_key: TextureKey,
    pub animation: Animation,
    pub size: Vec2,
    pub speed: Vec2,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CollectibleType {
    Coin,
}

impl Collectible {
    pub async fn new(collectible_type: CollectibleType, pos: Vec2, size: Vec2, texture_key: TextureKey, animation: Animation, speed: Vec2) -> Self {
        let collected = false;
        let collider = Collider::new_collectible(pos, size.x, size.y, vec2(0.0, 0.0)).await;

        Self {
            collectible_type,
            collected,
            collider,
            texture_key,
            animation,
            size,
            speed,
        }
    }

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
            AnimationType::Cycle(_, _, _) => {
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
        self.pos += self.speed * get_frame_time();
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