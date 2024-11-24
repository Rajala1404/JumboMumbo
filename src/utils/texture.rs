use macroquad::miniquad::FilterMode;
use macroquad::texture::{load_image, Texture2D};
use serde::{Deserialize, Serialize};
use crate::utils::enums::TextureKey;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn to_macro_rect(&self) -> macroquad::math::Rect {
        macroquad::math::Rect::new(
            self.x,
            self.y,
            self.w,
            self.h,
        )
    }
}

/// Loads a texture from a texture map by using a predefined json to get the position of the wanted texture <br>
/// `path` is the path of the texture without a file extension <br>
/// `position` is the position that is inside the json <br>
/// The json must have the same name as the texture and only PNG textures are supported
pub async fn load_textures_from_tile_map(path: String) -> Vec<Texture2D> {
    println!("Loading textures from '{}'", path);
    #[derive(Deserialize)]
    struct JsonMap {
        positions: Vec<Rect>
    }

    let json_path = format!("{}.json", path);
    let image_path = format!("{}.png", path);

    let map: JsonMap = serde_json::from_reader(std::fs::File::open(json_path).expect("Cannot open texture tile map (JSON)")).expect("Cannot parse tile map");
    let image = load_image(&image_path).await.expect("Cannot load texture tile map (PNG)");

    let mut result = vec![];

    for position in map.positions {
        let img = image.sub_image(position.to_macro_rect());
        let texture = Texture2D::from_image(&img);
        texture.set_filter(FilterMode::Nearest);
        result.push(texture);
    }

    result
}

/// Returns the path of the provided [TextureKey] of Platform (without extension)
pub async fn get_platform_path(key: TextureKey) -> String {
    match key {
        TextureKey::Platform0 => {
            String::from("./res/textures/platforms/platform_0")
        }
        _ => {
            unimplemented!()
        }
    }
}