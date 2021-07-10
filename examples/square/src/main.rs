#![no_std]
#![feature(start)]

use ogc_engine::prelude::*;

use embedded_graphics::{pixelcolor::Rgb888, primitives::Rectangle};

struct Game {
    x: i32,
    y: i32,
}

impl State for Game {
    fn update(&mut self, _video: &mut Video, display: &mut Display) {
        let rectangle = Rectangle::new(Point::new(self.x, self.y), Size::new(50, 50));
        display.fill_solid(&rectangle, Rgb888::WHITE).unwrap();
        display.clear(Rgb888::new(150, 150, 150)).unwrap();
        self.x = (self.x + 1) % 640;
        self.y = (self.y + 2) % 528;
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let state = Game { x: 50, y: 50 };
    Engine::new().state(Box::new(state)).run()
}
