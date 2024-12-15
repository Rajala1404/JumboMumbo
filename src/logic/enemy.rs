use macroquad::math::{vec2, Vec2};
use macroquad_platformer::{Actor, World};
use std::collections::BTreeMap;
use macroquad::prelude::{draw_texture_ex, get_frame_time, DrawTextureParams, Texture2D};
use macroquad::color::{Color, GREEN, RED, WHITE};
use macroquad::shapes::draw_rectangle;
use macroquad::time::get_time;
use crate::logic::collider::Collider;
use crate::logic::player::Player;
use crate::scenes::levels::structs::{Projectile, ProjectileOrigin};
use crate::utils::enums::{Direction, TextureKey};
use crate::utils::mathemann::{plus_minus_range, stretch_float_to};
use crate::utils::structs::{Matrix, Settings};

#[derive(PartialEq, Clone, Debug)]
pub struct Enemy {
    pub size: Vec2,
    pub deletable: bool,
    pub health: i16,
    pub texture_key: TextureKey,
    pub pos: Vec2,
    pub start_pos: Vec2,
    /// The value of damage the player receives if the enemy does damage
    pub damage: i16,
    pub colliders: Matrix<Collider>,
    pub world_collider: Actor,
    pub state: EnemyState,
    pub waiters: BTreeMap<EnemyWaiter, bool>,
    pub waiters_exec: BTreeMap<EnemyWaiter, f64>,
    pub behavior: Vec<EnemyBehavior>,
    pub speed: Vec2,
    pub color: Color
}

#[derive(PartialEq, Clone, Debug)]
pub enum EnemyState {
    Idling,
    Attacking,

}

/// Contains random things that an enemy may do or is needed to execute some bs
#[derive(PartialEq, Clone, Ord, Eq, PartialOrd, Debug)]
pub enum EnemyWaiter {
    /// `true` = Right
    /// `false`= Left
    IdlingDirection,
    Jumping,
    DamageCooldown,
    DamageOverlay,
}

#[derive(PartialEq, Clone, Ord, Eq, PartialOrd, Debug)]
pub enum EnemyBehavior {
    Move(Direction)
}

impl Enemy {
    pub async fn new(pos: Vec2, damage: i16, world: &mut World, size: Vec2, texture_key: TextureKey) -> Self {
        let width = size.x;
        let height = size.y;

        let colliders = {
            let mut result = Matrix::new();

            // Insert Enemy collider
            result.insert(0, 0, Collider::new_enemy(pos, width, height, vec2(0.0, 0.0)).await);

            // Insert collider that go around
            for row in -4..5 {
                for col in -3..3 {
                    if result.get(row, col).is_none() {
                        result.insert(row, col, Collider::new_trigger(vec2(size.x * row as f32, size.y * col as f32), size.x, size.y, vec2(size.x * row as f32, size.y * col as f32)).await)
                    }
                }
            }

            result
        };

        Self {
            size,
            health: 1000,
            deletable: false,
            texture_key,
            pos: pos + vec2(1.0, 0.0),
            start_pos: pos,
            damage,
            colliders,
            world_collider: world.add_actor(pos, width as i32, height as i32),
            state: EnemyState::Idling,
            behavior: Vec::new(),
            waiters: BTreeMap::new(),
            waiters_exec: BTreeMap::new(),
            speed: vec2(0.0, 0.0),
            color: WHITE,
        }
    }

