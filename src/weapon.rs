use crate::{Bullet, BulletType};
use macroquad::prelude::*;
use std::collections::VecDeque;

pub enum WeaponType {
    Straight,
    Tracking,
}

pub struct Weapon {
    pub weapon_type: WeaponType,
    pub cold_timer: f32,
    pub cold_limit: f32,
}

impl Weapon {
    pub fn new(weapon_type: WeaponType, cold_limit: f32) -> Self {
        Self {
            weapon_type,
            cold_limit,
            cold_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.cold_timer > 0.0 {
            self.cold_timer -= dt;
        }
    }
    pub fn fire(&mut self, x: f32, y: f32) -> Option<Vec<Bullet>> {
        if self.cold_timer > 0.0 {
            return None;
        }

        self.cold_timer = self.cold_limit;
        let mut bullets = Vec::new();
        match self.weapon_type {
            WeaponType::Straight => {
                let arc: f32 = 0.0;
                for i in 0..2 {
                    bullets.push(Bullet {
                        pos: vec2(x, y - 5.0 + 10.0 * (i) as f32),
                        v: vec2(arc.cos(), arc.sin()) * 600.0,
                        a: vec2(0.0, 0.0),
                        radius: 5.0,
                        damage: 1.0,
                        is_me: true,
                        is_actve: true,
                        timer: 0.0,
                        bullet_type: BulletType::RumiaStraight { angle: arc },
                        history: VecDeque::new(),
                        max_trail_length: 8,
                    });
                }
            }
            WeaponType::Tracking => {
                let arc: f32 = rand::gen_range(-0.5, 0.5);
                bullets.push(Bullet {
                    pos: vec2(x, y),
                    v: vec2(arc.cos(), arc.sin()) * 600.0,
                    a: vec2(0.0, 0.0),
                    radius: 5.0,
                    damage: 1.0,
                    is_me: true,
                    is_actve: true,
                    timer: 0.0,
                    bullet_type: BulletType::RumiaTracking { angle: arc },
                    history: VecDeque::new(),
                    max_trail_length: 12,
                });
            }
        }
        Some(bullets)
    }
}
