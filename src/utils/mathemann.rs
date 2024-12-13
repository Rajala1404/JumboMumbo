use std::ops::Range;
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

pub async fn point_to_point_direction_with_speed(p0: (f32, f32), p1: (f32, f32), speed: f32) -> (f32, f32) {
    let dx = p0.0 - p1.0;
    let dy = p0.1 - p1.1;
    let magnitude = (dx * dx + dy * dy).sqrt();

    ((dx / magnitude) * speed.clone(), (dy / magnitude) * speed)
}