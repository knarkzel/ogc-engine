extern crate alloc;

use crate::display::Display;
use alloc::boxed::Box;
use ogc::{gx::Gx, video::Video};

pub trait State {
    fn setup(&mut self, _video: &mut Video) {}
    fn update(&mut self, _video: &mut Video, _display: &mut Display) {}
}

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
        Video::set_black(true);
        Video::flush();
        Video::wait_vsync();
        Video::set_black(false);

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
