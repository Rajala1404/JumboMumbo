use std::collections::BTreeMap;
use macroquad::color::{DARKPURPLE, GRAY, WHITE};
use macroquad::input::{is_key_down, is_key_released, KeyCode};
use macroquad::prelude::{screen_height, screen_width};
use macroquad::shapes::draw_rectangle_lines;
use macroquad::text::{draw_text, measure_text};
use macroquad::time::get_fps;
use macroquad_platformer::World;
use crate::logic::collider::Collider;
use crate::logic::player::Player;
use crate::scenes::levels::structs::{LevelSceneData, Trigger};
use crate::utils::structs::Settings;

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
    let player = level_scene_data.level_data.player.as_ref().unwrap();
    let world = &level_scene_data.world;
    let triggers = &level_scene_data.level_data.triggers;

    if is_active(Trigger::ShowCameraColliders, triggers).await { draw_camera_collider(world, player, settings).await; }

    if is_active(Trigger::ShowColliders, triggers).await {
        let collectibles = async {
            for collectible in &level_scene_data.level_data.collectibles {
                collectible.collider.debug_render(settings).await;
            }
        };

        let enemies = async {
            for enemy in &level_scene_data.level_data.enemies {
                let iter_colliders: Vec<Collider> = enemy.colliders.clone().into();
                for collider in iter_colliders {
                    collider.debug_render(settings).await;
                }
            }
        };

        let platforms = async {
            for platform in &level_scene_data.level_data.platforms {
                platform.collider_new.debug_render(settings).await;
            }
        };

        let projectiles = async {
            for projectile in &level_scene_data.level_data.projectiles {
                projectile.collider.debug_render(settings).await;
            }
        };

        level_scene_data.level_data.player.as_ref().unwrap().collider_new.debug_render(settings).await;
        collectibles.await;
        enemies.await;
        platforms.await;
        projectiles.await;
    }

    if is_active(Trigger::ShowFPS, triggers).await {
        let camera_pos = world.actor_pos(level_scene_data.level_data.player.as_ref().unwrap().camera_collider[0]);
        let text = get_fps().to_string();
        let size = measure_text(&text, None, (32.0 * settings.gui_scale) as _, 1.0);
        draw_text(&text, camera_pos.x, size.height, 32.0 * settings.gui_scale, GRAY);
    }
}

async fn is_active(trigger: Trigger, triggers: &BTreeMap<Trigger, bool>) -> bool {
    triggers.get(&trigger).unwrap_or(&false).to_owned()
}

pub async fn check(triggers: &mut BTreeMap<Trigger, bool>, trigger_locks: &mut BTreeMap<Trigger, bool>) {
    debug_key_combo(KeyCode::C, Trigger::ShowCameraColliders, triggers, trigger_locks).await;
    debug_key_combo(KeyCode::H, Trigger::ShowColliders, triggers, trigger_locks).await;
    debug_key_combo(KeyCode::F, Trigger::ShowFPS, triggers, trigger_locks).await;
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