use crate::{bird::Bird, pipe::Pipe};

// TODO: Do proper collision
// rn treating bird as a square
pub fn is_bird_colliding_with_pipe(bird: &Bird, pipe: &Pipe) -> bool {
    let r = bird.radius;

    let left = bird.x - r;
    let right = bird.x + r;
    let top = bird.y - r;
    let bottom = bird.y + r;

    // outside pipe horizontally → no collision
    if right < pipe.x || left > pipe.x + pipe.width {
        return false;
    }

    // inside gap → no collision
    let in_gap = bottom < pipe.gap_start_y + pipe.gap_size && top > pipe.gap_start_y;

    if in_gap {
        return false;
    }

    true
}
