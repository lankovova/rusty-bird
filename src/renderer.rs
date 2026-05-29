use macroquad::prelude::*;

use crate::bird::Bird;
use crate::pipe::Pipe;
use crate::world::World;

pub struct Renderer;

impl Renderer {
    pub fn render_world(world: &World) {
        clear_background(Color::new(0.0, 0.5, 0.5, 1.0));

        Renderer::render_bird(&world.bird);

        world
            .pipes
            .iter()
            .for_each(|pipe| Renderer::render_pipe(pipe));

        // TODO: Stop physics on game over
        if world.game_over {
            draw_text(
                "You ded",
                screen_width() / 2.0 - 120.0,
                screen_height() / 2.0 - 100.0,
                60.0,
                BLACK,
            );
        }

        let fps_counter_str = format!("FPS {}", get_fps());
        draw_text(fps_counter_str, screen_width() - 120.0, 20.0, 30.0, BLACK);
    }

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
