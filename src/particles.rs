use macroquad::{
    color::{Color, ORANGE, RED, YELLOW},
    math::{Vec2, vec2},
    rand,
};

const BIRD_PARTICLE_VELOCITY_INFLUENCE: f32 = 50.0;

#[derive(Clone)]
pub struct Particle {
    pub pos: Vec2,
    vel: Vec2,
    pub life: f32,
    pub max_life: f32,
    pub color: Color,
    pub size: f32,
}

fn spawn_explosion(
    pos: Vec2,
    particles: &mut Vec<Particle>,
    max_life: f32,
    count: i32,
    color: Color,
    speed_min: f32,
    speed_max: f32,
    influence: Vec2,
    influence_strength: f32,
    size: f32,
) {
    for _ in 0..count {
        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let speed = rand::gen_range(speed_min, speed_max);

        let random_dir = vec2(angle.cos(), angle.sin()) * speed;
        let velocity_influence = influence * influence_strength;

        let particle = Particle {
            pos,
            vel: random_dir + velocity_influence,
            life: max_life,
            max_life,
            color,
            size,
        };

        particles.push(particle);
    }
}

pub fn spawn_bird_death_explosion(pos: Vec2, particles: &mut Vec<Particle>, bird_vel: Vec2) {
    spawn_explosion(
        pos,
        particles,
        3.0,
        100,
        YELLOW,
        400.0,
        1000.0,
        bird_vel,
        BIRD_PARTICLE_VELOCITY_INFLUENCE,
        5.0,
    );
    spawn_explosion(
        pos,
        particles,
        3.0,
        100,
        ORANGE,
        300.0,
        600.0,
        bird_vel,
        BIRD_PARTICLE_VELOCITY_INFLUENCE,
        8.0,
    );
    spawn_explosion(
        pos,
        particles,
        2.0,
        50,
        RED,
        200.0,
        400.0,
        bird_vel,
        BIRD_PARTICLE_VELOCITY_INFLUENCE,
        10.0,
    );
}

pub fn update_particles(particles: &mut Vec<Particle>, dt: f32) {
    for p in particles.iter_mut() {
        p.life -= dt;
        p.pos += p.vel * dt;

        // Gravity feel
        p.vel.y += 2000.0 * dt;
    }

    particles.retain(|p| p.life > 0.0);
}
