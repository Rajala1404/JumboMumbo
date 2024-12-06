use serde::{Deserialize, Serialize};

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