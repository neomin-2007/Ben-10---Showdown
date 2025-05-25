use macroquad::{miniquad::window::set_window_position, prelude::*};

mod data;
use data::*;

#[macroquad::main("Ben 10 - Showdown")]
async fn main() {

    set_window_position(800, 600);

    let mut player = Player::new().await;

    loop {

        Player::render(&mut player);
        Player::movement(&mut player);

        next_frame().await;
    }
}