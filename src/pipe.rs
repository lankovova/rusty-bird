use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Pipe {
    pub id: u32,
    pub x: f32,
    pub width: f32,
    pub gap_start_y: f32,
    pub gap_size: f32,
    pub has_scored: bool,
    pub direction: i8, // -1 (up), 1 (down) or 0 (static)
    pub moving_speed: f32,
}

impl Pipe {
    pub fn update(&mut self) {
        self.x -= 4.0;

        if self.direction != 0 {
            let new_gap_start = self.gap_start_y + self.moving_speed * (self.direction as f32);

            if new_gap_start < 10.0 || new_gap_start > screen_height() - self.gap_size - 10.0 {
                self.direction = -self.direction;
            }

            self.gap_start_y = new_gap_start.clamp(10.0, screen_height() - self.gap_size - 10.0);
        }
    }

    pub fn is_gone_offscreen(&self) -> bool {
        self.x < -self.width
    }
}
