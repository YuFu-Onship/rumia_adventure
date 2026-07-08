use core::{f32, panic};
use macroquad::{prelude::*, telemetry::drawcalls};
use std::{
    collections::{HashMap, VecDeque},
    default,
    env::{self},
    path::PathBuf,
    sync::OnceLock,
    vec,
};

mod enemy;
mod particle;
mod rumia;
mod weapon;

use enemy::EnemyTrait;
use particle::{CircleParticle, ParticleParams, ParticleSystem};
use rumia::Rumia;
use weapon::{Weapon, WeaponType};

use crate::enemy::{FairyAction, OrdinaryFairy, Punching};

// pico8颜色 ---------------------------------
static PICO_COLOR: OnceLock<HashMap<&'static str, Color>> = OnceLock::new();
fn get_color(name: &str) -> Color {
    PICO_COLOR
        .get()
        .and_then(|map| map.get(name))
        .copied()
        .unwrap_or(WHITE)
}
fn init_pico_palette() -> HashMap<&'static str, Color> {
    let mut m = HashMap::new();
    m.insert("black", Color::from_rgba(0, 0, 0, 255));
    m.insert("dark_blue", Color::from_rgba(29, 43, 83, 255));
    m.insert("dark_purple", Color::from_rgba(126, 37, 83, 255));
    m.insert("dark_green", Color::from_rgba(0, 135, 81, 255));
    m.insert("brown", Color::from_rgba(171, 82, 54, 255));
    m.insert("dark_gray", Color::from_rgba(95, 87, 79, 255));
    m.insert("light_gray", Color::from_rgba(194, 195, 199, 255));
    m.insert("white", Color::from_rgba(255, 241, 232, 255));
    m.insert("red", Color::from_rgba(255, 0, 77, 255));
    m.insert("orange", Color::from_rgba(255, 163, 0, 255));
    m.insert("yellow", Color::from_rgba(255, 236, 39, 255));
    m.insert("green", Color::from_rgba(0, 228, 54, 255));
    m.insert("blue", Color::from_rgba(41, 173, 255, 255));
    m.insert("indigo", Color::from_rgba(131, 118, 156, 255));
    m.insert("pink", Color::from_rgba(255, 119, 168, 255));
    m.insert("peach", Color::from_rgba(255, 204, 170, 255));
    m
}

// use crate::BulletType::RumiaStraight;
// bullet --------------------------------
#[derive(Debug, Clone, Copy)]
pub enum BulletType {
    RumiaStraight { angle: f32 },
    RumiaTracking { angle: f32 },
    EnemyStraight { speed: f32, angle: f32 },
}
pub struct Bullet {
    pub pos: Vec2,
    pub v: Vec2,
    pub a: Vec2,

    pub radius: f32,
    pub damage: f32,
    pub is_me: bool,
    pub bullet_type: BulletType,
    pub is_actve: bool,
    timer: f32,

    pub history: VecDeque<Vec2>,
    pub max_trail_length: usize,
}

impl Bullet {
    fn update(&mut self, dt: f32, enemy_pool: &EnemyPool) {
        match &mut self.bullet_type {
            BulletType::RumiaStraight { angle } => {
                self.history.push_front(self.pos);
                if self.history.len() >= self.max_trail_length {
                    self.history.pop_back();
                }
                self.pos += self.v * dt;
            }
            BulletType::RumiaTracking { angle } => {
                self.history.push_front(self.pos);
                if self.history.len() >= self.max_trail_length {
                    self.history.pop_back();
                }

                let mut shortest_distance = 0.0;
                let mut target_v = vec2(0.0, 0.0);
                for enemy in enemy_pool.enemies.iter() {
                    let (ex, ey) = enemy.position();
                    let dx = ex - self.pos.x;
                    let dy = ey - self.pos.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if shortest_distance == 0.0 || distance < shortest_distance {
                        shortest_distance = distance;
                        let nx = dx / distance;
                        let ny = dy / distance;
                        target_v = vec2(nx, ny) * 600.0;
                    }
                }
                self.v = self.v + (target_v - self.v) * 5.0 * dt;
                self.pos += self.v * dt;
            }
            BulletType::EnemyStraight { speed, angle } => {
                self.pos += self.v * dt;
            }
        }
    }
    fn draw(&self) {
        match &self.bullet_type {
            BulletType::RumiaStraight { .. } => {
                for (i, point) in self.history.iter().enumerate() {
                    draw_circle(
                        point.x,
                        point.y,
                        self.radius * (1.0 - (i as f32 + 1.0) / self.max_trail_length as f32),
                        get_color("red"),
                    );
                }
            }
            BulletType::RumiaTracking { .. } => {
                for (i, point) in self.history.iter().enumerate() {
                    draw_circle(
                        point.x,
                        point.y,
                        self.radius * (1.0 - (i as f32 + 1.0) / self.max_trail_length as f32),
                        get_color("red"),
                    );
                }
            }
            BulletType::EnemyStraight { .. } => {}
        }
    }
}

