use macroquad::color::{BLACK, GREEN, ORANGE, RED, YELLOW};
use macroquad::math::Vec2;
use macroquad::prelude::vec2;
use macroquad::shapes::draw_rectangle_lines;
use crate::logic::player::Player;
use crate::scenes::levels::structs::Enemy;
use crate::utils::structs::Settings;
use crate::utils::structs::Rect;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Collider {
    pub rect: Rect,
    pub collider_type: ColliderType
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ColliderType {
    Actor,
    Enemy,
    Solid,
    Collectible,
    Trigger,
}

impl Collider {
    pub async fn new_actor(pos: Vec2, width: f32, height: f32) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, collider_type: ColliderType::Actor}
    }
    pub async fn new_enemy(pos: Vec2, width: f32, height: f32) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, collider_type: ColliderType::Enemy}
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
    pub async fn touching_player(&self, player: &Player) -> bool {
        let player_rect = player.collider_new.rect;
        self.rect.overlaps(&player_rect).await
    }

    /// This functions checks if an enemy of the provided Vector collides with the Player on the relative position arguments <br>
    /// The position is relative to the top left corner of the collider <br>
    /// The returned [Vec<usize>] contains the index of each enemy that collides
    pub async fn collide_check_enemy(&self, enemies: &Vec<Enemy>, pos: Vec2) -> Vec<usize> {
        let mut result = Vec::new();
        let rect = {
            let mut result = self.rect;
            // Shift positions of Rectangle
            result.x += pos.x;
            result.y += pos.y;

            result
        };

        for (i, enemy) in enemies.iter().enumerate() {
            if rect.overlaps(&enemy.collider.rect).await {
                result.push(i)
            }
        }

        result
    }

    pub async fn pos(&self) -> Vec2 {
        vec2(self.rect.x, self.rect.y)
    }

    pub async fn change_pos(&mut self, new_pos: Vec2) {
        (self.rect.x, self.rect.y) = (new_pos.x, new_pos.y);
    }

    pub async fn debug_render(&self, settings: &Settings) {
        let color = {
            match self.collider_type {
                ColliderType::Actor => {
                    GREEN
                },
                ColliderType::Enemy => {
                    RED
                },
                ColliderType::Solid => {
                    BLACK
                }
                ColliderType::Collectible => {
                    YELLOW
                }
                ColliderType::Trigger => {
                    ORANGE
                }
            }
        };

        draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 10.0 * settings.gui_scale, color);
    }
}