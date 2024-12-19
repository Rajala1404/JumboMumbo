use macroquad::input::{is_key_pressed, KeyCode};
use crate::utils::enums::Scene;

pub async fn credits(scene: &mut Scene) {
    if is_key_pressed(KeyCode::Escape) {
        *scene = Scene::MainMenu;
        return;
    }
}