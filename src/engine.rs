extern crate alloc;

use crate::display::Display;
use alloc::boxed::Box;
use ogc::gx::Gx;
use ogc::video::Video;

type Setup = Box<dyn FnOnce(&mut Video)>;
type Update = Box<dyn Fn(&mut Video, &mut Display)>;

#[derive(Default)]
pub struct Engine {
    display: Option<Display>,
    setup: Option<Setup>,
    update: Option<Update>,
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(mut self, function: Setup) -> Self {
        self.setup = Some(function);
        self
    }

    pub fn update(mut self, function: Update) -> Self {
        self.update = Some(function);
        self
    }

    pub fn display(mut self, fifo_size: usize) -> Self {
        self.display = Some(Display::new(fifo_size));
        self
    }

    pub fn run(self) -> ! {
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

        if let Some(setup) = self.setup {
            setup(&mut video);
        }

        loop {
            Gx::set_viewport(0.0, 0.0, fb_width, emb_height, 0.0, 0.0);

            if let Some(ref update) = self.update {
                update(&mut video, &mut display);
            }

            Video::set_next_framebuffer(video.framebuffer);
            Video::flush();
            Video::wait_vsync();
        }
    }
}
