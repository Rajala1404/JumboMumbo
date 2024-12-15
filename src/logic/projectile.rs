use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_texture_ex, get_frame_time, get_time, DrawTextureParams, Texture2D};
use std::collections::BTreeMap;
use macroquad::color::WHITE;
use crate::logic::collider::Collider;
use crate::logic::level::LevelData;
use crate::utils::enums::TextureKey;

#[derive(PartialEq, Clone, Debug)]
pub struct Projectile {
    pub active: bool,
    pub deletable: bool,
    pub pos: Vec2,
    pub size: Vec2,
    pub start_time: f64,
    pub max_time: f64,
    pub collider: Collider,
    pub damage: i16,
    pub texture_key: TextureKey,
    pub origin: ProjectileOrigin,
    pub speed: Vec2,
}

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Debug)]
pub enum ProjectileOrigin {
    Player,
    Canon
}

impl Projectile {
    pub async fn new(pos: Vec2, size: Vec2, damage: i16, max_time: f64, texture_key: TextureKey, origin: ProjectileOrigin, speed: Vec2) -> Self {
        let start_time = get_time();
        let collider = Collider::new_projectile(pos, size.x, size.y, vec2(0.0, 0.0)).await;

        Self {
            active: true,
            deletable: false,
            pos,
            size,
            start_time,
            max_time,
            collider,
            damage,
            texture_key,
            origin,
            speed,
        }
    }

    pub async fn tick(&mut self, level_data: &LevelData) {
        let colliding_with_platform = self.collider.collide_check_platform(&level_data.platforms, vec2(0.0, 0.0)).await.is_empty();
        let colliding_with_enemy = self.collider.collide_check_enemy(&level_data.enemies, vec2(0.0, 0.0)).await.is_empty();
        let colliding_with_player = if self.origin != ProjectileOrigin::Player { self.collider.touching_player(level_data.player.as_ref().unwrap()).await } else { false };

        let colliding = !colliding_with_platform || !colliding_with_enemy || colliding_with_player;

        if colliding || self.start_time + self.max_time < get_time() {
            self.active = false;
            self.deletable = true;
        } else {
            self.perform_move().await;
        }
    }

    async fn perform_move(&mut self) {
        self.pos += self.speed * get_frame_time();
        self.collider.change_pos(self.pos).await;
    }

    // TODO: Implement Animation
    pub async fn render(&self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>) {
        draw_texture_ex(
            &textures.get(&self.texture_key).unwrap().get(0).unwrap(), self.pos.x, self.pos.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.size.x, self.size.y)),
                ..Default::default()
            },
        );
    }
}