    pub async fn tick(&mut self, world: &mut World, player: &mut Player, projectiles: &Vec<Projectile>, settings: &Settings) {

        // The same as for the player
        // SP Start
        let pos = world.actor_pos(self.world_collider);
        self.pos = pos;
        let on_ground = world.collide_check(self.world_collider, pos + vec2(0.0, 1.0));
        let sealing_hit = world.collide_check(self.world_collider, pos + vec2(0.0, -1.0));

        if sealing_hit {
            self.speed.y = (100.0 * settings.gui_scale) * get_frame_time(); // I have no idea why but if this doesn't get multiplied by the frame time it's inconsistent on different Frame Rates
        }

        if !on_ground {
            self.speed.y += (4800.0 * settings.gui_scale) * get_frame_time();
        } else {
            self.waiters.remove(&EnemyWaiter::Jumping);
            self.speed.y = 0.0;
        }
        // SP End

        // End Damage cooldown
        if *self.waiters.get(&EnemyWaiter::DamageCooldown).unwrap_or(&false) {
            if self.waiters_exec.get(&EnemyWaiter::DamageCooldown).unwrap_or(&0.0) + 0.5 < get_time() {
                self.waiters.remove(&EnemyWaiter::DamageCooldown);
                self.waiters_exec.remove(&EnemyWaiter::DamageCooldown);
            }
        }

        // End damage overlay
        if *self.waiters.get(&EnemyWaiter::DamageOverlay).unwrap_or(&false) {
            self.color = RED;
            if self.waiters_exec.get(&EnemyWaiter::DamageOverlay).unwrap_or(&0.0) + 0.25 < get_time() {
                self.color = WHITE;
                self.waiters.remove(&EnemyWaiter::DamageOverlay);
                self.waiters_exec.remove(&EnemyWaiter::DamageOverlay);
            }
        }
        
        let colliding_projectiles = self.colliders.get(0, 0).unwrap().collide_check_projectile(projectiles, vec2(0.0, 0.0)).await;
        if !colliding_projectiles.is_empty() {
            for projectile in colliding_projectiles {
                let projectile = projectiles.get(projectile);
                if projectile.is_some() {
                    if projectile.unwrap().origin == ProjectileOrigin::Player {
                        if !self.waiters.get(&EnemyWaiter::DamageCooldown).unwrap_or(&false) {
                            self.health += projectile.unwrap().damage;
                            if self.health < 0 {
                                self.health = 0;
                            }

                            if self.health == 0 {
                                player.kills += 1;
                                self.deletable = true;
                            }

                            player.total_damage += -projectile.unwrap().damage as u32;

                            self.waiters.insert(EnemyWaiter::DamageOverlay, true);
                            self.waiters_exec.insert(EnemyWaiter::DamageOverlay, get_time());
                            self.waiters.insert(EnemyWaiter::DamageCooldown, true);
                            self.waiters_exec.insert(EnemyWaiter::DamageCooldown, get_time());
                        }
                    }
                }
            }
        }

        // DI (Dumb intelligence)
        match self.state {
            EnemyState::Attacking => {
                // Jump if colliding with a wall
                if self.is_touching_wall(world).await {
                    self.behavior.push(EnemyBehavior::Move(Direction::Up));
                    self.waiters.insert(EnemyWaiter::Jumping, true);
                }

                for ((row, col), collider) in &self.colliders {
                    // I only care if player is above me
                    if !(col >= &-1) && ((row < &-1  && !(row > &1))|| (row > &1 && !(row < &-1))) && collider.touching_player(player).await {
                        self.behavior.push(EnemyBehavior::Move(Direction::Up))
                    }
                }

                let touched_right = {
                    let mut result = false;
                    for ((row, col), collider) in &self.colliders {
                        if row <= &0 { continue; }
                        if collider.touching_player(player).await {
                            if !self.tile_visible(world, row, col).await {
                                continue;
                            }
                            result = true;
                            break;
                        }
                    }
                    result
                };

                let touched_left = {
                    let mut result = false;
                    for ((row, col), collider) in &self.colliders {
                        if row >= &0 { continue; }
                        if collider.touching_player(player).await {
                            if !self.tile_visible(world, row, col).await {
                                continue;
                            }
                            result = true;
                            break;
                        }
                    }
                    result
                };

                if self.colliders.get(0, 0).unwrap().touching_player(player).await {

                } else if touched_right {
                    self.behavior.push(EnemyBehavior::Move(Direction::Right));
                    self.waiters.insert(EnemyWaiter::IdlingDirection, true);
                } else if touched_left {
                    self.behavior.push(EnemyBehavior::Move(Direction::Left));
                    self.waiters.insert(EnemyWaiter::IdlingDirection, false);
                } else {
                    self.state = EnemyState::Idling;
                }
            },
            EnemyState::Idling => {
                let touched = {
                    let mut result = false;
                    for ((row, col), collider) in &self.colliders {
                        if (row < &-3  && !(row > &3)) || (row > &3 && !(row < &-3)) { continue; }
                        if collider.touching_player(player).await {
                            if !self.tile_visible(world, row, col).await {
                                continue;
                            }
                            result = true;
                            break;
                        }
                    }
                    result
                };

                if touched {
                    self.state = EnemyState::Attacking;
                    self.behavior.clear();
                } else {
                    // Jump if colliding with a wall
                    if self.is_touching_wall(world).await {
                        self.behavior.push(EnemyBehavior::Move(Direction::Up));
                        self.waiters.insert(EnemyWaiter::Jumping, true);
                    }

                    if *self.waiters.get(&EnemyWaiter::IdlingDirection).unwrap_or(&true) {
                        // Why the fuck does this function check so wierd
                        if world.collide_check(self.world_collider, pos + vec2(self.size.x + 1.0, 1.0)) || *self.waiters.get(&EnemyWaiter::Jumping).unwrap_or(&false) {
                            self.waiters.insert(EnemyWaiter::IdlingDirection, true);
                            self.behavior.push(EnemyBehavior::Move(Direction::Right));
                        } else {
                            self.waiters.insert(EnemyWaiter::IdlingDirection, false);
                        }
                    } else {
                        // Same here
                        if world.collide_check(self.world_collider, pos + vec2(-self.size.x - 1.0, 1.0)) || *self.waiters.get(&EnemyWaiter::Jumping).unwrap_or(&false) {
                            self.waiters.insert(EnemyWaiter::IdlingDirection, false);
                            self.behavior.push(EnemyBehavior::Move(Direction::Left));
                        } else {
                            self.waiters.insert(EnemyWaiter::IdlingDirection, true);
                        }
                    }
                }
            },
        }
        self.speed.x = 0.0;
        for behavior in &self.behavior {
            match behavior {
                EnemyBehavior::Move(direction) => {
                    match direction {
                        Direction::Right => {
                            self.speed.x = 700.0 * settings.gui_scale;
                        }
                        Direction::Left => {
                            self.speed.x = -700.0 * settings.gui_scale;
                        }
                        Direction::Up => {
                            if on_ground {
                                self.speed.y = 1900.0 * -settings.gui_scale;
                            }
                        }
                        _ => unimplemented!()
                    }
                }
            }
        }
        self.behavior.clear();

        // Set positions using the previously defined speeds
        world.move_h(self.world_collider, self.speed.x * get_frame_time());
        world.move_v(self.world_collider, self.speed.y * get_frame_time());

        let pos = world.actor_pos(self.world_collider);
        self.pos = pos;
        self.update_pos().await;
    }

