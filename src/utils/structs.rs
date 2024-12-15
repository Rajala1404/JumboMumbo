use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use macroquad::math::f32;
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

impl Settings {
    pub async fn new(path: String, level_data_path: String) -> Settings {
        Settings {
            path,
            level_data_path,
            gui_scale: 1.0,
        }
    }

    pub async fn save(&self) {
        let mut file = fs::File::create(&self.path).unwrap();

        let s_settings = serde_json::to_string_pretty(&self).expect("Couldn't serialize settings");
        file.write_all(s_settings.as_bytes()).expect("Couldn't write settings file");
    }
}

/// Temporary Saves settings before they get applied
#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct TempSettings {
    pub settings: Settings,
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