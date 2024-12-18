use std::collections::BTreeMap;
use macroquad::color::WHITE;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};
use macroquad::time::get_time;
use macroquad_platformer::{Solid, World};
use crate::logic::collider::Collider;
use crate::logic::projectile::{Projectile, ProjectileOrigin};
use crate::utils::enums::{Direction, TextureKey};

#[derive(Clone, Debug)]
pub struct Cannon {
    pub pos: Vec2,
    pub size: Vec2,
    /// The shooting speed of the [Cannon]
    pub speed: f64,
    pub direction: Direction,
    pub last_shoot: f64,
    /// The projectile speed
    pub projectile_speed: f32,
    pub projectile_time: f64,
    pub collider: Collider,
    pub _world_collider: Solid,
    pub texture_key: TextureKey,
    pub projectile_texture_key: TextureKey,
    pub damage: i16
}

impl Cannon {
    pub async fn new(pos: Vec2, size: Vec2, speed: f64, offset: f64, direction: Direction, projectile_speed: f32, projectile_time: f64, texture_key: TextureKey, projectile_texture_key: TextureKey, damage: i16, world: &mut World) -> Self {
        let last_shoot = get_time() + offset;
        let collider = Collider::new_solid(pos, size.x, size.y, vec2(0.0, 0.0)).await;
        let _world_collider = world.add_solid(pos, size.x as i32, size.y as i32);

        Self { pos, size, speed, direction, last_shoot, projectile_speed, projectile_time, collider, _world_collider, texture_key, projectile_texture_key, damage }
    }

    pub async fn tick(&mut self, projectiles: &mut Vec<Projectile>) {
        if self.last_shoot + self.speed < get_time() {
            let size = self.size / 2.0;
            let pos = self.pos + self.size / 2.0 - size / 2.0;

            let movement_vector = {
                match self.direction {
                    Direction::Right => vec2(1.0, 0.0) * self.projectile_speed,
                    Direction::Left => vec2(-1.0, 0.0) * self.projectile_speed,
                    Direction::Up => vec2(0.0, -1.0) * self.projectile_speed,
                    Direction::Down => vec2(0.0, 1.0) * self.projectile_speed,
                }
            };

            let projectile  = Projectile::new(
                pos,
                size,
                self.damage,
                self.projectile_time,
                self.projectile_texture_key,
                ProjectileOrigin::Canon,
                movement_vector
            ).await;

            projectiles.push(projectile);
            self.last_shoot = get_time();
        }
    }

    pub async fn render(&self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>) {
        let texture_index = match self.direction {
            Direction::Right => 0,
            Direction::Left => 1,
            Direction::Up => 2,
            Direction::Down => 3,
        };
        let texture = textures.get(&self.texture_key).unwrap().get(texture_index).unwrap();
        draw_texture_ex(
            &texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams{
                dest_size: Some(self.size),
                ..Default::default()
            }
        )
    }
}