use crate::{
    bird::Bird,
    collider,
    particles::{self, Particle},
    pipe::Pipe,
    renderer::Renderer,
};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct World {
    pub bird: Bird,
    pub pipes: Vec<Pipe>,
    pub particles: Vec<Particle>,
    pub game_over: bool,
    pub score: u32,
    last_time_pipe_spawned: f32,
    next_pipe_id: u32,
}

impl World {
    pub fn new() -> Self {
        let bird = Bird {
            x: 300.0,
            y: 200.0,
            velocity: 0.0,
            radius: 20.0,
        };

        Self {
            bird,
            pipes: vec![],
            particles: vec![], // FIXME: Could optimize by using pre allocated space array
            game_over: false,
            score: 0,
            last_time_pipe_spawned: -100.0,
            next_pipe_id: 0,
        }
    }

    pub fn process(&mut self, current_time: f32, renderer: &mut Renderer) {
        if self.game_over {
            return;
        }

        if current_time - self.last_time_pipe_spawned > 2.0 {
            self.last_time_pipe_spawned = current_time;
            self.spawn_pipe();
        }

        if is_key_released(KeyCode::Space) {
            self.bird.flap();
        }

        if self.bird.is_off_screen() {
            self.game_over = true;
            self.kill_the_bird(renderer);
        }
    }

    pub fn update(&mut self, dt: f32, renderer: &mut Renderer) {
        particles::update_particles(&mut self.particles, dt);

        if self.game_over {
            return;
        }

        self.bird.update();
        self.pipes.iter_mut().for_each(|p| p.update());

        self.pipes.retain(|pipe| !pipe.is_gone());

        let is_bird_hit_the_pipe = self.is_bird_colliding_with_pipe();
        if is_bird_hit_the_pipe {
            self.game_over = true;
            self.kill_the_bird(renderer);
        }

        // Increase score for each passed pipe
        if !self.game_over {
            for pipe in &mut self.pipes {
                if !pipe.has_scored && self.bird.x - self.bird.radius > pipe.x + pipe.width {
                    self.score += 1;
                    pipe.has_scored = true;

                    // Flash as a passing pipe effect
                    renderer.add_flash(0.5, GOLD);
                }
            }
        }
    }

    fn kill_the_bird(&mut self, renderer: &mut Renderer) {
        particles::spawn_bird_death_explosion(
            vec2(self.bird.x, self.bird.y),
            &mut self.particles,
            vec2(8.0, self.bird.velocity),
        );

        renderer.add_trauma(0.5);
    }

    fn is_bird_colliding_with_pipe(&mut self) -> bool {
        for pipe in &self.pipes {
            if collider::is_bird_colliding_with_pipe(&self.bird, pipe) {
                return true;
            }
        }

        return false;
    }

    fn spawn_pipe(&mut self) {
        let gap_start_y = rand::gen_range(screen_height() * 0.1, screen_height() * 0.6);

        let new_pipe = Pipe {
            id: self.next_pipe_id,
            x: screen_width(),
            width: 100.0,
            gap_start_y: gap_start_y,
            gap_size: 300.0,
            has_scored: false,
        };

        debug!(
            "{} pipe #{} spawned at x={}",
            get_time(),
            new_pipe.id,
            new_pipe.x
        );

        self.next_pipe_id += 1;
        self.pipes.push(new_pipe);
    }
}
