use std::fmt::{Display, format, Formatter};
use macroquad::color::{BLACK, GRAY, WHITE};
use macroquad::input::{is_key_down};
use macroquad::input::KeyCode::{A, C, D, Down, Enter, Left, Right, S, Space, Up, W};
use macroquad::prelude::{clear_background, draw_text_ex, TextParams};
use macroquad::text::{Font, measure_text};
use macroquad::window::screen_width;
use crate::comet::{Comet, Size};
use crate::player::Player;
use crate::projectile::Projectile;

const ROTATION_SPEED: f32 = 0.15f32;
const PROJECTILE_SPEED: f32 = 30f32;
const SHOOTING_COOLDOWN: f32 = 0.2f32;
const SPLIT_RATIO: u32 = 2;
const COLLISION_KNOCK_BACK: f32 = 10f32;

const BASE_COMET_SPAWN_RATE: f32 = 10f32;
const INVINCIBILITY_DURATION: f32 = 1f32;

#[derive(Debug)]
pub enum GamePhase {
    MENU,
    PLAY,
    END,
}


pub struct GameState {
    game_state: GamePhase,
    player: Player,
    comets: Vec<Comet>,
    projectiles: Vec<Projectile>,
    font: Font,
    score: i32,
    weapon_cd: f32,
    player_lives: u32,
    comet_spawn_timer: f32,
    game_duration: f32,
    invincibility_timer: f32,
}

use GamePhase::*;

impl GameState {
    pub fn new(font: Font) -> GameState {
        GameState {
            player: Player::new(),
            comets: Vec::new(),
            projectiles: Vec::new(),
            font,
            score: 0,
            weapon_cd: 0f32,
            player_lives: 3,
            game_state: MENU,
            comet_spawn_timer: 0f32,
            game_duration: 0f32,
            invincibility_timer: 1f32,
        }
    }
    pub fn get_comets_mut(&mut self) -> &mut Vec<Comet> { &mut self.comets }
    pub fn get_comets(&self) -> &Vec<Comet> { &self.comets }
    pub fn get_projectiles_mut(&mut self) -> &mut Vec<Projectile> { &mut self.projectiles }
    pub fn get_projectiles(&self) -> &Vec<Projectile> { &self.projectiles }

