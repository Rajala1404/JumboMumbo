use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use macroquad::math::f32;
use crate::scenes::levels::structs::Level;

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