
//use sdl2::image;

#[cfg(not(target_os="linux"))]
pub const CAP_BACKEND: i32 = opencv::videoio::CAP_ANY;
#[cfg(target_os="linux")]
pub const CAP_BACKEND: i32 = opencv::videoio::CAP_V4L2;

use cgmath::MetricSpace;
use image::GenericImageView;
use image::GenericImage;

use opencv::{
    core::Mat,
    videoio::VideoCapture,
    prelude::*,
};
use eframe::{egui, epi};

use crate::{Color, ColorData, PointData, data::Data, img, math};


const IMG_SIZE: (usize, usize) = (600, 400);

pub struct BirdView {
    camera: VideoCapture,
    color_data: ColorData,
    point_data: PointData,
    raw_frame: Mat,
    processed_frame: Mat,
    is_raw: bool,
    tolerance: u8,
    texture: Option<(egui::TextureId, egui::Vec2)>,
}

impl BirdView {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            camera: VideoCapture::new(0, CAP_BACKEND)?,
            color_data: ColorData::new(Color::black(), Color::black(), Color::black()),
            point_data: PointData::new(),
            raw_frame: Mat::default(),
            processed_frame: Mat::default(),
            is_raw: true,
            tolerance: 0,
            texture: None,
        })
    }

    fn update_texture(&mut self, frame: &mut epi::Frame) {
        self.camera.read(&mut self.raw_frame).unwrap(); // read camera input to raw frame

        //process image and get points
        let points = img::process_image(&self.raw_frame, &self.color_data, self.tolerance, &mut self.processed_frame);

        let frame_ref = if self.is_raw { &self.raw_frame } else { &self.processed_frame };


        let pixels: &[opencv::core::Vec3b] = frame_ref.data_typed().unwrap();
        let size = frame_ref.size().unwrap();
        let size = (size.width as usize, size.height as usize);

        let pixels: Vec<_> = pixels
            .iter()
            //.map(|p| egui::Color32::from_rgba_unmultiplied(p[2], p[1], p[0], 255))
            .map(|p| egui::Color32::from_rgb(p[2], p[1], p[0]))
            .collect();

        let texture = frame.tex_allocator().alloc_srgba_premultiplied(size, &pixels);
        let size = egui::Vec2::new(size.0 as f32, size.1 as f32);

        self.point_data = points;

        if let Some((texture, _)) = self.texture {
            frame.tex_allocator().free(texture);
        }
        self.texture = Some((texture, size));

    }
}

impl epi::App for BirdView {
    
    fn name(&self) -> &str {
        "BirdView Client"
    }

    fn setup(&mut self, _ctx: &egui::CtxRef, _frame: &mut epi::Frame, _storage: Option<&dyn epi::Storage>) {
        
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame) {

        self.update_texture(frame);

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.image(self.texture.unwrap().0, self.texture.unwrap().1);
            ui.checkbox(&mut self.is_raw, "Raw");

            ui.color_edit_button_srgb(&mut self.color_data.back_mut().channels);
            ui.color_edit_button_srgb(&mut self.color_data.front_mut().channels);
            ui.color_edit_button_srgb(&mut self.color_data.dest_mut().channels);

            ui.add(egui::Slider::new(&mut self.tolerance, 0..=255));

            if ui.button("Save Points").clicked() {

                let mut buffer = opencv::core::Vector::new();
                opencv::imgcodecs::imencode(".bmp", &self.raw_frame, &mut buffer, &opencv::core::Vector::new()).unwrap();

                let img = image::load_from_memory(buffer.as_slice()).unwrap();
                let mut out = img.clone();

                for pixel in img.pixels() {
                    let pt1 = math::Point::new(pixel.0 as f64, pixel.1 as f64);

                    for pt in self.point_data.list() {
                        if let Some(pt2) = *pt {
                            if pt1.distance(pt2) < 5.0 {
                                out.put_pixel(pixel.0, pixel.1, image::Rgba([255, 255, 255, 255]));
                            }
                        }
                    }

                }



                out.save_with_format("./points.jpeg", image::ImageFormat::Jpeg).unwrap();
            }
        });

        ctx.request_repaint();
    }
}
