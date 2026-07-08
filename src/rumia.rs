use macroquad::prelude::*;

use crate::Bullet;
use crate::get_color;
use crate::weapon::{Weapon, WeaponType};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)] // 特征
pub struct Trajectory {
    x: f32,
    y: f32,
}

pub struct Rumia {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub speed: f32,
    pub hp: f32,
    pub a: f32,
    pub v: f32,

    traje: [Trajectory; 30],

    texture: Texture2D,
    sprite_rect: HashMap<String, Rect>,
    sprite_idx: u32,
    sprite_list: Vec<String>,

    pub weapons: Vec<Weapon>,
    pub weapon_idx: usize,

    animation_timer: f32,
    animation_speed: f32,
    animation_idx: usize,
    animation_frame: String,
    state: String,
}

impl Rumia {
    pub fn new(x: f32, y: f32, texture: Texture2D) -> Self {
        Self {
            x,
            y,
            texture,
            a: 0.0,
            v: 0.0,
            radius: 6.0,
            speed: 300.0,
            hp: 100.0,
            traje: [Trajectory { x, y }; 30],
            sprite_rect: Self::init_sprite_rect(),
            sprite_idx: 0,
            sprite_list: vec!["N", "NE", "E", "SE", "S", "SW", "W", "NW"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            weapons: vec![
                Weapon::new(WeaponType::Straight, 0.1),
                Weapon::new(WeaponType::Tracking, 0.05),
            ],
            weapon_idx: 0,

            animation_timer: 0.0,
            animation_speed: 0.1,
            animation_idx: 1,
            animation_frame: "mid_1".to_string(),
            state: "mid".to_string(),
        }
    }

    // 更新
    pub fn update(&mut self, dt: f32) {
        let mut vx = 0.0;
        let mut vy = 0.0;
        self.state = "mid".to_string();

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            vy -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            vy += 1.0;
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            vx -= 1.0;
            self.state = "left".to_string();
            self.animation_frame = "left".to_string();
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            vx += 1.0;
            self.state = "right".to_string();
            self.animation_frame = "right".to_string();
        }
        if is_key_down(KeyCode::LeftShift) {
            vx *= 0.3;
            vy *= 0.3;
        }

        if is_key_pressed(KeyCode::Key1) {
            self.weapon_idx = 0
        }
        if is_key_pressed(KeyCode::Key2) {
            self.weapon_idx = 1
        }

        self.x += vx * self.speed * dt;
        self.y += vy * self.speed * dt;

        if self.y <= 6.0 || self.y >= 174.0 {
            self.v = -self.v * 0.6;
            self.a = -self.a * 0.6;
        }

        if self.state == "mid" {
            self.animation_timer += dt;
            if self.animation_timer >= self.animation_speed {
                self.animation_timer = 0.0;
                self.animation_idx += 1;
                if self.animation_idx >= 4 {
                    self.animation_idx = 0;
                }
                match self.animation_idx {
                    0 => self.animation_frame = "mid_1".to_string(),
                    1 => self.animation_frame = "mid_2".to_string(),
                    2 => self.animation_frame = "mid_3".to_string(),
                    3 => self.animation_frame = "mid_4".to_string(),
                    _ => {}
                }
            }
        }

        self.x = self.x.clamp(3.0, 317.0);
        self.y = self.y.clamp(6.0, 174.0);

        for i in (0..self.traje.len()).rev() {
            if i == 0 {
                self.traje[0].x = self.x;
                self.traje[0].y = self.y;
                continue;
            }
            self.traje[i] = self.traje[i - 1];
            if i < 5 {
                let offset = rand::gen_range(-5.0 + i as f32 * 0.5, 5.0 - i as f32 * 0.5);
                let angle = rand::gen_range(0.0, std::f32::consts::PI) as f32;
                self.traje[i].x += offset * angle.cos();
                self.traje[i].y += offset * angle.sin();
            }
            self.traje[i].x -= self.speed * 0.3 * dt;
        }
    }

    //武器更新与开火
    pub fn update_weapons(&mut self, dt: f32) {
        if self.weapons.is_empty() {
            return;
        }
        self.weapons[self.weapon_idx].update(dt);
    }

    // 开火
    pub fn fire(&mut self) -> Option<Vec<Bullet>> {
        if self.weapons.is_empty() {
            return None;
        }
        if is_key_down(KeyCode::J) || is_key_down(KeyCode::Z) {
            return self.weapons[self.weapon_idx].fire(self.x + 10.0, self.y);
        }
        None
    }

    // 绘制
    pub fn draw(&self) {
        for (i, point) in self.traje.iter().enumerate() {
            draw_circle(
                point.x,
                point.y,
                ((self.traje.len() - i) as f32) / self.traje.len() as f32 * 12.0 + 3.0,
                get_color("white"),
            );
        }
        for (i, point) in self.traje.iter().enumerate() {
            draw_circle(
                point.x,
                point.y,
                ((self.traje.len() - i) as f32) / self.traje.len() as f32 * 12.0 + 1.0,
                get_color("black"),
            );
        }

        if let Some(crop_rect) = self
            .sprite_list
            .get(self.sprite_idx as usize)
            .and_then(|sprite_name| self.sprite_rect.get(sprite_name))
            .cloned()
        {
            draw_texture_ex(
                &self.texture,
                self.x - 10.0,
                self.y - 10.0,
                WHITE,
                DrawTextureParams {
                    source: Some(crop_rect),
                    dest_size: Some(vec2(crop_rect.w * 2.0, crop_rect.h * 2.0)),
                    rotation: 0.0,
                    ..Default::default()
                },
            );
        }
        if let Some(crop_rect) = self.sprite_rect.get(&self.animation_frame).cloned() {
            draw_texture_ex(
                &self.texture,
                self.x - 8.0,
                self.y - 8.0,
                WHITE,
                DrawTextureParams {
                    source: Some(crop_rect),
                    ..Default::default()
                },
            );
        }
    }
    pub fn init_sprite_rect() -> HashMap<String, Rect> {
        let mut sprite_rect = HashMap::new();
        let directions = [
            ("mid_1", 0.0, 0.0, 16.0, 16.0),
            ("mid_2", 0.0, 16.0, 16.0, 16.0),
            ("mid_3", 0.0, 32.0, 16.0, 16.0),
            ("mid_4", 0.0, 48.0, 16.0, 16.0),
            ("right", 16.0, 0.0, 16.0, 16.0),
            ("left", 32.0, 0.0, 16.0, 16.0),
        ];
        for (name, x_pos, y_pos, w, h) in directions {
            sprite_rect.insert(
                name.to_string(),
                Rect {
                    x: x_pos,
                    y: y_pos,
                    w: w,
                    h: h,
                },
            );
        }
        sprite_rect
    }
}
