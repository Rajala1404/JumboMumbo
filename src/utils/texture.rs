use std::collections::BTreeMap;
use macroquad::color::Color;
use macroquad::miniquad::FilterMode;
use macroquad::texture::{load_image, Image, Texture2D};
use serde::Deserialize;
use stopwatch2::Stopwatch;
use crate::utils::enums::TextureKey;
use crate::utils::loading::show_loading_screen;
use crate::utils::mathemann::stretch_float_to;
use crate::utils::structs::{Matrix, Rect};

/// Loads a texture from a texture map by using a predefined json to get the position of the wanted texture <br>
/// `path` is the path of the texture without a file extension <br>
/// `position` is the position that is inside the json <br>
/// The json must have the same name as the texture and only PNG textures are supported
pub async fn load_textures_from_tile_map(path: String) -> Vec<Texture2D> {
    println!("Loading texture tile map from '{}'", path);
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
        let img = image.sub_image(position.to_macro_rect().await);
        let texture = Texture2D::from_image(&img);
        texture.set_filter(FilterMode::Nearest);
        result.push(texture);
    }

    result
}

/// Returns the path of the provided [TextureKey] of a Texture (without extension)
pub async fn get_texture_path(key: TextureKey) -> String {
    match key {
        TextureKey::Player => String::from("./res/textures/entities/player"),
        TextureKey::Enemy0 => String::from("./res/textures/entities/enemy_0"),
        TextureKey::Projectile0 => String::from("./res/textures/entities/projectile_0"),
        TextureKey::Platform0 => String::from("./res/textures/platforms/platform_0"),
        TextureKey::Coin0 => String::from("./res/textures/items/coin_0"),
        TextureKey::PowerUps0 => String::from("./res/textures/items/power_ups_0"),
        TextureKey::Icons0 => String::from("./res/ui/icons_0"),
        TextureKey::Cannon0 => String::from("./res/textures/blocks/cannon_0"),
        TextureKey::Button0 => String::from("./res/ui/button_0"),
    }
}

impl From<Image> for Matrix<Color> {
    fn from(image: Image) -> Self {
        let mut result = Matrix::new();
        let width = image.width() as i32;
        let height = image.height() as i32;

        for i in 0..width {
            let row = width - i - 1;
            for j in 0..height {
                let colum = height - j - 1;
                let pixel = image.get_pixel(row as u32, colum as u32);
                result.insert(-i, -j, pixel);
            }
        }

        result
    }
}

pub async fn load_level_textures(name: &str, keys: Vec<TextureKey>) -> BTreeMap<TextureKey, Vec<Texture2D>> {
    let mut stopwatch = Stopwatch::default();
    let mut result = BTreeMap::new();

    println!("Loading textures for {}...", name);
    stopwatch.start();

    let total_keys = keys.len();

    let mut previous_index = 0;
    let previous_progress = stretch_float_to(previous_index as f32, total_keys as f32 + 2.0, 100.0).await;
    let progress = stretch_float_to(previous_progress, total_keys as f32 + 2.0, 100.0).await;
    show_loading_screen(previous_progress, progress, name).await;
    previous_index += 1;

    for key  in keys {
        let path = get_texture_path(key.to_owned()).await;
        result.insert(key.to_owned(), load_textures_from_tile_map(path).await);

        // Update Loading Screen
        let previous_progress = stretch_float_to(previous_index as f32, total_keys as f32 + 2.0, 100.0).await;
        previous_index += 1;
        let progress = stretch_float_to(previous_index as f32, total_keys as f32 + 2.0, 100.0).await;
        show_loading_screen(previous_progress, progress, name).await;
    }

    stopwatch.stop();
    println!("Loaded textures for {}! Took: {}ms", name, stopwatch.elapsed().as_millis());

    result
}