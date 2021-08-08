#![no_std]
#![feature(start)]

use ogc_engine::prelude::*;

const MUSIC: &[u8] = include_bytes!("sample.mp3");

struct Game;

impl State for Game {
    fn init() {
        Mp3Player::play_buffer(MUSIC);
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let state = Game;
    Engine::run(state)
}
