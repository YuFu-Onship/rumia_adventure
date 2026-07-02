use macroquad::prelude::*;

use crate::get_color;

pub trait EnemyTrait {
    fn update(&mut self, dt: f32);
    fn draw(&self);
    fn is_dead(&self) -> bool;
    fn position(&self) -> (f32, f32);
    fn check_collision(&mut self, x: f32, y: f32, radius: f32, demage: f32) -> bool;
}

// 小妖精 ------------------------------------------
pub struct OrdinaryFairy {
    pub hp: f32,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub speed: f32,
}

impl OrdinaryFairy {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            hp: 10.0,
            x,
            y,
            radius: 15.0,
            speed: 50.0,
        }
    }
}

impl EnemyTrait for OrdinaryFairy {
    fn update(&mut self, dt: f32) {
        self.x += self.speed * dt;
        if self.x > screen_width() || self.x < 0.0 {
            self.speed = -self.speed;
        }
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, 15.0, GREEN);
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }

    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    fn check_collision(&mut self, x: f32, y: f32, radius: f32, demage: f32) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        let dis = dx * dx + dy * dy;
        if dis < (self.radius + radius) * (self.radius + radius) {
            self.hp -= demage;
            return true;
        }
        false
    }
}

// 沙包 -----------------------------------------------------
pub struct Punching {
    pub hp: f32,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub speed: f32,
}
impl Punching {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            hp: 10000.0,
            x,
            y,
            radius: 10.0,
            speed: 50.0,
        }
    }
}
impl EnemyTrait for Punching {
    fn update(&mut self, _dt: f32) {
        self.hp = (self.hp + 100.0).min(10000.0);
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, get_color("white"));
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }

    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    fn check_collision(&mut self, x: f32, y: f32, radius: f32, demage: f32) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        let dis = dx * dx + dy * dy;
        if dis < (self.radius + radius) * (self.radius + radius) {
            self.hp -= demage;
            return true;
        }
        false
    }
}
