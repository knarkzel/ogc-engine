extern crate alloc;

use crate::display::Display;
use alloc::boxed::Box;
use ogc::{gx::Gx, video::Video};

/// Trait for enabling state.
///
/// # Example
///
/// ```rust
/// use ogc_engine::prelude::*;
///
/// struct Game {
///     x: i32,
///     y: i32,
/// }
///
/// impl State for Game {
///     fn update(&mut self, _video: &mut Video, display: &mut Display) {
///         self.x += 1;
///         self.y += 2;
///     }
/// }
///
/// #[start]
/// fn main(_argc: isize, _argv: *const *const u8) -> isize {
///     let state = Game { x: 50, y: 50 };
///     Engine::new().state(Box::new(state)).run()
/// }
/// ```
pub trait State {
    fn setup(&mut self, _video: &mut Video) {}
    fn update(&mut self, _video: &mut Video, _display: &mut Display) {}
}

/// Game engine abstraction.
#[derive(Default)]
pub struct Engine {
    display: Option<Display>,
    state: Option<Box<dyn State>>,
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(mut self, state: Box<dyn State>) -> Self {
        self.state = Some(state);
        self
    }

    pub fn display(mut self, fifo_size: usize) -> Self {
        self.display = Some(Display::new(fifo_size));
        self
    }

    pub fn run(mut self) -> ! {
        let mut video = Video::init();
        Video::configure(Video::get_preferred_mode().into());
        Video::set_next_framebuffer(video.framebuffer);
        Video::set_black(false);
        Video::flush();
        Video::wait_vsync();

        let mut display = self.display.unwrap_or(Display::new(256 * 1024));
        display.setup(&mut video.render_config);

        let fb_width = video.render_config.framebuffer_width as _;
        let emb_height = video.render_config.embed_framebuffer_height as _;

        if let Some(ref mut state) = self.state {
            state.setup(&mut video);
        }

        loop {
            Gx::set_viewport(0.0, 0.0, fb_width, emb_height, 0.0, 0.0);

            if let Some(ref mut state) = self.state {
                state.update(&mut video, &mut display);
                display.flush(video.framebuffer);
            }

            Video::set_next_framebuffer(video.framebuffer);
            Video::flush();
            Video::wait_vsync();
        }
    }
}
