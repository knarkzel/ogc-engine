#![no_std]
#![feature(start)]

use engine::prelude::*;

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, Primitive, RgbColor, Size},
    primitives::{PrimitiveStyleBuilder, Rectangle},
    Drawable,
};

fn update(video: &mut Video, display: &mut Display) {
    Rectangle::new(Point::new(50, 50), Size::new(50, 50))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::WHITE)
                .build(),
        )
        .draw(display)
        .unwrap();
    display.flush(video.framebuffer);
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    Engine::new().update(Box::new(update)).run()
}
