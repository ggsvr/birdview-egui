
//use sdl2::image;

#[cfg(not(target_os="linux"))]
pub const CAP_BACKEND: i32 = opencv::videoio::CAP_ANY;
#[cfg(target_os="linux")]
pub const CAP_BACKEND: i32 = opencv::videoio::CAP_V4L2;

use opencv::{
    core::{Vector, Mat},
    videoio::VideoCapture,
    prelude::*,
    imgcodecs,
};
use eframe::{egui, epi};

use crate::{Color, ColorData};


const IMG_SIZE: (usize, usize) = (600, 400);

pub struct BirdView {
    camera: VideoCapture,
    color_data: ColorData,
    raw_frame: Mat,
    processed_frame: Mat,
    is_raw: bool,
    framebuffer: Vector<u8>,
    tolerance: u8,
    texture: Option<(egui::TextureId, egui::Vec2)>,
}

impl BirdView {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            camera: VideoCapture::new(0, CAP_BACKEND)?,
            color_data: ColorData::new(Color::black(), Color::black(), Color::black()),
            raw_frame: Mat::default(),
            processed_frame: Mat::default(),
            is_raw: true,
            framebuffer: Vector::new(),
            tolerance: 0,
            texture: None,
        })
    }

    fn update_texture(&mut self, frame: &mut epi::Frame<'_>) {
        self.camera.read(&mut self.raw_frame).unwrap();

        crate::process_image(&self.raw_frame, &self.color_data, self.tolerance, &mut self.processed_frame);

        let frame_ref = if self.is_raw { &self.raw_frame } else { &self.processed_frame };

        let pixels: &[opencv::core::Vec3b] = frame_ref.data_typed().unwrap();
        let size = frame_ref.size().unwrap();
        let size = (size.width as usize, size.height as usize);

        let pixels: Vec<_> = pixels
            .iter()
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[2], p[1], p[0], 255))
            .collect();

        let texture = frame.tex_allocator().alloc_srgba_premultiplied(size, &pixels);
        let size = egui::Vec2::new(size.0 as f32, size.1 as f32);
        self.texture = Some((texture, size));

    }
}

impl epi::App for BirdView {
    
    fn name(&self) -> &str {
        "BirdView Client"
    }

    fn setup(&mut self, _ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>, _storage: Option<&dyn epi::Storage>) {
        
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {

        self.update_texture(frame);

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.image(self.texture.unwrap().0, self.texture.unwrap().1);
            ui.checkbox(&mut self.is_raw, "Raw");

        });

        ctx.request_repaint();
    }
}
