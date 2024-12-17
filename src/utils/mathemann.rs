use std::f32::consts::PI;
use std::ops::Range;
use macroquad::math::Vec2;
// Powered by 100% Vegan MatheMANN™

pub async fn stretch_float_to(f: f32, max_size: f32, target_f: f32) -> f32 {
    f * target_f / max_size
}

pub async fn plus_minus_range<T: PartialOrd + Clone>(value: T, target: T) -> Range<T>{
    if value > target {
        target..value
    } else if value < target {
        value..target
    } else {
        target.clone()..target
    }
}

pub async fn point_to_point_direction_with_speed(p0: Vec2, p1: Vec2, speed: f32) -> Vec2 {
    let diff_vec = p1 - p0;
    let vec =diff_vec.normalize();
    vec * speed
}

pub async fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}

pub async fn vec2_to_degrees(vec: Vec2) -> f32 {
    let angle_radians = vec.y.atan2(vec.x);
    angle_radians * (360.0 / PI)
}
