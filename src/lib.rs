#![allow(dead_code)]

pub mod data;
pub mod init;
mod math;

pub use init::*;

pub use data::{Color, ColorData, Point, PointData};
use opencv::core::Mat;



use opencv::core;
use opencv::core::MatTraitConst;

pub fn process_image(src: &Mat, color_data: &ColorData, tolerance: u8, dst: &mut Mat) {
    //let size = src.size().unwrap();
    //let (width, height) = (size.width, size.height);

    let lower_upper = |x: Color| {
        //let diff = (x as f32 * (tolerance/2.0)) as u16;
        let lower = x - tolerance/2;
        let upper = x + tolerance/2;
        (lower, upper)
    };

    //let mut hsv_img = Mat::default();
    let mut result = Mat::default();

    //imgproc::cvt_color(src, &mut hsv_img, imgproc::COLOR_BGR2HSV, 0).unwrap();


    for (i, color) in color_data.colors.iter().enumerate() {
        //let (r_lower, r_upper) = lower_upper(color.r() as u16);
        //let (g_lower, g_upper) = lower_upper(color.g() as u16);
        //let (b_lower, b_upper) = lower_upper(color.b() as u16);
        let (lower, upper) = lower_upper(*color);
        let lower = core::Scalar::from((lower.b().into(), lower.g().into(), lower.r().into()));
        let upper= core::Scalar::from((upper.b().into(), upper.g().into(), upper.r().into()));


        let mut mask = Mat::default();
        core::in_range(src, &lower, &upper, &mut mask).unwrap();

        if i == 0 {
            result = mask;
        } else {
            let tmp = result;
            result = Mat::default();
            println!("tmp: {}, mask: {}, result: {}", tmp.typ().unwrap(), mask.typ().unwrap(), result.typ().unwrap());
            core::bitwise_or(&tmp, &mask, &mut result, &core::no_array().unwrap()).unwrap();
        }
    }
    println!("{}", result.typ().unwrap());
    opencv::imgproc::cvt_color(&result, dst, opencv::imgproc::COLOR_GRAY2BGR, 3).unwrap();
    *dst = result;

}
