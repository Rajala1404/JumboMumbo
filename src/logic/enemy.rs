use macroquad::math::{vec2, Vec2};
use macroquad_platformer::{Actor, World};
use std::collections::BTreeMap;
use macroquad::prelude::{draw_texture_ex, get_frame_time, DrawTextureParams, Texture2D};
use macroquad::color::{WHITE};
use crate::logic::collider::Collider;
use crate::logic::player::Player;
use crate::utils::enums::{Direction, TextureKey};
use crate::utils::mathemann::plus_minus_range;
use crate::utils::structs::{Matrix, Settings};

#[derive(PartialEq, Clone, Debug)]
pub struct Enemy {
    pub size: Vec2,
    pub texture_key: TextureKey,
    pub pos: Vec2,
    pub start_pos: Vec2,
    pub colliders: Matrix<Collider>,
    pub world_collider: Actor,
    pub state: EnemyState,
    pub waiters: BTreeMap<EnemyWaiter, bool>,
    pub behavior: Vec<EnemyBehavior>,
    pub speed: Vec2,
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
    IdlingDirection
}

#[derive(PartialEq, Clone, Ord, Eq, PartialOrd, Debug)]
pub enum EnemyBehavior {
    Move(Direction)
}

impl Enemy {
    pub async fn new(pos: Vec2, world: &mut World, size: Vec2, texture_key: TextureKey) -> Self {
        let width = size.x;
        let height = size.y;

        let colliders = {
            let mut result = Matrix::new();

            // Insert Enemy collider
            result.insert(0, 0, Collider::new_enemy(pos, width, height, vec2(0.0, 0.0)).await);

            // Insert collider that go around
            for row in -2..3 {
                for col in -2..3 {
                    if result.get(row, col).is_none() {
                        result.insert(row, col, Collider::new_trigger(vec2(size.x * row as f32, size.y * col as f32), size.x, size.y, vec2(size.x * row as f32, size.y * col as f32)).await)
                    }
                }
            }

            result
        };

        Self {
            size,
            texture_key,
            pos: pos + vec2(1.0, 0.0),
            start_pos: pos,
            colliders,
            world_collider: world.add_actor(pos, width as i32, height as i32),
            state: EnemyState::Idling,
            behavior: Vec::new(),
            waiters: BTreeMap::new(),
            speed: vec2(0.0, 0.0),
        }
    }

    pub async fn tick(&mut self, world: &mut World, player: &mut Player, settings: &Settings) {

        // The same as for the player
        // SP Start
        let pos = world.actor_pos(self.world_collider);
        self.pos = pos;
        let on_ground = world.collide_check(self.world_collider, pos + vec2(0.0, 1.0));
        let sealing_hit = world.collide_check(self.world_collider, pos + vec2(0.0, -1.0));

        if sealing_hit {
            self.speed.y = (100.0 * settings.gui_scale) * get_frame_time()
        }

        if !on_ground {
            self.speed.y += (4800.0 * settings.gui_scale) * get_frame_time();
        } else {
            self.speed.y = 0.0;
        }
        // SP End

        // DI (Dumb intelligence)
        match self.state {
            EnemyState::Attacking => {
                //let touched_right = self.trigger_right.touching_player(player);
                //let touched_left = self.trigger_left.touching_player(player);
//
                //if touched_right.await {
                //    self.behavior.push(EnemyBehavior::Move(Direction::Right));
                //    self.waiters.insert(EnemyWaiter::IdlingDirection, true);
                //} else if touched_left.await {
                //    self.behavior.push(EnemyBehavior::Move(Direction::Left));
                //    self.waiters.insert(EnemyWaiter::IdlingDirection, false);
                //} else {
                //    self.state = EnemyState::Idling
                //}
//
                //if world.collide_check(self.world_collider, pos + vec2(0.0, -self.size.y * 2.0)) {
                //    self.state = EnemyState::Idling
                //}
            },
            EnemyState::Idling => {
                let touched = {
                    let mut result = false;
                    for ((row, col), collider) in &self.colliders {
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
                    if *self.waiters.get(&EnemyWaiter::IdlingDirection).unwrap_or(&true) {
                        // Why the fuck does this function check so wierd
                        if world.collide_check(self.world_collider, pos + vec2(self.size.x + 1.0, 1.0)) {
                            self.waiters.insert(EnemyWaiter::IdlingDirection, true);
                            self.behavior.push(EnemyBehavior::Move(Direction::Right));
                        } else {
                            self.waiters.insert(EnemyWaiter::IdlingDirection, false);
                        }
                    } else {
                        // Same here
                        if world.collide_check(self.world_collider, pos + vec2(-self.size.x - 1.0, 1.0)) {
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
                            self.speed.x = 600.0 * settings.gui_scale;
                        }
                        Direction::Left => {
                            self.speed.x = -600.0 * settings.gui_scale;
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

    pub async fn render(&self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>) {
        let texture = textures.get(&self.texture_key).unwrap().get(0).unwrap();
        draw_texture_ex(
            texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size),
                ..Default::default()
            },
        );
    }
}