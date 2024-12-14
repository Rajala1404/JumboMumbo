use std::collections::BTreeMap;
use macroquad::camera::set_camera;
use macroquad::color::{Color, RED, WHITE};
use macroquad::input::{is_key_down, is_key_pressed, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_texture_ex, get_frame_time, screen_height, Camera2D, DrawTextureParams, Rect, Texture2D};
use macroquad::shapes::draw_rectangle;
use macroquad::time::get_time;
use macroquad::window::screen_width;
use macroquad_platformer::{Actor, World};
use crate::logic::collider::Collider;
use crate::scenes::levels::structs::{LevelData, Projectile, ProjectileOrigin, Trigger};
use crate::utils::structs::Settings;
use crate::utils::enums::{Direction, TextureKey};
use crate::utils::mathemann::point_to_point_direction_with_speed;

// This file contains everything that is for the player

#[derive(PartialEq, Clone)]
pub struct Player {
    pub health: i16,
    pub color: Color,
    pub width: f32,
    pub height: f32,
    /// 0: Left <br>
    /// 1: Right
    pub state: i8,
    pub collider: Actor,
    pub collider_new: Collider,
    pub camera_collider: [Actor; 4],
    pub speed: Vec2,
    /// All triggers and if a Trigger is active or not
    pub triggers: BTreeMap<PlayerTrigger, bool>,
    /// Contains the last time a trigger was executed
    pub triggers_exec: BTreeMap<PlayerTrigger, f64>,
}

#[derive(PartialEq, Eq, Clone, Ord, PartialOrd, Copy)]
pub enum PlayerTrigger {
    Damage,
    ShootTimeout
}

impl Player {
    pub async fn new(width: f32, height: f32, pos: Vec2, state: i8, world: &mut World) -> Self {
        Player {
            health: 1000,
            color: WHITE,
            width,
            height,
            state,
            collider: world.add_actor(pos, width as i32, height as i32),
            collider_new: Collider::new_actor(pos, width, height, vec2(0.0, 0.0)).await,
            camera_collider: [
                // Left
                world.add_actor(vec2(0.0, 0.0), (screen_width() / 4.0) as i32, screen_height() as i32),
                // Right
                world.add_actor(vec2(screen_width() - (screen_width() / 4.0), 0.0), (screen_width() / 4.0) as i32, screen_height() as i32),
                // Up
                world.add_actor(vec2(0.0, 0.0), screen_width() as i32, (screen_height() / 8.0) as i32),
                // Down
                world.add_actor(vec2(0.0,  screen_height() - screen_height() / 8.0), screen_width() as i32, (screen_height() / 8.0) as i32),
            ],
            speed: vec2(0.0, 0.0),
            triggers: BTreeMap::new(),
            triggers_exec: BTreeMap::new(),
        }
    }

    /// This function handles everything regarding the controls of the player (including moving)
    pub async fn control(&mut self, world: &mut World, level_data: &mut LevelData, settings: &Settings) {
        // gets the current position of the player from the world
        let pos = world.actor_pos(self.collider);
        // Checks if the player is on another collider by checking if one collider is 1px beyond him
        let on_ground = world.collide_check(self.collider, pos + vec2(0.0, 1.0));
        // Checks if the player is hitting a sealing by checking if one collider is 1px above him
        let sealing_hit = world.collide_check(self.collider, pos + vec2(0.0, -1.0));

        // If the player is hitting the sealing reset the velocity to 0
        if sealing_hit {
            self.speed.y = (100.0 * settings.gui_scale) * get_frame_time(); // I have no idea why but if this doesn't get multiplied by the frame time it's inconsistent on different Frame Rates
        }

        // If the player is not on the ground change velocity of y to 500 (to simulate gravity)
        if !on_ground {      // multiplies by get_frame_time() so the speed is on all refresh rates the same
            self.speed.y += (4800.0 * settings.gui_scale) * get_frame_time();
        } else {
            self.speed.y = 0.0;
        }

        // 1 = Left, 2 = Right
        let mut direction = 0;

        // Checks if key is currently pressed
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            // If D or Right Arrow is pressed the Player will be moved to the right by increasing the speed on the x-axis
            self.speed.x = 1300.0 * settings.gui_scale;
            self.state = 1;
            direction = 2;
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.speed.x = 1300.0 * -settings.gui_scale;
            self.state = 0;
            direction = 1;
        } else {
            // Resets the speed if nothing is pressed
            self.speed.x = 0.0;
        }

        if is_key_down(KeyCode::Space) {
            if on_ground {
                self.speed.y = 1900.0 * -settings.gui_scale;
            }
        }

        self.perform_move(world).await;
        self.tick(level_data, world, settings).await;

        let pos = world.actor_pos(self.collider);

