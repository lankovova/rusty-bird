mod bird;
mod pipe;

use crate::bird::Bird;
use crate::pipe::Pipe;
use macroquad::prelude::*;

// draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
// draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
// draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

const FIXED_DT: f32 = 1.0 / 60.0;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut bird = Bird::new();
    let mut pipes_pool: Vec<Pipe> = vec![];
    let mut last_time_pipe_spawned = -100.0;
    let mut game_over = false;
    let mut accumulator = 0.0;

    // Skip first frame because screen dimensions are wrong on the first pass
    // probably because of auto resize that happens when user uses WM
    next_frame().await;

    loop {
        let current_time = get_time();
        let dt = get_frame_time();

        accumulator += dt;

        pipes_pool.retain(|pipe| !pipe.is_gone());

        if current_time - last_time_pipe_spawned > 6.0 {
            last_time_pipe_spawned = current_time;

            let new_pipe = Pipe::new();
            println!("pipe spawned at {}, {:?}", current_time, new_pipe);

            pipes_pool.push(new_pipe);
        }

        if is_key_released(KeyCode::Space) {
            bird.flap();
        }

        // Unties physics from FPS
        // FIXME: Feels like shit, wasted renders with no changes
        while accumulator >= FIXED_DT {
            bird.update();
            pipes_pool.iter_mut().for_each(|pipe| pipe.update());

            accumulator -= FIXED_DT;
        }

        for pipe in &pipes_pool {
            // FIXME: Could optimize by checking collision only with the closest pipe
            if bird.is_colliding_with_pipe(pipe) {
                game_over = true;
            }
        }

        if bird.is_off_screen() {
            game_over = true;
        }

        clear_background(Color::new(0.0, 0.5, 0.5, 1.0));

        bird.draw();
        pipes_pool.iter_mut().for_each(|pipe| pipe.draw());

        // TODO: Stop physics on game over
        if game_over {
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

        next_frame().await
    }
}
