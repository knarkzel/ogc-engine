extern crate alloc;
use core::{convert::TryInto, ffi::c_void};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{IntoStorage, OriginDimensions, RgbColor, Size},
    primitives::Rectangle,
    Pixel,
};

use ogc::{
    ffi::{
        Mtx as Mtx34, Mtx44, GX_ALWAYS, GX_AOP_AND, GX_BL_INVSRCALPHA, GX_BL_SRCALPHA, GX_BM_BLEND,
        GX_CLIP_ENABLE, GX_CLR_RGBA, GX_COLOR0A0, GX_CULL_NONE, GX_DIRECT, GX_F32, GX_GM_1_0,
        GX_GREATER, GX_LEQUAL, GX_LO_CLEAR, GX_MAX_Z24, GX_NONE, GX_ORTHOGRAPHIC, GX_PASSCLR,
        GX_PF_RGB8_Z24, GX_PNMTX0, GX_POS_XYZ, GX_QUADS, GX_RGBA8, GX_TEVSTAGE0, GX_TEXCOORD0,
        GX_TEXMAP0, GX_TEX_ST, GX_TRUE, GX_VA_CLR0, GX_VA_POS, GX_VA_TEX0, GX_VTXFMT0,
        GX_ZC_LINEAR,
    },
    prelude::*,
};

pub struct Display;

impl Display {
    pub fn new(fifo_size: usize) -> Self {
        let buffer = gp_fifo(fifo_size);
        Gx::init(buffer, fifo_size as u32);
        Self
    }

    pub fn flush(&self, framebuffer: *mut c_void) {
        Gx::draw_done();
        Gx::set_z_mode(GX_TRUE as _, GX_LEQUAL as _, GX_TRUE as _);
        Gx::copy_disp(framebuffer, GX_TRUE as _);
    }

    pub fn setup(&self, rc: &mut RenderConfig) {
        let mut ident: Mtx34 = [[0.0; 4]; 3];
        let mut perspective: Mtx44 = [[0.0; 4]; 4];

        let color = Color::new(0, 0, 0, 0);
        Gx::set_copy_clear(color, GX_MAX_Z24);
        Gx::set_pixel_fmt(GX_PF_RGB8_Z24 as _, GX_ZC_LINEAR as _);

        let fb_width = rc.framebuffer_width;
        let emb_height = rc.embed_framebuffer_height;
        let ext_height = rc.extern_framebuffer_height;

        Gx::set_viewport(0.0, 0.0, fb_width as _, emb_height as _, 0.0, 0.0);

        let y_scale = Gx::get_y_scale_factor(emb_height, ext_height);
        let ext_fb_height = Gx::set_disp_copy_y_scale(y_scale);

        let half_aspect_ratio = (rc.vi_height == 2 * ext_height) as u32;

        Gx::set_disp_copy_src(0, 0, fb_width, emb_height);
        Gx::set_disp_copy_dst(fb_width, ext_fb_height as _);

        Gx::set_copy_filter(
            rc.anti_aliasing,
            &mut rc.sample_pattern,
            GX_TRUE as _,
            &mut rc.v_filter,
        );

        Gx::set_field_mode(rc.field_rendering, half_aspect_ratio as _);
        Gx::set_disp_copy_gamma(GX_GM_1_0 as _);

        Gx::clear_vtx_desc();
        Gx::inv_vtx_cache();
        Gx::invalidate_tex_all();

        Gx::set_vtx_desc(GX_VA_TEX0 as _, GX_NONE as _);
        Gx::set_vtx_desc(GX_VA_POS as _, GX_DIRECT as _);
        Gx::set_vtx_desc(GX_VA_CLR0 as _, GX_DIRECT as _);

        Gx::set_vtx_attr_fmt(
            GX_VTXFMT0 as _,
            GX_VA_POS as _,
            GX_POS_XYZ as _,
            GX_F32 as _,
            0,
        );
        Gx::set_vtx_attr_fmt(GX_VTXFMT0 as _, GX_VA_TEX0, GX_TEX_ST as _, GX_F32 as _, 0);
        Gx::set_vtx_attr_fmt(
            GX_VTXFMT0 as _,
            GX_VA_CLR0,
            GX_CLR_RGBA as _,
            GX_RGBA8 as _,
            0,
        );
        Gx::set_z_mode(GX_TRUE as _, GX_LEQUAL as _, GX_TRUE as _);

        Gx::set_num_chans(1);
        Gx::set_num_tex_gens(1);
        Gx::set_tev_op(GX_TEVSTAGE0 as _, GX_PASSCLR as _);
        Gx::set_tev_order(
            GX_TEVSTAGE0 as _,
            GX_TEXCOORD0 as _,
            GX_TEXMAP0 as _,
            GX_COLOR0A0 as _,
        );

        Gu::mtx_identity(&mut ident);
        Gu::mtx_trans_apply(&mut ident.clone(), &mut ident, 0.0, 0.0, -100.0);
        Gx::load_pos_mtx_imm(&mut ident, GX_PNMTX0 as _);

        Gu::ortho(
            &mut perspective,
            0.0,
            emb_height as _,
            0.0,
            fb_width as _,
            0.0,
            1000.0,
        );
        Gx::load_projection_mtx(&mut perspective, GX_ORTHOGRAPHIC as _);

        Gx::set_viewport(0.0, 0.0, fb_width as _, emb_height as _, 0.0, 1.0);
        Gx::set_blend_mode(
            GX_BM_BLEND as _,
            GX_BL_SRCALPHA as _,
            GX_BL_INVSRCALPHA as _,
            GX_LO_CLEAR as _,
        );

        Gx::set_alpha_update(GX_TRUE as _);
        Gx::set_alpha_compare(GX_GREATER as _, 0, GX_AOP_AND as _, GX_ALWAYS as _, 0);
        Gx::set_color_update(GX_TRUE as _);
        Gx::set_cull_mode(GX_CULL_NONE as _);
        Gx::set_clip_mode(GX_CLIP_ENABLE as _);

        Gx::set_scissor(0, 0, fb_width as _, emb_height as _);
    }
}

impl DrawTarget for Display {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if let Ok((x @ 0..=639, y @ 0..=527)) = coord.try_into() {
                let color = Color::new(color.r(), color.g(), color.b(), 255);
                Gx::poke_argb(x as u16, y as u16, color);
            }
        }

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let color = color.into_storage();
        let bottom = area.bottom_right().expect("No bottom_right");
        let (top_x, top_y) = (area.top_left.x as f32, area.top_left.y as f32);
        let (bottom_x, bottom_y) = (bottom.x as f32, bottom.y as f32);

        Gx::begin(GX_QUADS as _, GX_VTXFMT0 as _, 4);
        Gx::position_3f32(top_x, top_y, 0.0);
        Gx::color_1u32(color);
        Gx::position_3f32(bottom_x, top_y, 0.0);
        Gx::color_1u32(color);
        Gx::position_3f32(bottom_x, bottom_y, 0.0);
        Gx::color_1u32(color);
        Gx::position_3f32(top_x, bottom_y, 0.0);
        Gx::color_1u32(color);
        Gx::end();

        Ok(())
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(640, 528)
    }
}
