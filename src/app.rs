
#[cfg(not(target_os="linux"))]
pub const CAP_BACKEND: i32 = opencv::videoio::CAP_ANY;
#[cfg(target_os="linux")]
pub const CAP_BACKEND: i32 = opencv::videoio::CAP_V4L2;

use cgmath::MetricSpace;
use image::GenericImageView;
use image::GenericImage;
use serialport::SerialPort;

use rayon::prelude::*;

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
    port: Box<dyn SerialPort>,
}

impl BirdView {
    pub fn new(port: Box<dyn SerialPort>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            camera: VideoCapture::new(0, CAP_BACKEND)?,
            color_data: ColorData::new(Color::black(), Color::black(), Color::black()),
            point_data: PointData::new(),
            raw_frame: Mat::default(),
            processed_frame: Mat::default(),
            is_raw: true,
            tolerance: 0,
            texture: None,
            port,
        })
    }

    fn update_texture(&mut self, frame: &mut epi::Frame) {
        self.camera.read(&mut self.raw_frame).unwrap(); // read camera input to raw frame

        // process image and get points, storing in self
        self.point_data = img::process_image(&self.raw_frame, &self.color_data, self.tolerance, &mut self.processed_frame);

        // choose frame based on the user preference
        let frame_ref = if self.is_raw { &self.raw_frame } else { &self.processed_frame };

        // convert from opencv array to egui texture
        let pixels: &[opencv::core::Vec3b] = frame_ref.data_typed().unwrap();
        let size = frame_ref.size().unwrap();
        let size = (size.width as usize, size.height as usize);

        let pixels: Vec<_> = pixels
            .par_iter()
            .map(|p| egui::Color32::from_rgb(p[2], p[1], p[0]))
            .collect();

        let texture = frame.tex_allocator().alloc_srgba_premultiplied(size, &pixels);
        let size = egui::Vec2::new(size.0 as f32, size.1 as f32);


        if let Some((texture, _)) = self.texture {
            frame.tex_allocator().free(texture);
        }
        self.texture = Some((texture, size));

    }

    fn set_color_from_pointer(&mut self, rect: egui::Rect, ctx: &egui::CtxRef)  {
        use egui::Key;
        let input = ctx.input();

        let b_down = input.key_pressed(Key::B);
        let f_down = input.key_pressed(Key::F);
        let d_down = input.key_pressed(Key::D);

        if b_down || f_down || d_down {

            let pointer = input.pointer.hover_pos();
            if let Some(pointer) = pointer {

                if !rect.contains(pointer) { return }

                let offset = rect.left_top() - pointer;
                let offset = (offset.x.abs() as u32, offset.y.abs() as u32);

                let mut buffer = opencv::core::Vector::new();
                opencv::imgcodecs::imencode(".bmp", &self.raw_frame, &mut buffer, &opencv::core::Vector::new()).unwrap();

                let img = image::load_from_memory(buffer.as_slice()).unwrap();

                let color = img.get_pixel(offset.0, offset.1);
                let color = Color::new(color.0[0], color.0[1], color.0[2]);

                if b_down {
                    *self.color_data.back_mut() = color;
                }
                if f_down {
                    *self.color_data.front_mut() = color;
                }
                if d_down {
                    *self.color_data.dest_mut() = color;
                }
            }

        } 

    }

    fn save_image(&self) {
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
}

impl epi::App for BirdView {
    
    fn name(&self) -> &str {
        "BirdView Client"
    }

    fn setup(&mut self, _ctx: &egui::CtxRef, _frame: &mut epi::Frame, _storage: Option<&dyn epi::Storage>) {
        
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame) {

        self.update_texture(frame);

        if let Err(e) = self.point_data.write_serial(&mut *self.port) {
            eprintln!("data not send over serial: {}", e);
        }

        egui::CentralPanel::default().show(ctx, |ui| {

            let response = ui.image(self.texture.unwrap().0, self.texture.unwrap().1);
            self.set_color_from_pointer(response.rect, ctx);
            //let pointer = ui.input().pointer.hover_pos();

            //if let Some(pos) = pointer {
            //    if response.rect.contains(pos) { println!("yes"); }
            //    else { println!("no") }

            //} else { println!("Can't get pointer pos"); }


            ui.checkbox(&mut self.is_raw, "Raw");

            //ui.color_edit_button_srgb(&mut self.color_data.back_mut().channels);
            //ui.color_edit_button_srgb(&mut self.color_data.front_mut().channels);
            //ui.color_edit_button_srgb(&mut self.color_data.dest_mut().channels);

            ui.add(egui::Slider::new(&mut self.tolerance, 0..=255));

            if ui.button("Save Points").clicked() {
                self.save_image();
            }
        });

        ctx.request_repaint();
    }
}
