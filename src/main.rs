#![allow(unused)]

mod create_board;
mod game;
mod greeting;
mod ship;
mod terminal_utils;

fn main() {
    greeting::greet();
    let mut game = game::Game::new();
    game.start_game();
}
