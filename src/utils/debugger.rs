use macroquad::color::{DARKPURPLE, WHITE};
use macroquad::prelude::{draw_rectangle, screen_height, screen_width};
use macroquad::text::draw_text;
use macroquad_platformer::World;
use crate::logic::player::Player;

pub async fn draw_camera_collider(world: &World, player: &Player) {
    let pos = world.actor_pos(player.camera_collider[0]);
    draw_rectangle(pos.x, pos.y, screen_width() / 4.0, screen_height(), DARKPURPLE);
    draw_text("Camera collider 0", pos.x, pos.y, screen_height() / 20.0, WHITE);
    let pos = world.actor_pos(player.camera_collider[1]);
    draw_rectangle(pos.x, pos.y, screen_width() / 4.0, screen_height(), DARKPURPLE);
    draw_text("Camera collider 1", pos.x, pos.y, screen_height() / 20.0, WHITE);
}