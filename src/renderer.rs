use macroquad::prelude::*;

use crate::bird::Bird;
use crate::pipe::Pipe;
use crate::world::World;

pub struct Renderer;

impl Renderer {
    pub fn render_world(world: &World, prev_world: &World, alpha: f32) {
        clear_background(Color::new(0.0, 0.5, 0.5, 1.0));

        Renderer::render_bird(&world.bird, &prev_world.bird, alpha);

        world.pipes.iter().for_each(|pipe| {
            // FIXME: Could be optimized because pipes have uniq ids and they are sorted in the vec
            let prev_pipe = prev_world
                .pipes
                .iter()
                .find(|p| p.id == pipe.id)
                .unwrap_or(pipe);

            Renderer::render_pipe(pipe, prev_pipe, alpha);
        });

        // FIXME: Stop physics on game over
        if world.game_over {
            draw_text(
                "You ded",
                screen_width() / 2.0 - 120.0,
                screen_height() / 2.0,
                60.0,
                BLACK,
            );
        }

        let fps_counter_str = format!("FPS {}", get_fps());
        draw_text(fps_counter_str, screen_width() - 120.0, 20.0, 30.0, BLACK);
    }

    pub fn render_bird(bird: &Bird, prev_bird: &Bird, alpha: f32) {
        let y = prev_bird.y.lerp(bird.y, alpha);
        draw_circle(bird.x, y, bird.radius, YELLOW);
    }

    pub fn render_pipe(pipe: &Pipe, prev_pipe: &Pipe, alpha: f32) {
        let x = prev_pipe.x.lerp(pipe.x, alpha);

        draw_rectangle(x, 0.0, pipe.width, pipe.gap_start_y, GREEN);

        draw_rectangle(
            x,
            pipe.gap_start_y + pipe.gap_size,
            pipe.width,
            screen_height() - pipe.gap_start_y,
            GREEN,
        );
    }
}
