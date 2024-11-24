use macroquad::math::i32;
use crate::scenes::levels::levels;

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
    Level(levels::Level)
}