// 敌人池 -----------------------------------
struct EnemyPool {
    // 使用box智能指针, 自动分配内存
    enemies: Vec<Box<dyn EnemyTrait>>,
}
impl EnemyPool {
    pub fn new() -> Self {
        Self {
            enemies: Vec::new(),
        }
    }
    pub fn spawn<T: EnemyTrait + 'static>(&mut self, enemy: T) {
        // 'static生命周期, 表示不包含对于其他临时局部变量的引用
        self.enemies.push(Box::new(enemy));
    }
    pub fn update(&mut self, dt: f32) {
        for enemy in self.enemies.iter_mut() {
            enemy.update(dt);
        }
    }
    pub fn draw(&self) {
        for enemy in self.enemies.iter() {
            enemy.draw();
        }
    }
}

// 主函数 -----------------------------------
#[macroquad::main("我的第一个游戏")] // 属性宏,编译时宏在下面注入代码
async fn main() {
    PICO_COLOR.set(init_pico_palette()).unwrap_or_else(|_| {
        println!("color init failed");
    });

    // 画布
    const GAME_WIDTH: f32 = 320.0;
    const GAME_HEIGHT: f32 = 180.0;
    let render_target = render_target(GAME_WIDTH as u32, GAME_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let mut game_camera = Camera2D::from_display_rect(Rect {
        x: 0.0,
        y: 0.0,
        w: GAME_WIDTH,
        h: GAME_HEIGHT,
    });
    game_camera.render_target = Some(render_target.clone());

    // 创建根路径
    let current_dir = env::current_dir().expect("failed to get cur dir");
    let mut project_root = current_dir.clone();
    project_root.pop();

    // 导入字体
    let font_path: PathBuf = project_root.join("assets").join("04B_03__.ttf");
    let font_path_str: &str = font_path.to_str().expect("load font error");
    let mut font: Font = load_ttf_font(font_path_str).await.unwrap_or_else(|e| {
        panic!("load font error:{}", e);
    });
    font.set_filter(FilterMode::Nearest);

    // 导入贴图
    let image_path = project_root.join("assets").join("rumia_1.png");
    let file_byte =
        std::fs::read(&image_path).unwrap_or_else(|e| panic!("read image failed: {}", e));

    let texture_rumia: Texture2D = Texture2D::from_file_with_format(&file_byte, None);
    texture_rumia.set_filter(FilterMode::Nearest);

    // 初始化露米娅
    let mut rumia = Rumia::new(80.0, 90.0, texture_rumia);

    // 创建弹幕池
    let mut bullet_pool: Vec<Bullet> = Vec::with_capacity(500);

    // 创建敌人池
    let mut enemy_pool: EnemyPool = EnemyPool::new();
    enemy_pool.spawn(Punching::new(200.0, 90.0));
    let mut fairy = OrdinaryFairy::new(260.0, 90.0);
    fairy.actions = vec![FairyAction::MoveCircle {
        center: Vec2 { x: 250.0, y: 90.0 },
        radius: 30.0,
        speed: 10.0,
    }];
    enemy_pool.spawn(fairy);

    // 创建粒子池
    let mut particle_pool: ParticleSystem = ParticleSystem::new(500);

    // 游戏主循环
    loop {
        let dt: f32 = get_frame_time();

        rumia.update(dt);
        rumia.update_weapons(dt);
        if let Some(bullets) = rumia.fire() {
            for bullet in bullets {
                bullet_pool.push(bullet);
            }
        }

        enemy_pool.update(dt);

        for bullet in bullet_pool.iter_mut() {
            bullet.update(dt, &enemy_pool);
        }

        for enemy in enemy_pool.enemies.iter_mut() {
            for bullet in bullet_pool.iter_mut() {
                if enemy.check_collision(bullet.pos.x, bullet.pos.y, bullet.radius, bullet.damage) {
                    bullet.is_actve = false;

                    for _ in 0..5 {
                        let arc = rand::gen_range(-f32::consts::PI, f32::consts::PI);
                        let offset = rand::gen_range(0.8, 1.2);
                        particle_pool.emit(ParticleParams {
                            pos: vec2(bullet.pos.x, bullet.pos.y),
                            v: vec2(arc.cos() * 40.0 * offset, arc.sin() * 40.0 * offset),
                            a: vec2(arc.cos() * 4.0 * offset, arc.sin() * 4.0 * offset),
                            radius: 10.0 * offset,
                            scale_coe: 0.04,
                            is_gravity: false,
                            color: get_color("red"),
                        });
                    }
                }
            }
        }
        particle_pool.update(dt);

        // 设置摄像机
        set_camera(&game_camera);
        clear_background(get_color("dark_blue"));

        rumia.draw();
        enemy_pool.draw();
        particle_pool.draw();

        for bullet in bullet_pool.iter() {
            bullet.draw();
        }
        bullet_pool.retain(|b| {
            b.pos.x >= -30.0
                && b.pos.x <= 400.0
                && b.pos.y >= -30.0
                && b.pos.y <= 210.0
                && b.is_actve
        });
        draw_text_ex(
            format!("HP:{}", rumia.hp),
            0.0,
            8.0,
            TextParams {
                font_size: 8,
                color: WHITE,
                font: Some(&font),
                ..Default::default()
            },
        );

        // 移动与缩放
        set_default_camera();
        let scale = (screen_width() / GAME_WIDTH).min(screen_height() / GAME_HEIGHT);
        let offset_x = (screen_width() - GAME_WIDTH * scale) * 0.5;
        let offset_y = (screen_height() + GAME_HEIGHT * scale) * 0.5;
        draw_texture_ex(
            &render_target.texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(GAME_WIDTH * scale, -GAME_HEIGHT * scale)),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
