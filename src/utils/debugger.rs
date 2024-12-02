use std::collections::BTreeMap;
use macroquad::color::{DARKPURPLE, WHITE};
use macroquad::input::{is_key_down, is_key_released, KeyCode};
use macroquad::prelude::{screen_height, screen_width};
use macroquad::shapes::draw_rectangle_lines;
use macroquad::text::draw_text;
use macroquad_platformer::World;
use crate::logic::player::Player;
use crate::scenes::levels::structs::{LevelSceneData, Trigger};
use crate::Settings;

pub async fn draw_camera_collider(world: &World, player: &Player, settings: &Settings) {
    let x_offset =  screen_width() / 60.0;
    let y_offset = screen_height() - screen_height() / 15.0;
    let f_size = 50.0 * settings.gui_scale;
    let thickness = f_size;
    let pos = world.actor_pos(player.camera_collider[0]);

    draw_rectangle_lines(pos.x, pos.y , screen_width() / 4.0, screen_height(), thickness, DARKPURPLE);
    draw_text("Camera collider 0", pos.x + x_offset, pos.y + y_offset, f_size, WHITE);
    let pos = world.actor_pos(player.camera_collider[1]);
    draw_rectangle_lines(pos.x, pos.y, screen_width() / 4.0, screen_height(), thickness, DARKPURPLE);
    draw_text("Camera collider 1", pos.x + x_offset, pos.y + y_offset, f_size, WHITE);
}

pub async fn render(level_scene_data: &LevelSceneData, settings: &Settings) {
    let player = level_scene_data.player.as_ref().unwrap();
    let world = &level_scene_data.world;
    let triggers = &level_scene_data.triggers;

    if is_active(Trigger::ShowCameraColliders, triggers).await { draw_camera_collider(world, player, settings).await; }

    if is_active(Trigger::ShowColliders, triggers).await {
        let collectibles = async {
            for collectible in &level_scene_data.collectibles {
                collectible.collider.debug_render(settings).await;
            }
        };

        level_scene_data.player.as_ref().unwrap().collider_new.debug_render(settings).await;
        collectibles.await;
    }
}

pub async fn check(triggers: &mut BTreeMap<Trigger, bool>, trigger_locks: &mut BTreeMap<Trigger, bool>) {
    debug_key_combo(KeyCode::C, Trigger::ShowCameraColliders, triggers, trigger_locks).await;
    debug_key_combo(KeyCode::H, Trigger::ShowColliders, triggers, trigger_locks).await;
}

async fn is_active(trigger: Trigger, triggers: &BTreeMap<Trigger, bool>) -> bool {
    triggers.get(&trigger).unwrap_or(&false).to_owned()
}

async fn debug_key_combo(key: KeyCode, trigger: Trigger, triggers: &mut BTreeMap<Trigger, bool>, trigger_locks: &mut BTreeMap<Trigger, bool>) {
    { // Initial Press
        let pressed = is_key_down(KeyCode::Q) && is_key_down(key) && !trigger_locks.get(&trigger).unwrap_or(&false).to_owned();

        if pressed {
            let value = triggers.get(&trigger);
            triggers.insert(trigger.to_owned(), !value.unwrap_or(&false));
            trigger_locks.insert(trigger.to_owned(), true);
        }
    }

    { // Released
        let released = is_key_released(KeyCode::Q) || is_key_released(key) && trigger_locks.get(&trigger).unwrap_or(&false).to_owned();

        if released {
            trigger_locks.insert(trigger.to_owned(), false);
        }
    }
}