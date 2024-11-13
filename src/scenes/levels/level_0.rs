use std::collections::BTreeMap;
use crate::{Scene, SceneTextureKey, TextureKey};
use macroquad::prelude::*;
use macroquad_platformer::World;
use crate::logic::player::Player;
use crate::scenes::levels::levels::{Level, LevelSceneData, Platform};


pub async fn level_0(scene: &mut Scene, mut camera: &mut Camera2D, mut textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>, level_scene_data: &mut LevelSceneData) {
    clear_background(DARKBLUE);

    // Load textures if not loaded already
    if textures.get(&SceneTextureKey::Level0).is_none() {
        load_textures(&mut textures).await;
    }
    let textures = textures.get(&SceneTextureKey::Level0).unwrap();

    // Load scene data for right level
    if level_scene_data.level != Some(Level::Level0) {
        let mut world = World::new();
        let x = screen_width() / 2.0;
        let y = screen_height() / 2.0;
        let width = screen_height() / 12.0;
        let height = screen_height() / 12.0;
        let pos = vec2(x, y);

        *level_scene_data = LevelSceneData {
            level: Some(Level::Level0),
            player: Some(Player {
                state: 1,
                collider: world.add_actor(pos, width as i32, height as i32),
                camera_collider: [
                    world.add_actor(vec2(0.0, 0.0), (screen_width() / 4.0) as i32, screen_height() as i32),
                    world.add_actor(vec2(screen_width() - (screen_width() / 4.0), 0.0), (screen_width() / 4.0) as i32, screen_height() as i32),
                ],
                speed: vec2(0.0, 0.0),
            }),
            platforms: vec![Platform{collider: world.add_solid(vec2(0.0 - screen_width(), screen_height()), screen_width()  as i32 * 3, (screen_height() / 32.0) as i32 ), speed: vec2(0.0, 0.0)}],
            world: Some(world),
        }
    }

    let mut world = level_scene_data.world.as_mut().unwrap();
    let player = level_scene_data.player.as_mut().unwrap();

    player.control(&mut world, &mut camera).await;
    player.render(&mut world, &textures).await;

    draw_line(screen_width() / 2.0, screen_height() / 2.0, screen_width() / 2.0 + 100.0, screen_height() / 2.0 + 100.0, 10.0, WHITE);
}

async fn load_textures(textures: &mut BTreeMap<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>) {
    let mut result = BTreeMap::new();

    // Load player textures
    let player = {
        let mut result = Vec::new();

        let player_walk_left = load_texture("res/textures/player/player_walk_left.png").await.unwrap();
        // FilterMode is set to Nearest so it doesn't pixelate when scaling
        player_walk_left.set_filter(FilterMode::Nearest);
        let player_walk_right = load_texture("res/textures/player/player_walk_right.png").await.unwrap();
        player_walk_right.set_filter(FilterMode::Nearest);

        result.push(player_walk_left);
        result.push(player_walk_right);

        result
    };
    result.insert(TextureKey::Player, player);



    // Insert result into the global texture map
    textures.insert(SceneTextureKey::Level0, result);
}