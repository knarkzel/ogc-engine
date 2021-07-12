#![no_std]
#![feature(start)]

use ogc_engine::prelude::*;

const MUSIC: &[u8] = include_bytes!("sample.mp3");

struct Game;

impl State for Game {
    fn setup(&mut self, _video: &mut Video) {
        Mp3Player::play_buffer(MUSIC);
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let game = Game;
    Engine::new().state(Box::new(game)).run()
}
