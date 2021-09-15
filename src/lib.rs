#![allow(dead_code)]

pub mod data;
pub mod init;
mod math;

pub use init::*;

pub use data::{ColorHSV, ColorDataHSV, Point, PointData};
use opencv::core::Mat;



use opencv::core;
use opencv::imgproc;

pub fn process_image(src: &Mat, color_data: &ColorDataHSV, tolerance: f32, dst: &mut Mat) {
    assert!(tolerance >= 0.0 && tolerance <= 1.0);
    //let size = src.size().unwrap();
    //let (width, height) = (size.width, size.height);

    let lower_upper = |x: u16| {
        let diff = (x as f32 * (tolerance/2.0)) as u16;
        let lower = x - diff;
        let upper = x + diff;
        (lower, upper)
    };

    let mut hsv_img = Mat::default();
    let mut result = Mat::default();

    imgproc::cvt_color(src, &mut hsv_img, imgproc::COLOR_BGR2HSV, 0).unwrap();


    for (i, color) in color_data.colors.iter().enumerate() {
        let (h_lower, h_upper) = lower_upper(color.h());
        let (s_lower, s_upper) = lower_upper(color.s());
        let (v_lower, v_upper) = lower_upper(color.v());
        let lower = core::Scalar::from((h_lower.into(), s_lower.into(), v_lower.into()));
        let upper = core::Scalar::from((h_upper.into(), s_upper.into(), v_upper.into()));

        let mut mask = Mat::default();
        core::in_range(src, &lower, &upper, &mut mask).unwrap();

        if i == 0 {
            result = mask;
        } else {
            let tmp = result;
            result = Mat::default();
            core::bitwise_or(&tmp, &mask, &mut result, &core::no_array().unwrap()).unwrap();
        }
    }
    *dst = result;

}
