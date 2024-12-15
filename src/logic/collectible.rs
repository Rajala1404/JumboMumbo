use macroquad::math::{vec2, Vec2};
use std::collections::BTreeMap;
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};
use macroquad::color::WHITE;
use crate::logic::collider::Collider;
use crate::logic::player::Player;
use crate::utils::enums::{Animation, AnimationType, TextureKey};

#[derive(PartialEq, Clone, Debug)]
pub struct Collectible {
    pub collectible_type: CollectibleType,
    pub collected: bool,
    pub collider: Collider,
    pub texture_key: TextureKey,
    pub animation: Animation,
    pub size: Vec2,
    pub speed: Vec2,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CollectibleType {
    Coin,
}

impl Collectible {
    pub async fn new(collectible_type: CollectibleType, pos: Vec2, size: Vec2, texture_key: TextureKey, animation: Animation, speed: Vec2) -> Self {
        let collected = false;
        let collider = Collider::new_collectible(pos, size.x, size.y, vec2(0.0, 0.0)).await;

        Self {
            collectible_type,
            collected,
            collider,
            texture_key,
            animation,
            size,
            speed,
        }
    }

    /// Runs all checks that may get called onto a collectible
    pub async fn check(&mut self, player: &Player) {
        // Check if the collectible collides with another thing
        if self.collider.touching_player(player).await {
            self.collected = true;
        }
    }

    pub async fn render(&mut self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>) {
        let pos = self.collider.pos().await;

        match self.animation.animation_type {
            AnimationType::Cycle(_, _, _) => {
                self.animation.animate().await;
                let texture = textures.get(&self.texture_key).unwrap().get(self.animation.index as usize).unwrap();
                draw_texture_ex(
                    texture,
                    pos.x,
                    pos.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(self.size),
                        ..Default::default()
                    },
                );
            }
        }
    }
}