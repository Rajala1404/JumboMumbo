use macroquad::color::{DARKPURPLE, WHITE};
use macroquad::prelude::{screen_height, screen_width};
use macroquad::shapes::draw_rectangle_lines;
use macroquad::text::draw_text;
use macroquad_platformer::World;
use crate::logic::player::Player;

pub async fn draw_camera_collider(world: &World, player: &Player) {
    let x_offset =  screen_width() / 60.0;
    let y_offset = screen_height() - screen_height() / 15.0;
    let pos = world.actor_pos(player.camera_collider[0]);
    draw_rectangle_lines(pos.x, pos.y , screen_width() / 4.0, screen_height(), screen_height() / 20.0, DARKPURPLE);
    draw_text("Camera collider 0", pos.x + x_offset, pos.y + y_offset, screen_height() / 20.0, WHITE);
    let pos = world.actor_pos(player.camera_collider[1]);
    draw_rectangle_lines(pos.x, pos.y, screen_width() / 4.0, screen_height(), screen_height() / 20.0, DARKPURPLE);
    draw_text("Camera collider 1", pos.x + x_offset, pos.y + y_offset, screen_height() / 20.0, WHITE);
}