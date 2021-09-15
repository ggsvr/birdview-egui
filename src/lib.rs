#![allow(dead_code)]

pub mod data;
pub mod init;
mod math;

pub use init::*;

pub use data::{Color, ColorData, Point, PointData};
use opencv::core::Mat;



use opencv::core;

pub fn process_image(src: &Mat, color_data: &ColorData, tolerance: u8, dst: &mut Mat) {
    //let size = src.size().unwrap();
    //let (width, height) = (size.width, size.height);

    let mut cols = [Mat::default(), Mat::default(), Mat::default()];

    for (i, color) in color_data.colors.iter().enumerate() {
        let lower = *color - tolerance / 2;
        let upper = *color + tolerance / 2;

        core::in_range(
            &src,
            &core::Scalar::from((lower.r() as f64, lower.g() as f64, lower.b() as f64)),
            &core::Scalar::from((upper.r() as f64, upper.g() as f64, upper.b() as f64)),
           &mut cols[i]
        ).unwrap();

    }
    let [r, g, b] = cols;
    *dst = r;
}