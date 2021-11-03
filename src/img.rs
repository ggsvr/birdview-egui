use crate::data::{Color, ColorData, Point, PointData};
use opencv::core::Mat;

use opencv::core;
use opencv::features2d::{SimpleBlobDetector, SimpleBlobDetector_Params};
use opencv::prelude::*;


pub fn process_image(src: &Mat, color_data: &ColorData, tolerance: u8, dst: &mut Mat) -> PointData {
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
    let mut output = PointData::new();

    //imgproc::cvt_color(src, &mut hsv_img, imgproc::COLOR_BGR2HSV, 0).unwrap();

    let mut blob_params = SimpleBlobDetector_Params::default().unwrap();
    blob_params.filter_by_color = true;
    blob_params.blob_color = 255;

    blob_params.filter_by_area = false;
    blob_params.filter_by_circularity = false;
    blob_params.filter_by_convexity = false;
    blob_params.filter_by_inertia = false;

    let mut blob_detector = SimpleBlobDetector::create(blob_params).unwrap();


    for (i, color) in color_data.colors.iter().enumerate() {
        //let (r_lower, r_upper) = lower_upper(color.r() as u16);
        //let (g_lower, g_upper) = lower_upper(color.g() as u16);
        //let (b_lower, b_upper) = lower_upper(color.b() as u16);
        let (lower, upper) = lower_upper(*color);
        let lower = core::Scalar::from((lower.b().into(), lower.g().into(), lower.r().into()));
        let upper= core::Scalar::from((upper.b().into(), upper.g().into(), upper.r().into()));


        let mut mask = Mat::default();
        core::in_range(src, &lower, &upper, &mut mask).unwrap();

        let mut keypoints: core::Vector<core::KeyPoint> = core::Vector::new();

        blob_detector.detect(&mask, &mut keypoints, &core::no_array().unwrap()).unwrap();

        output.points[i] = match keypoints.get(0) {
            Ok(k) => Some(Point::new(k.pt.x.into(), k.pt.y.into())),
            Err(_) => None,
        };



        if i == 0 {
            result = mask;
        } else {
            let tmp = result;
            result = Mat::default();
            core::bitwise_or(&tmp, &mask, &mut result, &core::no_array().unwrap()).unwrap();
        }
    }
    opencv::imgproc::cvt_color(&result, dst, opencv::imgproc::COLOR_GRAY2BGR, 3).unwrap();
    output

}