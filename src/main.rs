mod player;
mod comet;
mod shape;
mod projectile;
mod game_state;

use macroquad::input::KeyCode::*;
use macroquad::prelude::*;
use crate::game_state::GameState;


const FPS: i32 = 30;    //fps for physics
const TIME_PER_FRAME: f32 = 1f32 / FPS as f32;

#[macroquad::main("Asteroids")]
async fn main() {
    let font = load_ttf_font("res/RubikDoodleShadow-Regular.ttf").await.unwrap();

    let mut game_state = GameState::new(font);

    let mut lag = 0f32;
    loop {
        game_state.refresh_all_cool_downs(get_frame_time());
        lag += get_frame_time();
        while lag >= TIME_PER_FRAME {
            game_state.update();
            lag -= TIME_PER_FRAME;
        }
        game_state.draw();

        next_frame().await;
    }
}
