#![no_std]
#![feature(start)]

use embedded_graphics::{mono_font::MonoTextStyle, text::Text};
use ogc_engine::prelude::*;
use profont::PROFONT_18_POINT;

struct Game {
    y: i32,
}

impl State for Game {
    fn init() {}

    fn draw(&self, display: &mut Display) -> Result<(), ogc_engine::DrawError> {
        let text_style = MonoTextStyle::new(&PROFONT_18_POINT, Rgb::WHITE);
        Text::new(
            "Hello world, this is me drawing some text!",
            Point::new(0, self.y),
            text_style,
        )
        .draw(display)?;

        Ok(())
    }

    fn update(&mut self) {
        self.y = (self.y + 4) % 528;
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let state = Game { y: 0 };
    Engine::run(state)
}
