#![no_std]
#![feature(start)]

use embedded_graphics::image::Image;
use ogc_engine::prelude::*;
use tinytga::Tga;

const TGA: &[u8] = include_bytes!("football_seal.tga");

struct Game<'a> {
    tga: Tga<'a, Rgb>,
}

impl State for Game<'_> {
    fn draw(&self, display: &mut Display) -> Result<(), ogc_engine::DrawError> {
        let image = Image::new(&self.tga, Point::zero());
        image.draw(display)?;

        Ok(())
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let tga: Tga<Rgb> = Tga::from_slice(TGA).unwrap();
    let state = Game { tga };
    Engine::run(state)
}
