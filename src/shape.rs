use std::f32::consts::PI;
use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::shapes::draw_line;

pub struct Shape {
    pos: Vec2,
    rotation: f32,
    sides: u32,
    radius: f32,
    line_thickness: f32,
}

impl Shape {
    pub fn new(pos: Vec2, sides: u32, radius: f32, line_thickness: f32) -> Shape {
        Shape {
            pos,
            rotation: 0f32,
            sides,
            radius,
            line_thickness,
        }
    }
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }
    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
    pub fn get_rad(&self) -> f32 {
        self.radius
    }
    pub fn add_pos(&mut self, vec: Vec2) {
        self.pos += vec;
    }
    pub fn add_rot(&mut self, angle: f32) {
        self.rotation += angle;
    }
    pub fn draw(&self) {
        let step_size = 2f32 * PI / self.sides as f32;
        let mut points: Vec<Vec2> = Vec::new();
        for i in 0..self.sides {
            points.push(Vec2::new((self.rotation + i as f32 * step_size).cos(), (self.rotation + i as f32 * step_size).sin()) * self.radius + self.pos)
        }
        for i in 0..self.sides as usize {
            draw_line(points[i].x, points[i].y,
                      points[(i + 1) % self.sides as usize].x, points[(i + 1) % self.sides as usize].y,
                      self.line_thickness,
                      WHITE)
        }
    }
    //checks if point is within radius of shape, not precise increasingly for low sides
    pub fn contains(&self, point: Vec2) -> bool {
        let distance = (self.pos - point).length();
        if distance < self.radius {
            return true
        }
        false
    }
}