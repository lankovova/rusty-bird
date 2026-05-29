use crate::{bird::Bird, collider, pipe::Pipe};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct World {
    pub bird: Bird,
    pub pipes: Vec<Pipe>,
    pub game_over: bool,
    last_time_pipe_spawned: f32,
    next_pipe_id: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            bird: Bird::new(),
            pipes: vec![],
            game_over: false,
            last_time_pipe_spawned: -100.0,
            next_pipe_id: 0,
        }
    }

    pub fn process(&mut self, current_time: f32) {
        self.pipes.retain(|pipe| !pipe.is_gone());

        if current_time - self.last_time_pipe_spawned > 3.0 {
            self.last_time_pipe_spawned = current_time;
            self.spawn_pipe();
        }

        if is_key_released(KeyCode::Space) {
            self.bird.flap();
        }

        if self.bird.is_off_screen() {
            self.game_over = true;
        }
    }

    pub fn update(&mut self) {
        self.bird.update();
        self.pipes.iter_mut().for_each(|p| p.update());

        self.check_collisions();
    }

    fn check_collisions(&mut self) {
        // FIXME: Could optimize by checking collision only with the closest pipe
        for pipe in &self.pipes {
            if collider::collide(&self.bird, pipe) {
                self.game_over = true;
            }
        }
    }

    fn spawn_pipe(&mut self) {
        let new_pipe = Pipe::new(self.next_pipe_id);

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
