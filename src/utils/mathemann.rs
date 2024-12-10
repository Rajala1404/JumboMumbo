// Powered by 100% Vegan MatheMANN™

use std::ops::Range;

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