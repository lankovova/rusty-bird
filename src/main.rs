use macroquad::prelude::*;

struct Point {
    x: f32,
    y: f32,
}

const GRAVITY: f32 = 0.05;

struct Bird {
    pos: Point,
    radius: f32,
    velocity: f32,
}

impl Bird {
    fn new() -> Self {
        Self {
            pos: Point { x: 300.0, y: 200.0 },
            velocity: 0.0,
            radius: 20.0,
        }
    }

    fn flap(&mut self) {
        self.velocity = -4.0;
    }

    fn update(&mut self) {
        self.velocity += GRAVITY;
        self.pos.y += self.velocity;
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, YELLOW);
    }

    // TODO: Do proper collision
    // rn treating bird as a square
    fn is_colliding_with_pipe(&self, pipe: &Pipe) -> bool {
        let leftmost = self.pos.x;
        let rightmost = self.pos.x + self.radius * 2.0;

        if leftmost < pipe.x || rightmost > pipe.x + pipe.width {
            return false;
        }

        let topmost = self.pos.y;
        let bottommost = self.pos.y + self.radius * 2.0;

        if topmost > pipe.gap_start_y && bottommost < pipe.gap_start_y + pipe.gap_size {
            return false;
        }

        return true;
    }

    fn is_off_screen(&self) -> bool {
        let topmost = self.pos.y;
        let bottommost = self.pos.y + self.radius * 2.0;

        if topmost < 0.0 || bottommost > screen_height() {
            return true;
        }

        let leftmost = self.pos.x;
        let rightmost = self.pos.x + self.radius * 2.0;

        if leftmost < 0.0 || rightmost > screen_width() {
            return true;
        }

        return false;
    }
}

#[derive(Debug)]
struct Pipe {
    x: f32,
    width: f32,
    gap_start_y: f32,
    gap_size: f32,
}

impl Pipe {
    fn new() -> Self {
        Self {
            x: screen_width(),
            width: 150.0,
            gap_start_y: 400.0,
            gap_size: 300.0,
        }
    }

    fn update(&mut self) {
        self.x -= 1.5;
    }

    fn is_gone(&self) -> bool {
        self.x < -self.width
    }

    fn draw(&self) {
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

// draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
// draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
// draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

const FIXED_DT: f64 = 1.0 / 60.0;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut bird = Bird::new();
    let mut pipes_pool: Vec<Pipe> = vec![];
    let mut last_time_pipe_spawned = -100.0;
    let mut game_over = false;
    let mut accumulator = 0.0;
    let mut prev_time = 0.0;

    next_frame().await;

    loop {
        let current_time = get_time();
        let dt = current_time - prev_time;

        accumulator += dt;

        pipes_pool.retain(|pipe| !pipe.is_gone());

        clear_background(Color::new(0.0, 0.5, 0.5, 1.0));

        let fps_counter_str = format!("FPS {}", get_fps());
        draw_text(fps_counter_str, screen_width() - 120.0, 20.0, 30.0, BLACK);

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
            pipes_pool.iter_mut().for_each(|pipe| {
                pipe.update();
            });

            accumulator -= FIXED_DT;
        }

        bird.draw();
        pipes_pool.iter_mut().for_each(|pipe| {
            pipe.draw();
        });

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

        for pipe in &pipes_pool {
            if bird.is_colliding_with_pipe(pipe) {
                game_over = true;
            }
        }

        if bird.is_off_screen() {
            game_over = true;
        }

        prev_time = current_time;

        next_frame().await
    }
}
