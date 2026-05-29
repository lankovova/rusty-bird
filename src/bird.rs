use macroquad::prelude::*;

const GRAVITY: f32 = 0.2;

#[derive(Clone)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub velocity: f32,
}

impl Bird {
    pub fn flap(&mut self) {
        self.velocity = -8.0;
    }

    pub fn update(&mut self) {
        self.velocity += GRAVITY;
        self.y += self.velocity;
    }

    pub fn is_off_screen(&self) -> bool {
        let topmost = self.y;
        let bottommost = self.y + self.radius * 2.0;

        if topmost < -200.0 || bottommost > screen_height() {
            return true;
        }

        let leftmost = self.x;
        let rightmost = self.x + self.radius * 2.0;

        if leftmost < 0.0 || rightmost > screen_width() {
            return true;
        }

        return false;
    }
}
