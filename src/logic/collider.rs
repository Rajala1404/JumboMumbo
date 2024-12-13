use macroquad::color::{GREEN, ORANGE, RED, VIOLET, WHITE, YELLOW};
use macroquad::math::Vec2;
use macroquad::prelude::vec2;
use macroquad::shapes::draw_rectangle_lines;
use crate::logic::player::Player;
use crate::logic::enemy::Enemy;
use crate::scenes::levels::structs::{Platform, Projectile};
use crate::utils::structs::Settings;
use crate::utils::structs::Rect;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Collider {
    pub rect: Rect,
    pub offset: Vec2,
    pub collider_type: ColliderType
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ColliderType {
    Actor,
    Enemy,
    Projectile,
    Solid,
    Collectible,
    Trigger,
}

impl Collider {
    pub async fn new_actor(pos: Vec2, width: f32, height: f32, offset: Vec2) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, offset, collider_type: ColliderType::Actor}
    }
    pub async fn new_enemy(pos: Vec2, width: f32, height: f32, offset: Vec2) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, offset, collider_type: ColliderType::Enemy}
    }

    pub async fn new_projectile(pos: Vec2, width: f32, height: f32, offset: Vec2) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, offset, collider_type: ColliderType::Projectile}
    }

    pub async fn new_solid(pos: Vec2, width: f32, height: f32, offset: Vec2) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, offset, collider_type: ColliderType::Solid}
    }

    pub async fn new_collectible(pos: Vec2, width: f32, height: f32, offset: Vec2) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, offset, collider_type: ColliderType::Collectible}
    }

    pub async fn new_trigger(pos: Vec2, width: f32, height: f32, offset: Vec2) -> Self {
        let rect = Rect::new(pos.x, pos.y, width, height).await;
        Self { rect, offset, collider_type: ColliderType::Trigger}
    }

    /// Checks if the collider gets touched by the player
    /// This means if the Players [Collider] is inside the collider of [Self]
    pub async fn touching_player(&self, player: &Player) -> bool {
        let player_rect = player.collider_new.rect;
        self.rect.overlaps(&player_rect).await
    }

    /// This functions checks if an enemy of the provided Vector collides with the [Collider] on the relative position arguments <br>
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
            if rect.overlaps(&enemy.colliders.get(0, 0).unwrap().rect).await {
                result.push(i)
            }
        }

        result
    }

    pub async fn collide_check_projectile(&self, projectiles: &Vec<Projectile>, pos: Vec2) -> Vec<usize> {
        let mut result = Vec::new();
        let rect = {
            let mut result = self.rect;
            // Shift positions of Rectangle
            result.x += pos.x;
            result.y += pos.y;

            result
        };

        for (i, projectile) in projectiles.iter().enumerate() {
            if rect.overlaps(&projectile.collider.rect).await {
                result.push(i)
            }
        }

        result
    }

    pub async fn collide_check_platform(&self, platforms: &Vec<Platform>, pos: Vec2) -> Vec<usize> {
        let mut result = Vec::new();
        let rect = {
            let mut result = self.rect;
            // Shift positions of Rectangle
            result.x += pos.x;
            result.y += pos.y;

            result
        };

        for (i, platforms) in platforms.iter().enumerate() {
            if rect.overlaps(&platforms.collider_new.rect).await {
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
                ColliderType::Projectile => {
                    VIOLET
                }
                ColliderType::Solid => {
                    WHITE
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