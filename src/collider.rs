use crate::{bird::Bird, pipe::Pipe};

// TODO: Do proper collision
// rn treating bird as a square
pub fn collide(bird: &Bird, pipe: &Pipe) -> bool {
    let leftmost = bird.pos.x;
    let rightmost = bird.pos.x + bird.radius * 2.0;

    if leftmost < pipe.x || rightmost > pipe.x + pipe.width {
        return false;
    }

    let topmost = bird.pos.y;
    let bottommost = bird.pos.y + bird.radius * 2.0;

    if topmost > pipe.gap_start_y && bottommost < pipe.gap_start_y + pipe.gap_size {
        return false;
    }

    return true;
}
