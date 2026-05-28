use crate::Pipe;
use macroquad::prelude::*;

const GRAVITY: f32 = 0.05;

struct Point {
    x: f32,
    y: f32,
}

pub struct Bird {
    pos: Point,
    radius: f32,
    velocity: f32,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            pos: Point { x: 300.0, y: 200.0 },
            velocity: 0.0,
            radius: 20.0,
        }
    }

    pub fn flap(&mut self) {
        self.velocity = -4.0;
    }

    pub fn update(&mut self) {
        self.velocity += GRAVITY;
        self.pos.y += self.velocity;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, YELLOW);
    }

    // TODO: Do proper collision
    // rn treating bird as a square
    pub fn is_colliding_with_pipe(&self, pipe: &Pipe) -> bool {
        let leftmost = self.pos.x;
        let rightmost = self.pos.x + self.radius * 2.0;

        if leftmost < pipe.x || rightmost > pipe.x + pipe.width {
            return false;
        }

        let topmost = self.pos.y;
        let bottommost = self.pos.y + self.radius * 2.0;

        if topmost > pipe.gap_start_y && bottommost < pipe.gap_start_y + pipe.gap_size {
            return false;
        }

        return true;
    }

    pub fn is_off_screen(&self) -> bool {
        let topmost = self.pos.y;
        let bottommost = self.pos.y + self.radius * 2.0;

        if topmost < 0.0 || bottommost > screen_height() {
            return true;
        }

        let leftmost = self.pos.x;
        let rightmost = self.pos.x + self.radius * 2.0;

        if leftmost < 0.0 || rightmost > screen_width() {
            return true;
        }

        return false;
    }
}