    async fn tile_visible(&self, world: &World, row: &i32, col: &i32) -> bool {
        let col_range = plus_minus_range(*col, 0);
        let row_range = plus_minus_range(*row, 0);


        for row in row_range.await {
            let collider_pos = self.colliders.get(row, *col).unwrap().pos().await;
            if world.collide_check(self.world_collider, collider_pos) {
                return false
            }
        }

        for col in col_range.await {
            let collider_pos = self.colliders.get(*row, col).unwrap().pos().await;
            if world.collide_check(self.world_collider, collider_pos) {
                return false
            }
        }

        true
    }

    async fn update_pos(&mut self) {
        //self.trigger_left.change_pos(self.pos - vec2(self.trigger_left.rect.w, -self.trigger_left.offset.y)).await;
        //self.trigger_right.change_pos(self.pos + vec2(self.size.x, self.trigger_right.offset.y)).await;
        //self.collider.change_pos(self.pos).await;
        for (_, collider) in &mut self.colliders {
            collider.change_pos(self.pos + collider.offset).await
        }
    }

    async fn is_touching_wall(&self, world: &World) -> bool {
        world.collide_check(self.world_collider, self.pos + vec2(-1.0, 0.0)) || world.collide_check(self.world_collider, self.pos + vec2(1.0, 0.0))
    }

    pub async fn render(&self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>, settings: &Settings) {
        let texture = textures.get(&self.texture_key).unwrap().get(0).unwrap();
        draw_texture_ex(
            texture,
            self.pos.x,
            self.pos.y,
            self.color,
            DrawTextureParams {
                dest_size: Some(self.size),
                ..Default::default()
            },
        );

        // Render health bar if health below 100%
        if self.health < 1000 {
            let c_width = self.colliders.get(0, 0).unwrap().rect.w;
            let spacing = c_width / 16.0;
            let width = stretch_float_to(self.health as f32, 1000.0, c_width - spacing * 2.0).await;
            let width_full = stretch_float_to(1000.0, 1000.0, c_width - spacing * 2.0).await;
            let height = 16.0 * settings.gui_scale;
            let pos = vec2(self.pos.x + spacing, self.pos.y - height * 2.0);

            // Full health (start health)
            draw_rectangle(pos.x, pos.y, width_full, height, RED);
            // Actual health
            draw_rectangle(pos.x, pos.y, width, height, GREEN);
        }
    }
}