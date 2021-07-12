#![no_std]
#![feature(start)]

use ogc_engine::prelude::*;

use embedded_graphics::primitives::{Rectangle, Triangle};

struct Game {
    x: i32,
    y: i32,
}

impl State for Game {
    fn update(&mut self, _video: &mut Video, display: &mut Display) {
        display.clear(Rgb::RED).unwrap();

        let rectangle = Rectangle::new(Point::new(self.x, self.y), Size::new(50, 50));
        display.fill_solid(&rectangle, Rgb::WHITE).unwrap();

        let triangle = Triangle::new(Point::new(100, 10), Point::new(50, 10), Point::new(100, 50));
        display.fill_triangle(&triangle, Rgb::YELLOW);

        self.x = (self.x + 1) % 640;
        self.y = (self.y + 2) % 528;
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let state = Game { x: 50, y: 50 };
    Engine::new().state(Box::new(state)).run()
}
