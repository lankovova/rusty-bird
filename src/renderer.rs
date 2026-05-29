use macroquad::color::{GREEN, YELLOW};
use macroquad::shapes::{draw_circle, draw_rectangle};
use macroquad::window::screen_height;

use crate::bird::Bird;
use crate::pipe::Pipe;

pub struct Renderer;

impl Renderer {
    pub fn render_bird(bird: &Bird) {
        draw_circle(bird.pos.x, bird.pos.y, bird.radius, YELLOW);
    }

    pub fn render_pipe(pipe: &Pipe) {
        draw_rectangle(pipe.x, 0.0, pipe.width, pipe.gap_start_y, GREEN);

        draw_rectangle(
            pipe.x,
            pipe.gap_start_y + pipe.gap_size,
            pipe.width,
            screen_height() - pipe.gap_start_y,
            GREEN,
        );
    }
}
