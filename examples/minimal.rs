#![no_std]
#![feature(start)]

use ogc_engine::prelude::*;

struct Game;

impl State for Game {}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let state = Game;
    Engine::run(state)
}
