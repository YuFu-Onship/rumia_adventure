use macroquad::prelude::*;

use crate::get_color;

pub trait EnemyTrait {
    fn update(&mut self, dt: f32);
    fn draw(&self);
    fn is_dead(&self) -> bool;
    fn is_leave(&self) -> bool;
    fn position(&self) -> (f32, f32);
    fn check_collision(&mut self, x: f32, y: f32, radius: f32, demage: f32) -> bool;

    fn api_set_pos(&mut self, x: f32, y: f32);
    fn api_get_action(&self) -> FairyAction;
    fn api_get_base_attribute(&mut self) -> &mut BaseAtttribute;
}

#[derive(Clone)]
pub enum FairyAction {
    MoveTarget {
        pos: Vec2,
        coe: f32,
    },
    MoveCircle {
        center: Vec2,
        radius: f32,
        speed: f32,
    },
    IdleOrShoot {
        duration: f32,
    },
    Idle {},
}

pub struct BaseAtttribute {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub radius: f32,
    pub timer: f32,
}

// 小妖精 ------------------------------------------
pub struct OrdinaryFairy {
    pub base_attribute: BaseAtttribute,
    pub speed: f32,
    pub leave: bool,
    pub actions: Vec<FairyAction>,
    cur_action_idx: usize,
    timer: f32,
}

impl OrdinaryFairy {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            base_attribute: BaseAtttribute {
                x,
                y,
                hp: 10.0,
                timer: 0.0,
                radius: 8.0,
            },
            speed: 50.0,
            actions: vec![FairyAction::Idle {}],
            cur_action_idx: 0,
            timer: 0.0,
            leave: false,
        }
    }

    pub fn move_to(&mut self, dt: f32, x: f32, y: f32) {
        let dx = x - self.base_attribute.x;
        let dy = y - self.base_attribute.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let speed = self.speed * dt;
        if distance <= 0.1 {
            self.base_attribute.x = x;
            self.base_attribute.y = y;
        } else {
            self.base_attribute.x += dx / distance * speed;
            self.base_attribute.y += dy / distance * speed;
        }
    }
    pub fn move_circle(&mut self, dt: f32, center: Vec2, radius: f32, speed: f32) {
        let angle = self.timer * speed;
        let ndx = angle.cos() - angle.sin();
        let ndy = angle.sin() + angle.cos();
        self.base_attribute.x = center.x + ndx * radius;
        self.base_attribute.y = center.y + ndy * radius;
    }
}

impl EnemyTrait for OrdinaryFairy {
    fn update(&mut self, dt: f32) {
        self.timer += dt;
        self.base_attribute.timer += dt;
        match self.actions[self.cur_action_idx] {
            FairyAction::MoveTarget { pos, coe: _ } => {
                println!("{}", pos.x);
                self.move_to(dt, pos.x, pos.y);
            }
            FairyAction::MoveCircle {
                center,
                radius,
                speed,
            } => {
                self.move_circle(dt, center, radius, speed);
            }
            FairyAction::IdleOrShoot { .. } => {}
            FairyAction::Idle {} => {}
        }
    }

    fn draw(&self) {
        draw_circle(
            self.base_attribute.x,
            self.base_attribute.y,
            self.base_attribute.radius,
            GREEN,
        );
    }

    fn is_dead(&self) -> bool {
        self.base_attribute.hp <= 0.0
    }

    fn position(&self) -> (f32, f32) {
        (self.base_attribute.x, self.base_attribute.y)
    }
    fn check_collision(&mut self, x: f32, y: f32, radius: f32, demage: f32) -> bool {
        let dx = self.base_attribute.x - x;
        let dy = self.base_attribute.y - y;
        let dis = dx * dx + dy * dy;
        if dis < (self.base_attribute.radius + radius) * (self.base_attribute.radius + radius) {
            self.base_attribute.hp -= demage;
            return true;
        }
        false
    }
    fn is_leave(&self) -> bool {
        self.leave
    }
    fn api_set_pos(&mut self, x: f32, y: f32) {
        self.base_attribute.x = x;
        self.base_attribute.y = y;
    }
    fn api_get_action(&self) -> FairyAction {
        self.actions
            .get(self.cur_action_idx)
            .cloned()
            .unwrap_or(FairyAction::IdleOrShoot { duration: 0.0 })
    }
    fn api_get_base_attribute(&mut self) -> &mut BaseAtttribute {
        &mut self.base_attribute
    }
    // fn api_get_action(&self) -> Option<&FairyAction> {
    //         self.actions.get(self.cur_action_idx)
    // }
}

// 沙包 -----------------------------------------------------
pub struct Punching {
    pub base_attribute: BaseAtttribute,
    // pub speed: f32,
}
impl Punching {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            base_attribute: BaseAtttribute {
                x,
                y,
                hp: 10000.0,
                radius: 10.0,
                timer: 0.0,
            }, // speed: 50.0,
        }
    }
}
impl EnemyTrait for Punching {
    fn update(&mut self, _dt: f32) {
        self.base_attribute.hp = (self.base_attribute.hp + 100.0).min(10000.0);
    }

    fn draw(&self) {
        draw_circle(
            self.base_attribute.x,
            self.base_attribute.y,
            self.base_attribute.radius,
            get_color("white"),
        );
    }

    fn is_dead(&self) -> bool {
        self.base_attribute.hp <= 0.0
    }

    fn position(&self) -> (f32, f32) {
        (self.base_attribute.x, self.base_attribute.y)
    }
    fn check_collision(&mut self, x: f32, y: f32, radius: f32, demage: f32) -> bool {
        let dx = self.base_attribute.x - x;
        let dy = self.base_attribute.y - y;
        let dis = dx * dx + dy * dy;
        if dis < (self.base_attribute.radius + radius) * (self.base_attribute.radius + radius) {
            self.base_attribute.hp -= demage;
            return true;
        }
        false
    }
    fn is_leave(&self) -> bool {
        false
    }
    fn api_set_pos(&mut self, x: f32, y: f32) {
        self.base_attribute.x = x;
        self.base_attribute.y = y;
    }
    fn api_get_action(&self) -> FairyAction {
        FairyAction::IdleOrShoot { duration: 0.0 }
    }
    fn api_get_base_attribute(&mut self) -> &mut BaseAtttribute {
        &mut self.base_attribute
    }
}
