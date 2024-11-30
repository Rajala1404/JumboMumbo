use macroquad::math::Vec2;
use macroquad::prelude::vec2;
use crate::logic::player::Player;
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
    Trigger,
}

impl Collider {
    pub async fn new_actor(pos: Vec2, width: f32, height: f32) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, collider_type: ColliderType::Actor}
    }

    pub async fn new_solid(pos: Vec2, width: f32, height: f32) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, collider_type: ColliderType::Solid}
    }

    pub async fn new_collectible(pos: Vec2, width: f32, height: f32) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, collider_type: ColliderType::Collectible}
    }

    pub async fn new_trigger(pos: Vec2, width: f32, height: f32) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, collider_type: ColliderType::Trigger}
    }

    /// Checks if the collider gets touched by the player
    /// This means if the Players [Collider] is inside the collider of [Self]
    pub async fn touched_by_player(&self, player: &Player) -> bool {
        let player_rect = player.collider_new.rect;
        self.rect.overlaps_with(&player_rect).await
    }

    pub async fn pos(&self) -> Vec2 {
        vec2(self.rect.x, self.rect.y)
    }

    pub async fn change_pos(&mut self, new_pos: Vec2) {
        (self.rect.x, self.rect.y) = (new_pos.x, new_pos.y);
    }
}