        { // Make camera follow player
            fn move_camera_collider(collider: Actor, world: &mut World, direction: Direction, trigger: bool, pos: &Vec2, player: &Player) {
                let y = pos.y + player.height - screen_height();

                match direction {
                    Direction::Right => {
                        if trigger {
                            world.set_actor_position(collider, vec2(pos.x + player.width, y));
                        } else {
                            world.set_actor_position(collider, vec2(pos.x + (screen_width() / 2.0), y));
                        }
                    }
                    Direction::Left => {
                        if trigger {
                            world.set_actor_position(collider, vec2(pos.x - screen_width() / 4.0, y));
                        } else {
                            world.set_actor_position(collider, vec2(pos.x + player.width + screen_width() / 4.0 - screen_width(), y));
                        }
                    }
                    Direction::Up => {
                        if trigger {
                            world.set_actor_position(collider, vec2(pos.x, pos.y - screen_height() / 8.0));
                        } else {
                            world.set_actor_position(collider, vec2(pos.x, pos.y + screen_height() / 8.0 + player.height - screen_height()));
                        }
                    }
                    Direction::Down => {
                        if trigger {
                            world.set_actor_position(collider, vec2(pos.x, pos.y + player.height));
                        } else {
                            world.set_actor_position(collider, vec2(pos.x, (pos.y + screen_height() / 8.0) + screen_height() - screen_height() / 2.0 + screen_height() / 8.0));
                        }
                    }
                }
            }

            if pos.x - 1.0 <= world.actor_pos(self.camera_collider[0]).x + screen_width() / 4.0 && direction != 2 {
                let y = world.actor_pos(self.camera_collider[2]);
                set_camera(&Camera2D::from_display_rect(Rect::new(pos.x - screen_width() / 4.0, y.y + screen_height(), screen_width(), -screen_height())));

                move_camera_collider(self.camera_collider[0], world, Direction::Left, true, &pos, self);
                move_camera_collider(self.camera_collider[1], world, Direction::Right, false, &pos, self);
            } else if pos.x + self.height + 1.0 >= world.actor_pos(self.camera_collider[1]).x && direction != 1 {
                let y = world.actor_pos(self.camera_collider[2]);
                set_camera(&Camera2D::from_display_rect(Rect::new(pos.x + self.width - (screen_width() - screen_width() / 4.0), y.y + screen_height(), screen_width(), -screen_height())));

                move_camera_collider(self.camera_collider[0], world, Direction::Left, false, &pos, self);
                move_camera_collider(self.camera_collider[1], world, Direction::Right, true, &pos, self);
            }

            if pos.y -1.0 <= world.actor_pos(self.camera_collider[2]).y + screen_height() / 8.0 && !(pos.y + self.height + 1.0 >= world.actor_pos(self.camera_collider[3]).y) {
                let x = world.actor_pos(self.camera_collider[0]);
                set_camera(&Camera2D::from_display_rect(Rect::new(x.x, pos.y - screen_height() / 8.0 + screen_height(), screen_width(), -screen_height())));
                move_camera_collider(self.camera_collider[2], world, Direction::Up, true, &pos, self);
                move_camera_collider(self.camera_collider[3], world, Direction::Down, false, &pos, self);
            } else if pos.y + self.height + 1.0 >= world.actor_pos(self.camera_collider[3]).y && !(pos.y -1.0 <= world.actor_pos(self.camera_collider[2]).y + screen_height() / 8.0) {
                let x = world.actor_pos(self.camera_collider[0]);
                set_camera(&Camera2D::from_display_rect(Rect::new(x.x, ((pos.y + self.height + screen_height() / 8.0) - screen_height()) + screen_height(), screen_width(), -screen_height())));
                move_camera_collider(self.camera_collider[2], world, Direction::Up, false, &pos, self);
                move_camera_collider(self.camera_collider[3], world, Direction::Down, true, &pos, self);
            }
        }
    }

    pub async fn tick(&mut self, level_data: &mut LevelData, world: &World, settings: &Settings) {
        let enemies = &level_data.enemies;
        let colliding_enemies = self.collider_new.collide_check_enemy(enemies, vec2(0.0, 0.0)).await;
        if !colliding_enemies.is_empty() {
            for enemy in colliding_enemies {
                let damage = enemies.get(enemy).expect("Oh no! This should be impossible!").damage;
                self.damage(damage).await;
            }
        }

        if *self.triggers.get(&PlayerTrigger::Damage).unwrap_or(&false) {
            self.color = RED;
            if self.triggers_exec.get(&PlayerTrigger::Damage).unwrap() + 0.5 < get_time() {
                self.triggers.remove(&PlayerTrigger::Damage);
                self.triggers_exec.remove(&PlayerTrigger::Damage);
                self.color = WHITE;
            }
        }

        if self.health == 0 {
            level_data.triggers.insert(Trigger::GameOver, true);
        }

        if !self.triggers.get(&PlayerTrigger::ShootTimeout).unwrap_or(&false) {
            if is_mouse_button_pressed(MouseButton::Left) {
                let size = vec2(32.0, 32.0) * settings.gui_scale;
                let pos = world.actor_pos(self.collider) + vec2(self.width / 2.0, self.height / 2.0) - vec2(size.x / 2.0, size.y / 2.0);
                let pos_c_x = world.actor_pos(self.camera_collider[0]);
                let pos_c_y = world.actor_pos(self.camera_collider[3]);
                let (mut mouse_x, mut mouse_y) = mouse_position();
                mouse_x += pos_c_x.x;
                mouse_y -= pos_c_y.normalize().y;

                let movement_vector = point_to_point_direction_with_speed(pos, vec2(mouse_x, mouse_y), 1000.0 * settings.gui_scale).await;

                let projectile  = Projectile::new(
                    pos,
                    size,
                    -25,
                    4.0,
                    TextureKey::Coin0, ProjectileOrigin::Player, movement_vector).await;

                self.triggers.insert(PlayerTrigger::ShootTimeout, true);
                self.triggers_exec.insert(PlayerTrigger::ShootTimeout, get_time());

                level_data.projectiles.push(projectile);
            } else if is_key_pressed(KeyCode::Q) {
                let size = vec2(32.0, 32.0) * settings.gui_scale;
                let pos = world.actor_pos(self.collider) + vec2(self.width / 2.0, self.height / 2.0)- vec2(size.x / 2.0, size.y / 2.0);

                let movement_vector = vec2(-1.0, 0.0) * (1000.0 * settings.gui_scale);

                let projectile  = Projectile::new(
                    pos,
                    size,
                    -25,
                    4.0,
                    TextureKey::Coin0, ProjectileOrigin::Player, movement_vector).await;

                self.triggers.insert(PlayerTrigger::ShootTimeout, true);
                self.triggers_exec.insert(PlayerTrigger::ShootTimeout, get_time());

                level_data.projectiles.push(projectile);
            } else if is_key_pressed(KeyCode::E) {
                let size = vec2(32.0, 32.0) * settings.gui_scale;
                let pos = world.actor_pos(self.collider) + vec2(self.width / 2.0, self.height / 2.0)- vec2(size.x / 2.0, size.y / 2.0);

                let movement_vector = vec2(1.0, 0.0) * (1000.0 * settings.gui_scale);

                let projectile  = Projectile::new(
                    pos,
                    size,
                    -25,
                    4.0,
                    TextureKey::Coin0, ProjectileOrigin::Player, movement_vector).await;

                self.triggers.insert(PlayerTrigger::ShootTimeout, true);
                self.triggers_exec.insert(PlayerTrigger::ShootTimeout, get_time());

                level_data.projectiles.push(projectile);
            }
        } else if self.triggers_exec.get(&PlayerTrigger::ShootTimeout).unwrap_or(&0.0) + 0.05 < get_time() {
            self.triggers.remove(&PlayerTrigger::ShootTimeout);
            self.triggers_exec.remove(&PlayerTrigger::ShootTimeout);
        }
    }

    /// Moves the player and checks for all necessary things (like collision)
    pub async fn perform_move(&mut self, world: &mut World) {
        // Set positions using the previously defined speeds
        world.move_h(self.collider, self.speed.x * get_frame_time());
        world.move_v(self.collider, self.speed.y * get_frame_time());

        let pos = world.actor_pos(self.collider);
        self.collider_new.change_pos(pos).await;
    }

    pub async fn damage(&mut self, health: i16) {
        self.health += health;
        if self.health < 0 { self.health = 0; }
        self.triggers.insert(PlayerTrigger::Damage, true);
        self.triggers_exec.insert(PlayerTrigger::Damage, get_time());
    }

    pub async fn render(&mut self, world: &World, textures: &BTreeMap<TextureKey, Vec<Texture2D>>, settings: &Settings) {
        let pos = world.actor_pos(self.collider);

        draw_texture_ex(
            &textures.get(&TextureKey::Player).unwrap().get(self.state as usize).unwrap(), pos.x, pos.y, self.color,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                ..Default::default()
            },
        );

        // Draw Health bar
        let height = 32.0 * settings.gui_scale;
        let camera_collider_pos = world.actor_pos(self.camera_collider[0]);
        let camera_collider_pos_2 = world.actor_pos(self.camera_collider[2]);
        draw_rectangle(camera_collider_pos.x, camera_collider_pos_2.y, (self.health as f32 / 2.0) * settings.gui_scale, height, RED);
    }
}