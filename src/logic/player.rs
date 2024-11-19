use std::collections::BTreeMap;
use macroquad::camera::set_camera;
use macroquad::color::WHITE;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_texture_ex, get_frame_time, screen_height, Camera2D, DrawTextureParams, Rect, Texture2D};
use macroquad::window::screen_width;
use macroquad_platformer::{Actor, World};
use crate::utils::enums::TextureKey;



// This file contains everything that is for the player

#[derive(PartialEq, Copy, Clone)]
pub struct Player {
    pub width: f32,
    pub height: f32,
    /// 0: Left <br>
    /// 1: Right
    pub state: i8,
    pub collider: Actor,
    pub camera_collider: [Actor; 2],
    pub speed: Vec2,
}

impl Player {
    pub async fn new(width: f32, height: f32, pos: Vec2, state: i8, world: &mut World) -> Self {
        Player {
            width,
            height,
            state,
            collider: world.add_actor(pos, width as i32, height as i32),
            camera_collider: [
                world.add_actor(vec2(0.0, 0.0), (screen_width() / 4.0) as i32, screen_height() as i32),
                world.add_actor(vec2(screen_width() - (screen_width() / 4.0), 0.0), (screen_width() / 4.0) as i32, screen_height() as i32),
            ],
            speed: vec2(0.0, 0.0),
        }
    }

    /// This function handles everything regarding the controls of the player (including moving)
    pub async fn control(&mut self, world: &mut World) {
        // gets the current position of the player from the world
        let pos = world.actor_pos(self.collider);
        let on_ground = world.collide_check(self.collider, pos + vec2(0.0, 1.0));

        // If the player is not on the ground change velocity of y to 500 (to simulate gravity)
        if on_ground == false {      // multiplies by get_frame_time() so the speed is on all refresh rates the same
            self.speed.y += screen_height() / 0.3 * get_frame_time();
        }

        // 1 = Left, 2 = Right
        let mut direction = 0;

        // Checks if key is currently pressed
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            // If D or Right Arrow is pressed the Player will be moved to the right by increasing the speed on the x-axis
            self.speed.x = screen_width() / 2.5;
            self.state = 1;
            direction = 2;
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.speed.x = screen_width() / -2.5;
            self.state = 0;
            direction = 1;
        } else {
            // Resets the speed if nothing is pressed
            self.speed.x = 0.0;
        }

        if is_key_down(KeyCode::Space) {
            if on_ground {
                self.speed.y = screen_height() / -0.9;
            }
        }

        // Set positions using the previously defined speeds
        world.move_h(self.collider, self.speed.x * get_frame_time());
        world.move_v(self.collider, self.speed.y * get_frame_time());

        let pos = world.actor_pos(self.collider);

        fn move_camera_collider(collider: Actor, world: &mut World, left: bool, trigger: bool, pos: &Vec2, player: &Player) {
            let y = pos.y + player.height - screen_height();
            if left {
                if trigger {
                    world.set_actor_position(collider, vec2(pos.x - screen_width() / 4.0, y));
                } else {
                    world.set_actor_position(collider, vec2(pos.x + player.width + screen_width() / 4.0 - screen_width(), y));
                }
            } else {
                if trigger {
                    world.set_actor_position(collider, vec2(pos.x + player.width, y));
                } else {
                    world.set_actor_position(collider, vec2(pos.x + (screen_width() / 2.0), y));
                }
            }
        }

        // Make camera follow player
        if pos.x -1.0 <= world.actor_pos(self.camera_collider[0]).x + screen_width() / 4.0 && direction != 2 {
            set_camera(&Camera2D::from_display_rect(Rect::new(pos.x - screen_width() / 4.0 , screen_height() , screen_width(), -screen_height())));

            move_camera_collider(self.camera_collider[0], world, true, true, &pos, self);
            move_camera_collider(self.camera_collider[1], world, false, false, &pos, self);
        } else if pos.x + screen_height() / 12.0 + 1.0 >= world.actor_pos(self.camera_collider[1]).x && direction != 1 {
            set_camera(&Camera2D::from_display_rect(Rect::new(pos.x + self.width - (screen_width() - screen_width() / 4.0) , screen_height() , screen_width(), -screen_height())));

            move_camera_collider(self.camera_collider[0], world, true, false, &pos, self);
            move_camera_collider(self.camera_collider[1], world, false, true, &pos, self);
        }
    }

    pub async fn render(&mut self, world: &World, textures: &BTreeMap<TextureKey, Vec<Texture2D>>) {
        let pos = world.actor_pos(self.collider);

        draw_texture_ex(
            &textures.get(&TextureKey::Player).unwrap().get(self.state as usize).unwrap(), pos.x, pos.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                ..Default::default()
            },
        );
    }
}