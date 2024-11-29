use macroquad::math::Vec2;
use crate::scenes::levels::structs::LevelSceneData;
use crate::utils::structs::Rect;

#[derive(PartialEq, Copy, Clone)]
pub struct Collider {
    pub rect: Rect,
    pub collider_type: ColliderType
}

#[derive(PartialEq, Copy, Clone)]
pub enum ColliderType {
    Actor,
    Solid,
    Collectible,
}

impl Collider {
    pub async fn new_actor(pos: Vec2, width: f32, height: f32) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height);
        Self { rect, collider_type: ColliderType::Actor}
    }

    pub async fn new_solid(pos: Vec2, width: f32, height: f32) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height);
        Self { rect, collider_type: ColliderType::Solid}
    }

    pub async fn new_collectible(pos: Vec2, width: f32, height: f32) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height);
        Self { rect, collider_type: ColliderType::Collectible}
    }

    /// Checks if the collider gets touched by the player
    /// This means if the Players [Collider] is inside the collider of [Self]
    pub async fn touched_by_player(&self, level_scene_data: &LevelSceneData) -> bool {
        unimplemented!()
    }
}