use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Pipe {
    pub id: u32,
    pub x: f32,
    pub width: f32,
    pub gap_start_y: f32,
    pub gap_size: f32,
    pub has_scored: bool,
}

impl Pipe {
    pub fn update(&mut self) {
        self.x -= 3.0;
    }

    pub fn is_gone(&self) -> bool {
        self.x < -self.width
    }
}
