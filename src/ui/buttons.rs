use std::collections::BTreeMap;
use macroquad::color::WHITE;
use macroquad::input::{is_mouse_button_down, is_mouse_button_released, mouse_position, MouseButton};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_texture_ex, is_mouse_button_pressed, measure_text, Texture2D};
use macroquad::text::draw_text;
use macroquad::texture::DrawTextureParams;
use crate::utils::enums::TextureKey;
use crate::utils::structs::Rect;

pub struct Button {
    pub pos: Vec2,
    pub size: Vec2,
    pub border_size: Vec2,
    pub rect: Rect,
    pub text: String,
    pub font_size: f32,
    /// `0, 8` is the normal state
    /// `9, 17` is the hover state
    /// `18, 26` is the down state
    pub texture_key: TextureKey
}

impl Button {
    pub async fn new(pos: Vec2, size: Vec2, border_size: Vec2, text: String, font_size: f32, texture_key: TextureKey) -> Self {
        let rect = Rect::new(pos.x, pos.y, size.x, size.y).await;
        Self { pos, size, border_size, rect, text, font_size, texture_key }
    }

    pub async fn is_pressed(&self, button: MouseButton) -> bool {
        self.is_hovered().await && is_mouse_button_pressed(button)
    }

    pub async fn is_down(&self, button: MouseButton) -> bool {
        self.is_hovered().await && is_mouse_button_down(button)
    }

    pub async fn is_released(&self, button: MouseButton) -> bool {
        self.is_hovered().await && is_mouse_button_released(button)
    }

    pub async fn is_hovered(&self) -> bool {
        let mouse_rect = {
            let (mouse_x, mouse_y) = mouse_position();
            Rect::new(mouse_x, mouse_y, 1.0, 1.0).await
        };

        self.rect.overlaps(&mouse_rect).await
    }

    pub async fn render(&self, textures: &BTreeMap<TextureKey, Vec<Texture2D>>) {

        if self.is_down(MouseButton::Left).await || self.is_down(MouseButton::Right).await || self.is_down(MouseButton::Middle).await {
            let texture_slice = textures.get(&self.texture_key).unwrap().get(18..27).unwrap();
            self.render_texture_slice(texture_slice).await;
        } else if self.is_hovered().await {
            let texture_slice = textures.get(&self.texture_key).unwrap().get(9..18).unwrap();
            self.render_texture_slice(texture_slice).await;
        } else {
            let texture_slice = textures.get(&self.texture_key).unwrap().get(0..9).unwrap();
            self.render_texture_slice(texture_slice).await;
        }

        let size = measure_text(&self.text, None, self.font_size as _, 1.0);
        draw_text(
            &self.text,
            self.pos.x + self.size.x / 2.0 - size.width / 2.0,
            self.pos.y + self.size.y / 2.0 + size.offset_y / 2.0,
            self.font_size,
            WHITE
        );
    }

    async fn render_texture_slice(&self, texture_slice: &[Texture2D]) {
        // Draw top left corner
        draw_texture_ex(
            &texture_slice[0],
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.border_size),
                ..Default::default()
            }
        );

        // Draw top border
        draw_texture_ex(
            &texture_slice[1],
            self.pos.x + self.border_size.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.size.x - self.border_size.x * 2.0, self.border_size.y)),
                ..Default::default()
            }
        );

        // Draw top right corner
        draw_texture_ex(
            &texture_slice[2],
            self.pos.x + self.size.x - self.border_size.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.border_size),
                ..Default::default()
            }
        );

        // Draw left border
        draw_texture_ex(
            &texture_slice[3],
            self.pos.x,
            self.pos.y + self.border_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.border_size.x, self.size.y - self.border_size.y * 2.0)),
                ..Default::default()
            }
        );

        // Draw middle
        draw_texture_ex(
            &texture_slice[4],
            self.pos.x + self.border_size.x,
            self.pos.y + self.border_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size - self.border_size * 2.0),
                ..Default::default()
            }
        );

        // Draw right border
        draw_texture_ex(
            &texture_slice[5],
            self.pos.x + self.size.x - self.border_size.x,
            self.pos.y + self.border_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.border_size.x, self.size.y - self.border_size.y * 2.0)),
                ..Default::default()
            }
        );

        // Draw bottom left corner
        draw_texture_ex(
            &texture_slice[6],
            self.pos.x,
            self.pos.y + self.size.y - self.border_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.border_size),
                ..Default::default()
            }
        );

        // Draw bottom border
        draw_texture_ex(
            &texture_slice[7],
            self.pos.x + self.border_size.x,
            self.pos.y + self.size.y - self.border_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.size.x - self.border_size.x * 2.0, self.border_size.y)),
                ..Default::default()
            }
        );

        // Draw bottom left corner
        draw_texture_ex(
            &texture_slice[8],
            self.pos.x + self.size.x - self.border_size.x,
            self.pos.y + self.size.y - self.border_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.border_size),
                ..Default::default()
            }
        )
    }
}