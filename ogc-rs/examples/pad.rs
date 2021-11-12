#![no_std]
#![feature(start)]

extern crate alloc;
use ogc::prelude::*;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Initialise the video system
    let video = Video::init();

    // Initialise the console, required for print.
    Console::init(&video);

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

    // Initialise the pad
    Pad::init();

    loop {
        // Scan all pads
        Pad::scan_pads();

        // Check for pressed buttons
        let pressed = Pad::buttons_down(Controller::One);

        if pressed == Button::A {
            println!("A was pressed");
        }

        if pressed == Button::B {
            println!("B was pressed");
        }

        // Check for released buttons
        let released = Pad::buttons_up(Controller::One);

        if released == Button::X {
            println!("X was released");
        }

        if released == Button::Z {
            println!("Z was released");
        }

        // Check for held buttons, also shows how to test multiple buttons
        let held = Pad::buttons_held(Controller::One);

        if held == (Button::Up | Button::A) {
            println!("Up and A is being held");
        }

        if held == (Button::Down | Button::B) {
            println!("Down and B is being held");
        }

        // Check for analog stick
        let (stick_x, stick_y) = (Pad::stick_x(Controller::One), Pad::stick_y(Controller::One));

        if stick_x.abs() > 50 || stick_y.abs() > 50 {
            println!("Analog stick: ({}, {})", stick_x, stick_y);
        }

        // Check for c-stick
        let (sub_stick_x, sub_stick_y) = (
            Pad::sub_stick_x(Controller::One),
            Pad::sub_stick_y(Controller::One),
        );

        if sub_stick_x.abs() > 50 || sub_stick_y.abs() > 50 {
            println!("C-stick: ({}, {})", sub_stick_x, sub_stick_y);
        }

        // Check for triggers
        let (trigger_l, trigger_r) = (
            Pad::trigger_l(Controller::One),
            Pad::trigger_r(Controller::One),
        );

        if trigger_l > 50 || trigger_r > 50 {
            println!("Triggers: ({}, {})", trigger_l, trigger_r);
        }

        // Wait for the next frame.
        Video::wait_vsync();
    }
}
