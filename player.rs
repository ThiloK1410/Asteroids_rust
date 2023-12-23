use macroquad::color::{Color, WHITE};
use macroquad::math::{Vec2};
use macroquad::shapes::{draw_triangle_lines};
use macroquad::window::{screen_height, screen_width};
use crate::shape::Shape;

pub struct Player {
    pos: Vec2,
    vel: Vec2,
    dir: Vec2,
    hit_box_radius: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vec2::new(screen_width() / 2f32, screen_height() / 2f32),
            vel: Vec2::splat(0f32),
            dir: Vec2::new(0f32, -1f32),
            hit_box_radius: 10f32,
        }
    }
    pub fn get_dir(&self) -> Vec2 {
        self.dir
    }
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }
    pub fn update(&mut self) {
        self.pos += self.vel;
        if self.vel.length() > 0.2f32 {
            self.vel -= 0.2f32*self.vel.normalize();
        } else {
            self.vel = Vec2::ZERO;
        }
        match (self.pos.x, self.pos.y) {
            (w, _) if w <= 0f32 => self.pos.x = screen_width(),
            (w, _) if w >= screen_width() => self.pos.x = 0f32,
            (_, h) if h <= 0f32 => self.pos.y = screen_height(),
            (_, h) if h >= screen_height() => self.pos.y = 0f32,
            _ => ()
        }
    }
    pub fn draw(&self, color: Color) {
        // hardcoded with collision !!!!!
        let perp = self.dir.perp().normalize();
        let (a, b, c) = (self.pos-self.dir*10f32-perp*10f32,
                         self.pos-self.dir*10f32+perp*10f32,
                         self.pos+self.dir*20f32);
        draw_triangle_lines(a, b, c, 2f32, color);
    }
    pub fn rotate(&mut self, angle: f32) {
        self.dir = Vec2::new(angle.cos()*self.dir.x - angle.sin()*self.dir.y,
        angle.sin()*self.dir.x + angle.cos()*self.dir.y).normalize_or_zero()
    }
    pub fn accelerate(&mut self, factor: f32) {
        self.vel += self.dir * factor;
        if self.vel.length() > 10f32 {
            self.vel = self.vel.normalize()*10f32;
        }
    }
    pub fn contains(&self, point: Vec2) -> bool {
        let hit_box_radius = 10f32;
        let abs_dist = (point - self.pos).length();
        if abs_dist > hit_box_radius { return false }
        return true
    }
    pub fn overlaps_shape(&self, shape: &Shape) -> bool {
        let collision_distance = shape.get_rad() + self.hit_box_radius;
        if (shape.get_pos() - self.pos).length() > collision_distance { return false }
        true
    }
    pub fn give_impulse(&mut self, direction: Vec2, power: f32) {
        let dir = direction.normalize();
        self.vel += dir * power;
    }
}