use macroquad::{prelude::*, rand};

use crate::bird::Bird;
use crate::particles::Particle;
use crate::pipe::Pipe;
use crate::world::World;

const MAX_SHAKE_IN_PIXELS: f32 = 40.0;
const MAX_BIRD_UP_ANGLE: f32 = -1.0;
const MAX_BIRD_DOWN_ANGLE: f32 = 1.0;

struct Flash {
    life: f32,
    max_life: f32,
    color: Color,
}

impl Flash {
    fn new() -> Self {
        Self {
            life: 0.0,
            max_life: 0.0,
            color: WHITE,
        }
    }
}

pub struct Renderer {
    shake_offset: Vec2,
    shake_trauma: f32, // 0.0 = no shake, 1.0 = max shake
    flash: Flash,
    bird_texture: Texture2D,
}

impl Renderer {
    pub fn new(bird_texture: Texture2D) -> Self {
        Self {
            shake_offset: vec2(0.0, 0.0),
            shake_trauma: 0.0,
            flash: Flash::new(),
            bird_texture,
        }
    }

    pub fn add_flash(&mut self, life: f32, color: Color) {
        self.flash = Flash {
            life,
            max_life: life,
            color,
        };
    }

    pub fn add_trauma(&mut self, amount: f32) {
        self.shake_trauma = (self.shake_trauma + amount).min(1.0);
    }

    pub fn update(&mut self, dt: f32) {
        self.shake_trauma = (self.shake_trauma - 0.5 * dt).max(0.0);

        let magnitude = self.shake_trauma * self.shake_trauma;

        self.shake_offset = vec2(
            rand::gen_range(-1.0, 1.0) * magnitude * MAX_SHAKE_IN_PIXELS,
            rand::gen_range(-1.0, 1.0) * magnitude * MAX_SHAKE_IN_PIXELS,
        );

        if self.flash.life > 0.0 {
            self.flash.life = (self.flash.life - dt).max(0.0);
        }
    }

    pub fn render_world(&self, world: &World, prev_world: &World, alpha: f32) {
        clear_background(Color::new(0.0, 0.5, 0.5, 1.0));

        if !world.game_over {
            self.render_bird(&world.bird, &prev_world.bird, alpha);
        }

        world.pipes.iter().for_each(|pipe| {
            let prev_pipe = prev_world
                .pipes
                .iter()
                .find(|p| p.id == pipe.id)
                .unwrap_or(pipe);

            self.render_pipe(pipe, prev_pipe, alpha);
        });

        self.draw_particles(&world.particles);
        self.draw_falsh();

        if world.game_over {
            draw_text(
                format!("You ded.\nScore {}!", world.score),
                screen_width() / 2.0 - 200.0 + self.shake_offset.x,
                screen_height() / 2.0 + self.shake_offset.y,
                60.0,
                BLACK,
            );
            draw_text(
                "[R] to restart",
                screen_width() / 2.0 - 150.0 + self.shake_offset.x,
                screen_height() / 2.0 + 50.0 + self.shake_offset.y,
                50.0,
                BLACK,
            );
        }

        let fps_counter_str = format!("FPS {}", get_fps());
        draw_text(fps_counter_str, screen_width() - 100.0, 20.0, 20.0, BLACK);

        let score_count_str = format!("Score: {}", world.score);
        draw_text(
            score_count_str,
            20.0 + self.shake_offset.x,
            40.0 + self.shake_offset.y,
            40.0,
            GOLD,
        );
    }

    pub fn render_bird(&self, bird: &Bird, prev_bird: &Bird, alpha: f32) {
        let y = prev_bird.y.lerp(bird.y, alpha);

        draw_circle(
            bird.x + self.shake_offset.x,
            y + self.shake_offset.y,
            bird.radius,
            YELLOW,
        );

        let rotation = bird
            .velocity
            .clamp(-13.0, 15.0) // prevent insane values
            .remap(-13.0, 15.0, MAX_BIRD_UP_ANGLE, MAX_BIRD_DOWN_ANGLE);

        let size = vec2(32.0 * 1.5, 29.0 * 1.5);
        draw_texture_ex(
            &self.bird_texture,
            bird.x - size.x / 2.0,
            y - size.y / 2.0 - 2.0,
            WHITE,
            DrawTextureParams {
                rotation: rotation,
                dest_size: Some(size),
                pivot: Some(vec2(bird.x, y)),
                ..Default::default()
            },
        );
    }

    pub fn render_pipe(&self, pipe: &Pipe, prev_pipe: &Pipe, alpha: f32) {
        let x = prev_pipe.x.lerp(pipe.x, alpha);

        draw_rectangle(
            x + self.shake_offset.x,
            0.0 + self.shake_offset.y,
            pipe.width,
            pipe.gap_start_y,
            GREEN,
        );
        draw_rectangle(
            x + self.shake_offset.x,
            pipe.gap_start_y + pipe.gap_size + self.shake_offset.y,
            pipe.width,
            screen_height() - pipe.gap_start_y,
            GREEN,
        );
    }

    pub fn draw_particles(&self, particles: &Vec<Particle>) {
        for p in particles {
            let t = p.life / p.max_life;
            let alpha = t.clamp(0.0, 1.0);
            let color = Color::new(p.color.r, p.color.g, p.color.b, alpha);

            draw_rectangle(
                p.pos.x + self.shake_offset.x,
                p.pos.y + self.shake_offset.y,
                p.size * t,
                p.size * t,
                color,
            );
        }
    }

    fn draw_falsh(&self) {
        if self.flash.life <= 0.0 {
            return;
        }

        let t = self.flash.life / (4.0 * self.flash.max_life);
        let alpha = t.clamp(0.0, 1.0);
        let color = self.flash.color.with_alpha(alpha);

        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), color);
    }
}
