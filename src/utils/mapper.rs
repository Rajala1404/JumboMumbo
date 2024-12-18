use macroquad::color::Color;
use macroquad::math::{vec2, Vec2};
use macroquad::texture::load_image;
use macroquad::time::get_time;
use macroquad_platformer::World;
use crate::logic::cannon::Cannon;
use crate::logic::collectible::{Collectible, CollectibleType};
use crate::logic::enemy::Enemy;
use crate::logic::level::Level;
use crate::logic::platform::{Platform, PlatformTile};
use crate::logic::player::{Player, PlayerPowerUp, PowerUp};
use crate::utils::enums::{Animation, AnimationType, Direction, TextureKey};
use crate::utils::structs::{Matrix, Settings};

/// # Colors
/// `rgba()` <br>
/// `(92, 0, 255, 255)` = Player <br>
/// `(255, 0, _, 255)` = Platform (0) <br>
/// `(254, 0, _, 255)` = Platform (1) <br>
/// ... <br>
/// `(247, 0, _, 255)` = Platform (8) <br>
/// `(246, 1..255, 0..255, 1..255)` = Cannon (Left) where G / 10 is the shooting speed, B / 10 the offset in Seconds and A * 2 the damage <br>
/// `(200, 1..255, 0..255, 1..255)` = Cannon (Right) <br>
/// `(100, 1..255, 0..255, 1.255)` = Cannon (Up) <br>
/// `(0, 1..255, 0..255, 1.255)` = Cannon (Down) <br>
/// `(245, 255, 0, 255)` = Coin <br>
/// `(244, 120, 0, 1..255)` = Coin2x where A is the duration in Seconds <br>
/// `(243, 0, 255, 1..255)` = SpeedBoost where A is the duration in Seconds <br>
/// `(242, 255, 100, 1..255)` = DamageBoost where A is the duration in Seconds <br>
/// `(80, 255, 255, 1..255)` = JumpBoost where A is the duration in Seconds <br>
/// `(241, 120, 100, 1..255)` = Enemy where A * 2.0 is the damage (in reverse) <br>
///
/// 0, 0 is at the bottom left of the image
pub async fn level_map_from_image(
    path: String,
    tile_size: Vec2,
    settings: &Settings,
    world: &mut World,
    platform_texture_key: TextureKey,
    coin_texture_key: TextureKey,
    enemy_texture_key: TextureKey,
    cannon_texture_key: TextureKey,
    projectile_texture_key: TextureKey,
    power_ups_texture_key: TextureKey,
) -> (Player, Vec<Platform>, Vec<Collectible>, Vec<Enemy>, Vec<Cannon>, Vec<PowerUp>) {
    let start_time = get_time();
    let nv2 = vec2(0.0, 0.0);
    let mut player = Player::new(
        0.0,
        0.0,
        vec2(0.0, 0.0),
        0,
        world,
    ).await;

    let mut platforms = Vec::new();
    let mut collectibles = Vec::new();
    let mut enemies = Vec::new();
    let mut cannons = Vec::new();
    let mut power_ups = Vec::new();

    let matrix: Matrix<Color> = load_image(&path).await.unwrap().into();

    for ((row, col), color) in matrix {
        let rgba = [(color.r * 255.0) as i32, (color.g * 255.0) as i32, (color.b * 255.0) as i32, (color.a * 255.0) as i32];
        if rgba[3] == 0 { continue; }
        let pos = vec2(tile_size.x * row as f32, tile_size.y * col as f32);

        match rgba {
            [92, 0, 255, 255] => { // Player
                player = Player::new(
                    tile_size.x - 2.0,
                    tile_size.y - 2.0,
                    vec2(tile_size.x * row as f32, tile_size.y * col as f32),
                    1,
                    world,
                ).await;
            },
            [247..=255, 0, _, 255] => { // Platforms
                let tile = vec![
                    PlatformTile::new(
                        platform_texture_key,
                        255 - rgba[0] as usize,
                        vec2(0.0, 0.0)
                    ).await
                ];

                platforms.push(Platform::new(
                    world.add_solid(pos, tile_size.x as i32, tile_size.y as i32),
                    pos,
                    tile_size,
                    tile_size,
                    tile,
                    nv2.to_owned()
                ).await);
            },
            [246, 1..=255, 0..=255, 1..=255] => { // Cannon left
                cannons.push(Cannon::new(
                    pos,
                    tile_size,
                    rgba[1] as f64 / 10.0,
                    rgba[2] as f64 / 10.0,
                    Direction::Left,
                    start_time,
                    1000.0 * settings.gui_scale,
                    10.0,
                    cannon_texture_key,
                    projectile_texture_key,
                    (255 - rgba[3]) as i16 * -2,
                    world
                ).await)
            },
            [200, 1..=255, 0..=255, 1..=255] => { // Cannon right
                cannons.push(Cannon::new(
                    pos,
                    tile_size,
                    rgba[1] as f64 / 10.0,
                    rgba[2] as f64 / 10.0,
                    Direction::Right,
                    start_time,
                    1000.0 * settings.gui_scale,
                    10.0,
                    cannon_texture_key,
                    projectile_texture_key,
                    (255 - rgba[3]) as i16 * -2,
                    world
                ).await);
            },
            [100, 1..=255, 0..=255, 1..=255] => { // Cannon up
                cannons.push(Cannon::new(
                    pos,
                    tile_size,
                    rgba[1] as f64 / 10.0,
                    rgba[2] as f64 / 10.0,
                    Direction::Up,
                    start_time,
                    1000.0 * settings.gui_scale,
                    10.0,
                    cannon_texture_key,
                    projectile_texture_key,
                    (255 - rgba[3]) as i16 * -2,
                    world
                ).await);
            },
            [0, 1..=255, 0..=255, 1..=255] => { // Cannon down
                cannons.push(Cannon::new(
                    pos,
                    tile_size,
                    rgba[1] as f64 / 10.0,
                    rgba[2] as f64 / 10.0,
                    Direction::Down,
                    start_time,
                    1000.0 * settings.gui_scale,
                    10.0,
                    cannon_texture_key,
                    projectile_texture_key,
                    (255 - rgba[3]) as i16 * -2,
                    world
                ).await);
            },
            [245, 255, 0, 255] => { // Coin
                collectibles.push(Collectible::new(
                    CollectibleType::Coin,
                    pos,
                    tile_size,
                    coin_texture_key,
                    Animation::new(AnimationType::Cycle(0, 5, 0.1)),
                    nv2.to_owned()
                ).await);
            },
            [244, 120, 0, 1..=255] => { // Coin2x
                power_ups.push(PowerUp::new(
                    PlayerPowerUp::Coins2x,
                    rgba[3] as f64,
                    pos,
                    tile_size,
                    power_ups_texture_key,
                    (41, 63),
                    0.1
                ).await)
            },
            [243, 0, 255, 1..=255] => { // SpeedBoost
                power_ups.push(PowerUp::new(
                    PlayerPowerUp::SpeedBoost,
                    rgba[3] as f64,
                    pos,
                    tile_size,
                    power_ups_texture_key,
                    (18, 40),
                    0.1
                ).await)
            },
            [242, 255, 100, 1..=255] => { // DamageBoost
                power_ups.push(PowerUp::new(
                    PlayerPowerUp::Damage2x,
                    rgba[3] as f64,
                    pos,
                    tile_size,
                    power_ups_texture_key,
                    (64, 83),
                    0.1
                ).await)
            },
            [80, 255, 255, 1..=255] => { // JumpBoost
                power_ups.push(PowerUp::new(
                    PlayerPowerUp::JumpBoost,
                    rgba[3] as f64,
                    pos,
                    tile_size,
                    power_ups_texture_key,
                    (0, 17),
                    0.1
                ).await)
            },
            [241, 120, 100, 1..=255] => { // Enemy
                enemies.push(Enemy::new(
                    pos,
                    (255 - rgba[3]) as i16 * -2,
                    world,
                    tile_size - vec2(2.0, 2.0),
                    enemy_texture_key
                ).await);
            }
            _ => {}
        }
    }

    (player, platforms, collectibles, enemies, cannons, power_ups)
}

pub async fn level_map_image_path(level: Level) -> String {
    match level {
        Level::Level2 => "./res/levels/level_2.png".to_string(),
        Level::Level3 => "./res/levels/level_3.png".to_string(),
        _ => unimplemented!()
    }
}