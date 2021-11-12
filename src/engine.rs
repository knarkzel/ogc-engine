extern crate alloc;

use crate::display::Display;
use ogc::{asnd::Asnd, gx::Gx, mp3player::Mp3Player, pad::Pad, video::Video};

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
///     fn update(&mut self) {
///         self.x += 1;
///         self.y += 2;
///     }
///
///     fn draw(&self, display: &mut Display) -> Result<(), DrawError> {
///         let rectangle = Rectangle::new(Point::new(self.x, self.y), Size::new(50, 50));
///         display.fill_solid(&rectangle, Rgb::WHITE)?;
///
///         Ok(())
///     }
/// }
///
/// #[start]
/// fn main(_argc: isize, _argv: *const *const u8) -> isize {
///     let state = Game { x: 50, y: 50 };
///     Engine::run(state)
/// }
/// ```
pub trait State {
    fn init() {}
    fn draw(&self, _display: &mut Display) -> Result<(), crate::DrawError> {
        Ok(())
    }
    fn update(&mut self) {}
}

/// Game engine abstraction.
pub struct Engine;

impl Engine {
    pub fn run<T: State>(mut state: T) -> ! {
        // Init
        let mut video = Video::init();
        Asnd::init();
        Mp3Player::init();
        Pad::init();

        Video::configure(Video::get_preferred_mode().into());
        Video::set_next_framebuffer(video.framebuffer);
        Video::set_black(false);
        Video::flush();
        Video::wait_vsync();

        let mut display = Display::new(256 * 1024);
        display.setup(&mut video.render_config);

        let fb_width = video.render_config.framebuffer_width as _;
        let emb_height = video.render_config.embed_framebuffer_height as _;

        T::init();

        loop {
            Gx::set_viewport(0.0, 0.0, fb_width, emb_height, 0.0, 0.0);
            Pad::scan_pads();

            // Update
            state.update();

            // Draw
            state
                .draw(&mut display)
                .expect("Error occured while drawing");
            display.flush(video.framebuffer);

            Video::set_next_framebuffer(video.framebuffer);
            Video::flush();
            Video::wait_vsync();
            video.flip_framebuffer();
        }
    }
}
