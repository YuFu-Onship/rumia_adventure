use macroquad::prelude::*;

// 粒子系统 --------------------------------------------------------------------
pub struct ParticleSystem {
    particles: Vec<CircleParticle>,
}
impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        let mut particles = Vec::with_capacity(max_particles);
        for _ in 0..max_particles {
            particles.push(CircleParticle {
                pos: Vec2::ZERO,
                v: Vec2::ZERO,
                a: Vec2::ZERO,
                s: 1.0,
                sc: 0.3,
                radius: 0.0,
                is_active: false,
                is_gravity: false,
                color: WHITE,
            });
        }
        ParticleSystem { particles }
    }

    pub fn emit(&mut self, params: ParticleParams) {
        if let Some(p) = self.particles.iter_mut().find(|p| !p.is_active) {
            p.pos = params.pos;
            p.v = params.v;
            p.a = params.a;
            p.s = 1.0;
            p.sc = params.scale_coe;
            p.radius = params.radius;
            p.is_active = true;
            p.is_gravity = params.is_gravity;
            p.color = params.color;
        }
    }
    pub fn update(&mut self, dt: f32) {
        for p in self.particles.iter_mut() {
            if p.is_active {
                if p.is_gravity {
                    p.v.y += 100.0 * dt;
                }
                p.update(dt);
            }
        }
    }
    pub fn draw(&self) {
        for p in self.particles.iter() {
            if p.is_active {
                p.draw();
            }
        }
    }
}

// 基础圆形粒子 ----------------------------------------------------------------
pub struct ParticleParams {
    pub pos: Vec2,
    pub v: Vec2,
    pub a: Vec2,
    pub radius: f32,
    pub scale_coe: f32,
    pub is_gravity: bool,
    pub color: Color,
}
pub struct CircleParticle {
    pub pos: Vec2,
    pub v: Vec2,
    pub a: Vec2,
    pub s: f32,
    pub sc: f32,
    pub radius: f32,
    pub color: Color,
    pub is_active: bool,
    pub is_gravity: bool,
}

impl CircleParticle {
    pub fn update(&mut self, dt: f32) {
        self.s -= self.s * self.sc;
        if self.s <= 0.01 {
            self.is_active = false;
            return;
        }
        if self.s * self.radius < 1.0 {
            self.is_active = false
        }
        self.v += self.a * dt;
        self.pos += self.v * dt;
    }
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius * self.s, self.color);
    }
}
