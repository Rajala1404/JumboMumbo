use std::collections::BTreeMap;
use macroquad::camera::set_camera;
use macroquad::color::WHITE;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_texture_ex, get_frame_time, screen_height, Camera2D, DrawTextureParams, Rect, Texture2D};
use macroquad::window::screen_width;
use macroquad_platformer::{Actor, World};
use crate::TextureKey;



// This file contains everything that is for the player

#[derive(PartialEq, Copy, Clone)]
pub struct Player {
    /// 0: Left <br>
    /// 1: Right
    pub state: i8,
    pub collider: Actor,
    pub camera_collider: [Actor; 2],
    pub speed: Vec2,
}

impl Player {
    /// This function handles everything regarding the controls of the player (including moving)
    pub async fn control(&mut self, world: &mut World, camera: &mut Camera2D) {
        // gets the current position of the player from the world
        let pos = world.actor_pos(self.collider);
        let on_ground = world.collide_check(self.collider, pos + vec2(0.0, 1.0));

        // If the player is not on the ground change velocity of y to 500 (to simulate gravity)
        if on_ground == false {      // multiplies by get_frame_time() so the speed is on all refresh rates the same
            self.speed.y += screen_height() / 0.3 * get_frame_time();
        }

        // Checks if key is currently pressed
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            // If D or Right Arrow is pressed the Player will be moved to the right by increasing the speed on the x-axis
            self.speed.x = screen_width() / 2.5;
            self.state = 1;
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.speed.x = screen_width() / -2.5;
            self.state = 0;
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

        // Make camera follow player
        if pos.x -1.0 <= world.actor_pos(self.camera_collider[0]).x + screen_width() / 4.0 {
            set_camera(&Camera2D::from_display_rect(Rect::new(pos.x - screen_width() / 4.0 , screen_height() , screen_width(), -screen_height())));
            world.move_h(self.camera_collider[0], self.speed.x * get_frame_time());
            world.move_h(self.camera_collider[1], self.speed.x * get_frame_time());
        } else if pos.x + screen_height() / 12.0 + 1.0 >= world.actor_pos(self.camera_collider[1]).x {
            set_camera(&Camera2D::from_display_rect(Rect::new(pos.x + screen_height() / 12.0 - (screen_width() - screen_width() / 4.0) , screen_height() , screen_width(), -screen_height())));
            world.move_h(self.camera_collider[0], self.speed.x * get_frame_time());
            world.move_h(self.camera_collider[1], self.speed.x * get_frame_time());
        }
    }

    pub async fn render(&mut self, world: &World, textures: &BTreeMap<TextureKey, Vec<Texture2D>>) {
        let pos = world.actor_pos(self.collider);
        let width = screen_height() / 12.0;
        let height = screen_height() / 12.0;

        draw_texture_ex(
            &textures.get(&TextureKey::Player).unwrap().get(self.state as usize).unwrap(), pos.x, pos.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                ..Default::default()
            },
        );
    }
}