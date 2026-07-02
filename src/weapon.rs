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
                bullets.push(Bullet {
                    x,
                    y,
                    radius: 5.0,
                    damage: 1.0,
                    is_me: true,
                    is_actve: true,
                    bullet_type: BulletType::RumiaStraight {
                        angle: rand::gen_range(-0.05, 0.05),
                    },
                    history: VecDeque::new(),
                    max_trail_length: 8,
                });
            }
            WeaponType::Tracking => {
                bullets.push(Bullet {
                    x,
                    y,
                    radius: 5.0,
                    damage: 1.0,
                    is_me: true,
                    is_actve: true,
                    bullet_type: BulletType::RumiaStraight { angle: 0.0 },
                    history: VecDeque::new(),
                    max_trail_length: 8,
                });
            }
        }
        Some(bullets)
    }
}
