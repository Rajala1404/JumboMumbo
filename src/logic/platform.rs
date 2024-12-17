use macroquad::math::{f32, vec2, Vec2};
use macroquad_platformer::{Solid, World};
use std::collections::BTreeMap;
use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Texture2D};
use macroquad::color::WHITE;
use crate::logic::collider::Collider;
use crate::utils::enums::TextureKey;

#[derive(PartialEq, Clone)]
pub struct Platform {
    pub collider: Solid,
    pub collider_new: Collider,
    pub tile_size: Vec2,
    pub tiles: Vec<PlatformTile>,
    pub speed: Vec2,
}

impl Platform {
    pub async fn new(collider: Solid, pos: Vec2, size: Vec2, tile_size: Vec2, tiles: Vec<PlatformTile>, speed: Vec2) -> Self {
        let collider_new = Collider::new_solid(pos, size.x, size.y, vec2(0.0, 0.0)).await;
        Self { collider, collider_new, tile_size, tiles, speed }
    }

    /// Basic Floating platform
    /// `length` are the tiles between the start and end
    pub async fn floating(length: i32, tile_size: Vec2, texture_key: TextureKey, pos: Vec2, world: &mut World) -> Self {
        let mut tiles = vec![
            PlatformTile {
                texture_key,
                texture_index: 0,
                pos: vec2(0.0, 0.0),
            },
        ];

        for i in 1..length {
            tiles.push(PlatformTile{
                texture_key,
                texture_index: 1,
                pos: vec2(i as f32, 0.0),
            })
        }

        tiles.push(PlatformTile{
            texture_key,
            texture_index: 2,
            pos: vec2(length as f32, 0.0),
        });

        let width = tile_size.x * (length + 1) as f32;

        Self::new(
            world.add_solid(pos, width as i32, tile_size.y as i32),
            pos,
            vec2(width, tile_size.y),
            tile_size,
            tiles,
            vec2(0.0, 0.0)
        ).await
    }

    pub async fn full(length: usize, height: usize, tile_size: Vec2, texture_key: TextureKey, pos: Vec2, world: &mut World) -> Self {
        let mut tiles = vec![
            // Top left corner
            PlatformTile {
                texture_key,
                texture_index: 0,
                pos: vec2(0.0, 0.0),
            },
            // Top right corner
            PlatformTile {
                texture_key,
                texture_index: 2,
                pos: vec2(length as f32 + 1.0, 0.0),
            },
            // Bottom left corner
            PlatformTile {
                texture_key,
                texture_index: 6,
                pos: vec2(0.0, height as f32 + 1.0),
            },
            // Bottom right corner
            PlatformTile {
                texture_key,
                texture_index: 8,
                pos: vec2(length as f32 + 1.0, height as f32 + 1.0),
            }
        ];

        for i in 1..=length {
            // Top border
            tiles.push(PlatformTile {
                texture_key,
                texture_index: 1,
                pos: vec2(i as f32, 0.0),
            });
            // Middle
            for j in 1..=height {
                tiles.push(PlatformTile {
                    texture_key,
                    texture_index: 4,
                    pos: vec2(i as f32, j as f32),
                })
            }
            // Bottom Border
            tiles.push(PlatformTile {
                texture_key,
                texture_index: 7,
                pos: vec2(i as f32, height as f32 + 1.0)
            });
        }

        // Push left and right border
        for i in 1..=height {
            tiles.push(PlatformTile {
                texture_key,
                texture_index: 3,
                pos: vec2(0.0, i as f32),
            });
            tiles.push(PlatformTile {
                texture_key,
                texture_index: 5,
                pos: vec2(length as f32 + 1.0, i as f32),
            })
        }

        let width = (length as f32 + 2.0) * tile_size.x;
        let height = (height as f32 + 2.0) * tile_size.y;
        Self::new(
            world.add_solid(pos, width as i32, height as i32),
            pos,
            vec2(width, height),
            tile_size,
            tiles,
            vec2(0.0, 0.0)
        ).await
    }

    pub async fn render(&self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>, world: &World) {
        let pos = world.solid_pos(self.collider);

        for tile in &self.tiles {
            let texture = textures.get(&tile.texture_key).unwrap().get(tile.texture_index).unwrap();
            let pos =  pos + self.tile_size * tile.pos;
            draw_texture_ex(
                &texture,
                pos.x,
                pos.y,
                WHITE,
                DrawTextureParams{
                    dest_size: Some(self.tile_size),
                    ..Default::default()
                }
            )
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct PlatformTile {
    pub texture_key: TextureKey,
    /// The usize is the Index of the texture inside the TileMap. <br>
    /// For more info please see the json of the platform you're trying to render
    pub texture_index: usize,
    /// Contains the relative position of the Platform tile (e.g. vec2(1.0, 0.0))
    pub pos: Vec2,
}

impl PlatformTile {
    pub async fn new(texture_key: TextureKey, texture_index: usize, pos: Vec2) -> Self {
        Self {texture_key, texture_index, pos}
    }
}