    pub fn update(&mut self) {
        self.inputs();
        match self.game_state {
            PLAY => {
                self.spawn_comet_with_spawn_rate();
                self.player.update();
                let mut new_comets: Vec<Comet> = Vec::new();
                for comet in self.comets.iter_mut() {
                    comet.update();
                    if self.player.overlaps_shape(comet.get_shape()) {
                        let direction = self.player.get_pos() - comet.get_pos();
                        self.player.give_impulse(direction, COLLISION_KNOCK_BACK);
                        if self.invincibility_timer == 0f32 {
                            self.player_lives -= 1;
                            self.invincibility_timer = INVINCIBILITY_DURATION;
                            if self.player_lives <= 0 { self.game_state = END }
                        }
                    }
                }
                for comet in self.comets.iter_mut() {
                    for projectile in self.projectiles.iter_mut() {
                        if comet.contains(projectile.get_tip_pos()) {
                            match comet.get_size() {
                                Size::Three => {
                                    for _idx in 0..SPLIT_RATIO {
                                        new_comets.push(Comet::spawn(Size::Two, Some(comet.get_pos())));
                                    }
                                }
                                Size::Two => {
                                    for _idx in 0..SPLIT_RATIO {
                                        new_comets.push(Comet::spawn(Size::One, Some(comet.get_pos())));
                                    }
                                }
                                Size::One => ()
                            }
                            comet.destroy();
                            self.score += 10;
                            projectile.destroy();
                            break;
                        }
                    }
                }
                self.comets.append(&mut new_comets);
                for projectile in self.projectiles.iter_mut() {
                    projectile.update();
                    if projectile.is_off_screen() {
                        projectile.destroy();
                        self.score -= 1;
                    }
                }
                self.projectiles.retain(|projectile| projectile.is_alive());
                self.comets.retain(|comet| comet.is_alive());
            }
            _ => ()
        }

    }
    pub fn draw(&self) {
        clear_background(BLACK);
        match self.game_state {
            PLAY => {
                match self.invincibility_timer {
                    0f32 => self.player.draw(WHITE),
                    _ => self.player.draw(GRAY),
                }

                for comet in self.comets.iter() {
                    comet.draw();
                }
                for projectile in self.projectiles.iter() {
                    projectile.draw();
                }
                // draw score
                draw_text_ex(&format!("Score: {}", self.score), 200f32, 40f32, TextParams {
                    font: Option::from(&self.font),
                    font_size: 30,
                    font_scale: 1f32,
                    font_scale_aspect: 1f32,
                    rotation: 0.0,
                    color: WHITE,
                });
                draw_text_ex(&format!("Lives: {}", self.player_lives), 20f32, 40f32, TextParams {
                    font: Option::from(&self.font),
                    font_size: 30,
                    font_scale: 1f32,
                    font_scale_aspect: 1f32,
                    rotation: 0.0,
                    color: WHITE,
                });
            }
            END => {
                let text = "GAME OVER!";
                let (font_size) = (50);
                let text_dim = measure_text(text, Option::from(&self.font), font_size, 1f32);
                draw_text_ex(text, screen_width()/2f32 - text_dim.width/2f32, 200f32, TextParams {
                    font: Option::from(&self.font),
                    font_size,
                    font_scale: 1f32,
                    font_scale_aspect: 1f32,
                    rotation: 0.0,
                    color: WHITE,
                });
                let text = &format!("YOUR SCORE WAS: {}", self.score);
                let text_dim = measure_text(text, Option::from(&self.font), font_size, 1f32);
                draw_text_ex(text, screen_width()/2f32 - text_dim.width/2f32, 280f32, TextParams {
                    font: Option::from(&self.font),
                    font_size,
                    font_scale: 1f32,
                    font_scale_aspect: 1f32,
                    rotation: 0.0,
                    color: WHITE,
                });
                let text = "PRESS ENTER TO RESTART";
                let text_dim = measure_text(text, Option::from(&self.font), font_size, 1f32);
                draw_text_ex(text, screen_width()/2f32 - text_dim.width/2f32, 360f32, TextParams {
                    font: Option::from(&self.font),
                    font_size,
                    font_scale: 1f32,
                    font_scale_aspect: 1f32,
                    rotation: 0.0,
                    color: WHITE,
                });
            }
            MENU => {
                let text = "ASTEROIDS";
                let font_size = 50;
                let text_dim = measure_text(text, Option::from(&self.font), font_size+40, 1f32);
                draw_text_ex(text, screen_width()/2f32 - text_dim.width/2f32, 200f32, TextParams {
                    font: Option::from(&self.font),
                    font_size: font_size+40,
                    font_scale: 1f32,
                    font_scale_aspect: 1f32,
                    rotation: 0.0,
                    color: WHITE,
                });
                let text = "PRESS ENTER TO START";
                let text_dim = measure_text(text, Option::from(&self.font), font_size, 1f32);
                draw_text_ex(text, screen_width()/2f32 - text_dim.width/2f32, 280f32, TextParams {
                    font: Option::from(&self.font),
                    font_size,
                    font_scale: 1f32,
                    font_scale_aspect: 1f32,
                    rotation: 0.0,
                    color: WHITE,
                });

            }
            _ => ()
        }

    }
    pub fn shoot(&mut self) {
        if self.weapon_cd == 0f32 {
            self.projectiles.push(Projectile::new(PROJECTILE_SPEED, self.player.get_dir(), self.player.get_pos()));
            self.weapon_cd = SHOOTING_COOLDOWN;
        }
    }
    pub fn refresh_all_cool_downs(&mut self, delta_time: f32) {
        match self.weapon_cd {
            _ if self.weapon_cd <= delta_time => self.weapon_cd = 0f32,
            _ => self.weapon_cd -= delta_time,
        }
        match self.game_state {
            PLAY => {
                match self.invincibility_timer {
                    _ if self.invincibility_timer <= delta_time => self.invincibility_timer = 0f32,
                    _ => self.invincibility_timer -= delta_time,
                }
                self.game_duration += delta_time;
                self.comet_spawn_timer += delta_time;
            }
            _ => ()
        }
    }
    pub fn spawn_comet(&mut self) {
        self.comets.push(Comet::spawn(Size::Three, None));
    }
    pub fn spawn_comet_with_spawn_rate(&mut self) {
        if self.comet_spawn_timer >= BASE_COMET_SPAWN_RATE / (0.5f32 * self.game_duration.sqrt()) {
            self.comet_spawn_timer = 0f32;
            self.spawn_comet();
        }
    }
    pub fn accelerate(&mut self, factor: f32) {
        self.player.accelerate(factor);
    }
    pub fn rotate(&mut self, right: bool) {
        if right { self.player.rotate(ROTATION_SPEED) } else { self.player.rotate(-ROTATION_SPEED) }
    }
    pub fn inputs(&mut self) {
        match self.game_state {
            PLAY => {
                if is_key_down(Left) || is_key_down(A) {
                    self.rotate(false);
                }
                if is_key_down(Right) || is_key_down(D) {
                    self.rotate(true);
                }
                if is_key_down(Up) || is_key_down(W) {
                    self.accelerate(1f32);
                }
                if is_key_down(Down) || is_key_down(S) {
                    self.accelerate(-0.5f32);
                }
                if is_key_down(Space) {
                    self.shoot();
                }
                if is_key_down(C) {
                    self.spawn_comet()
                }
            }
            END => {
                if is_key_down(Enter) {
                    self.game_state = PLAY;
                    self.reset();
                }
            }
            MENU => {
                if is_key_down(Enter) {
                    self.game_state = PLAY;
                    self.reset();
                }
            }
            _ => ()
        }
    }

    pub fn reset(&mut self) {
        self.player = Player::new();
        self.comets.clear();
        self.projectiles.clear();
        self.score = 0;
        self.weapon_cd = 0f32;
        self.player_lives = 3;
        self.game_duration = 0f32;
        self.spawn_comet();
    }
}