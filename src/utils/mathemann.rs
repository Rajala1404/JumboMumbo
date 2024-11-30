use crate::utils::structs::Rect;

// Powered by 100% Vegan MatheMANN™

pub async fn stretch_float_to(f: f32, max_size: f32, target_f: f32) -> f32 {
    f * target_f / max_size
}

/// Checks if on [Rect] overlaps with another [Rect]
pub async fn rect_overlap_rect(rect_0: &Rect, rect_1: &Rect) -> bool {
    // Calculates the bottom right of each rect to compare if one of 4 corners is inside the provided rect
        !(rect_0.x >= (rect_1.x + rect_1.w) ||
        rect_0.y >= (rect_1.y + rect_1.h) ||
        (rect_0.x + rect_0.w) <= rect_1.x ||
        (rect_0.y + rect_0.h) <= rect_1.y)
}