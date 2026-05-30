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

#[macroquad::main("Rusty Bird")]
async fn main() {
    let mut world = World::new();
    let mut prev_world: World = world.clone();
    let mut accumulator = 0.0;
    let mut current_time = 0.0;

    // Skip first frame because screen dimensions are wrong on the first pass
    // probably because of auto resize that happens when user uses WM
    next_frame().await;

    loop {
        let dt = get_frame_time();
        current_time += dt;
        accumulator += dt;

        if is_key_released(KeyCode::R) {
            world = World::new();
            prev_world = world.clone();
            accumulator = 0.0;
        }

        world.process(current_time);

        while accumulator >= FIXED_DT {
            prev_world = world.clone();

            world.update();
            accumulator -= FIXED_DT;
        }

        let alpha = accumulator / FIXED_DT;

        Renderer::render_world(&world, &prev_world, alpha);

        next_frame().await
    }
}
