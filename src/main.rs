mod bird;
mod collider;
mod pipe;
mod renderer;
mod world;

use crate::renderer::Renderer;
use macroquad::prelude::*;
use world::World;

// draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
// draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
// draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

const FIXED_DT: f32 = 1.0 / 60.0;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut world = World::new();
    let mut accumulator = 0.0;
    let mut current_time: f32 = 0.0;

    // Skip first frame because screen dimensions are wrong on the first pass
    // probably because of auto resize that happens when user uses WM
    next_frame().await;

    loop {
        let dt = get_frame_time();
        current_time += dt;
        accumulator += dt;

        world.process(current_time);

        // Unties physics from FPS
        // FIXME: Feels like shit, wasted renders with no changes
        while accumulator >= FIXED_DT {
            world.update();
            accumulator -= FIXED_DT;
        }

        Renderer::render_world(&world);

        next_frame().await
    }
}
