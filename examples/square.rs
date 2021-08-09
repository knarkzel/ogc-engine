#![no_std]
#![feature(start)]

use embedded_graphics::primitives::{Rectangle, Triangle};
use ogc_engine::prelude::*;

struct Game {
    x: i32,
    y: i32,
}

impl State for Game {
    fn update(&mut self) {
        self.x = (self.x + 1) % 640;
        self.y = (self.y + 2) % 528;
    }

    fn draw(&self, display: &mut Display) -> Result<(), DrawError> {
        display.clear(Rgb::CYAN)?;

        let rectangle = Rectangle::new(Point::new(self.x, self.y), Size::new(50, 50));
        display.fill_solid(&rectangle, Rgb::WHITE)?;

        let triangle = Triangle::new(Point::new(100, 10), Point::new(50, 10), Point::new(100, 50));
        display.fill_triangle(&triangle, Rgb::YELLOW)?;

        Ok(())
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let state = Game { x: 0, y: 0 };
    Engine::run(state)
}
