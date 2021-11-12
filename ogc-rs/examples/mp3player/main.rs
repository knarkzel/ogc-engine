#![no_std]
#![feature(start)]

extern crate alloc;
use ogc::prelude::*;

const MUSIC: &[u8] = include_bytes!("sample.mp3");

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Initialise the video system
    let video = Video::init();

    // Initialise the console, required for print.
    Console::init(&video);

    // Initialize the audio
    Asnd::init();
    Mp3Player::init();

    // Set up the video registers with the chosen mode.
    Video::configure(video.render_config.into());

    // Tell the video hardware where our display memory is.
    Video::set_next_framebuffer(video.framebuffer);

    // Make the display visible.
    Video::set_black(false);

    // Flush the video register changes to the hardware.
    Video::flush();

    // Wait for Video setup to complete.
    Video::wait_vsync();

    // Play music
    Mp3Player::play_buffer(MUSIC);
    println!("Playing music with buffer size {}", MUSIC.len());

    loop {
        // Wait for the next frame.
        Video::wait_vsync();
    }
}
