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
        self.x -= 1.5;
    }

    pub fn is_gone(&self) -> bool {
        self.x < -self.width
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, 0.0, self.width, self.gap_start_y, GREEN);

        draw_rectangle(
            self.x,
            self.gap_start_y + self.gap_size,
            self.width,
            screen_height() - self.gap_start_y,
            GREEN,
        );
    }
}
