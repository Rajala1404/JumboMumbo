use macroquad::math::i32;
use crate::scenes::levels::structs;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum SceneTextureKey {
    Level0,
}

/// All textures
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum TextureKey {
    Player,

    // Platforms
    Platform0,
}

pub enum Scene {
    MainMenu,
    SettingsMenu,
    /// The i32 is the Page
    LevelSelector(i32),
    Level(structs::Level)
}

#[derive(Copy, Clone, PartialEq)]
pub struct Animation {
    animation_type: AnimationType,
    /// The relative floating position of the current collectible
    pub pos_f: f32
}

impl Animation {
    pub fn animate(&mut self) {

    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum AnimationType {
    /// Floating animation that creates a floating effect by letting objects move up and down by the defined px
    Floating(f32),
}