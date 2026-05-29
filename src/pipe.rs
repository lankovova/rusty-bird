use macroquad::prelude::*;

#[derive(Debug)]
pub struct Pipe {
    pub x: f32,
    pub width: f32,
    pub gap_start_y: f32,
    pub gap_size: f32,
}

impl Pipe {
    pub fn new() -> Self {
        Self {
            x: screen_width(),
            width: 150.0,
            gap_start_y: 400.0,
            gap_size: 300.0,
        }
    }

    pub fn update(&mut self) {
        self.x -= 3.0;
    }

    pub fn is_gone(&self) -> bool {
        self.x < -self.width
    }
}
