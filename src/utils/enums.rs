use macroquad::time::get_time;
use crate::scenes::levels::structs;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum SceneTextureKey {
    Level0,
}

/// All textures
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TextureKey {
    Player,

    // Platforms
    Platform0,

    // Collectibles
    /// This texture needs to be animated
    Coin0,
}

pub enum Scene {
    MainMenu,
    SettingsMenu,
    /// The i32 is the Page
    LevelSelector(i32),
    Level(structs::Level)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Animation {
    pub animation_type: AnimationType,
    pub last_time: f64,
    /// Contains the current index or frame of the animation (should be -1 at first)
    pub index: i32,
}

impl Animation {
    pub fn new(animation_type: AnimationType) -> Self {
        Self {animation_type, last_time: get_time(), index: -1}
    }

    /// Executes the current animation <b>
    /// Depending on what animation you are trying to animate you may need to do some steps manually <br>
    /// For more information about what to do please refer to the documentation of the chosen animation
    pub async fn animate(&mut self) {
        match self.animation_type {
            AnimationType::Cycle(start, end) => {
                // Set index to start (if not already done)
                if self.index == -1 {
                    self.index = start as i32 - 1
                }

                if self.last_time < get_time() - 0.1 {

                    // Reset index if above max
                    if self.index < end as i32 {
                        self.index += 1;
                    } else {
                        self.index = start as i32
                    }

                    self.last_time = get_time();
                }

                // Set index to start (if not already done)
                if self.index == -1 {
                    self.index = start as i32
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AnimationType {
    /// Goes through a fixed number of textures <br>
    /// For this animation the index represents the current texture index. <br>
    /// **This animation needs to be rendered manually** <br>
    /// The first [u32] represents the start and the last the end
    Cycle(u32, u32)
}

#[derive(Copy, Clone, PartialEq, Ord, Eq, PartialOrd, Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}