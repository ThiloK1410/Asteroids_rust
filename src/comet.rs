use macroquad::math::Vec2;
use macroquad::window::{screen_height, screen_width};
use rand::{Rng};
use crate::shape::Shape;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Size {
    One,
    Two,
    Three,
}


pub struct Comet {
    size: Size,
    vel: Vec2,
    rot_speed: f32,
    shape: Shape,
    alive: bool,
}
impl Comet {

    pub fn new(size: u32, speed: f32) -> Comet{
        let mut rng = rand::thread_rng();
        let (start_pos, start_vel) = Comet::get_random_start_pos_and_vel();
        Comet {
            size: Size::Three,
            vel: start_vel*speed,
            rot_speed: rng.gen_range(-0.1f32..0.132),
            shape: Shape::new(start_pos, size, size as f32*10f32, 2f32),
            alive: true,
        }
    }

    pub fn spawn(size: Size, pos: Option<Vec2>) -> Comet {
        let mut rng = rand::thread_rng();
        let (default_pos, start_vel) = Comet::get_random_start_pos_and_vel();
        let start_pos = pos.unwrap_or(default_pos);
        let (size, speed, rot_speed, radius, n_sides) = match size {
            Size::Three => (Size::Three, 2f32, rng.gen_range(-0.05f32..0.0532), 50f32, 7u32),
            Size::Two => (Size::Two, 3f32, rng.gen_range(-0.1f32..0.132), 40f32, 5u32),
            Size::One => (Size::One, 5f32, rng.gen_range(-0.2f32..0.232), 30f32, 3u32),
        };
        Comet {
            size,
            vel: start_vel*speed,
            rot_speed,
            shape: Shape::new(start_pos, n_sides, radius, 2f32),
            alive: true,
        }
    }

    pub fn destroy(&mut self) {self.alive = false}
    pub fn is_alive(&self) -> bool {self.alive}
    pub fn get_size(&self) -> Size { self.size.clone() }
    pub fn get_pos(&self) -> Vec2 { self.shape.get_pos() }
    pub fn get_shape(&self) -> &Shape { &self.shape }
    pub fn get_random_vel() -> Vec2 {
        let mut rng = rand::thread_rng();
        Vec2::new(rng.gen_range(-1f32..1f32), rng.gen_range(-1f32..1f32)).normalize()
    }
    pub fn get_random_start_pos_and_vel() -> (Vec2, Vec2) {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => (Vec2::new(0f32, rng.gen_range(0f32..screen_height())),
                  Vec2::new(rng.gen_range(0f32..1f32), rng.gen_range(-1f32..1f32)).normalize()),
            1 => (Vec2::new(rng.gen_range(0f32..screen_width()), 0f32),
                  Vec2::new(rng.gen_range(-1f32..1f32), rng.gen_range(0f32..1f32)).normalize()),
            2 => (Vec2::new(screen_width(), rng.gen_range(0f32..screen_height())),
                  Vec2::new(rng.gen_range(-1f32..0f32), rng.gen_range(-1f32..1f32)).normalize()),
            3 => (Vec2::new(rng.gen_range(0f32..screen_width()), screen_height()),
                  Vec2::new(rng.gen_range(-1f32..1f32), rng.gen_range(-1f32..0f32)).normalize()),
            _ => panic!()
        }
    }

    pub fn draw(&self) {
        self.shape.draw();
    }

    pub fn update(&mut self) {
        self.shape.add_pos(self.vel);
        self.shape.add_rot(self.rot_speed);

        let puffer = self.shape.get_rad()*2f32;
        let mut pos = self.shape.get_pos();
        match (pos.x, pos.y) {
            (w, _) if w <= 0f32-puffer => pos.x = screen_width()+puffer,
            (w, _) if w >= screen_width()+puffer => pos.x = 0f32-puffer,
            (_, h) if h <= 0f32-puffer => pos.y = screen_height()+puffer,
            (_, h) if h >= screen_height()+puffer => pos.y = 0f32-puffer,
            _ => ()
        }
        self.shape.set_pos(pos);
    }
    pub fn contains(&self, point: Vec2) -> bool {
        self.shape.contains(point)
    }

}