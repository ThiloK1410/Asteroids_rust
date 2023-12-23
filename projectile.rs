use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::shapes::draw_line;
use macroquad::window::{screen_height, screen_width};

pub struct Projectile {
    pos: Vec2,
    dir: Vec2,
    speed: f32,
    length: f32,
    alive: bool,
}
impl Projectile {
    pub fn new(speed: f32, direction: Vec2, position: Vec2) -> Projectile {
        Projectile {
            speed,
            dir: direction.normalize(),
            pos: position,
            length: 10f32,
            alive: true,
        }
    }
    pub fn update(&mut self) {
        self.pos += self.dir * self.speed;
    }
    pub fn destroy(&mut self) {self.alive = false}

    pub fn is_alive(&self) -> bool {self.alive}
    pub fn draw(&self) {
        let sec_point = self.pos + self.dir * self.length;
        draw_line(self.pos.x, self.pos.y, sec_point.x, sec_point.y, 2f32, WHITE);
    }
    pub fn get_tip_pos(&self) -> Vec2 {
        self.pos + self.dir * self.length
    }

    pub fn is_off_screen(&self) -> bool{
        let puffer = 100f32;
        if self.pos.x < 0f32-puffer || self.pos.x > screen_width()+puffer {return true}
        if self.pos.y < 0f32-puffer || self.pos.y > screen_height()+puffer {return true}
        false
    }
}