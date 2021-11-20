use crate::data::{Color, ColorData, Point, PointData};
use opencv::core::Mat;

use opencv::core::{self, Ptr};
use opencv::features2d::{SimpleBlobDetector, SimpleBlobDetector_Params};
use opencv::prelude::*;

#[inline(always)]
fn lower_upper(color: Color, tolerance: u8) -> (Color, Color) {
    (color - tolerance/2, color + tolerance/2)
}

fn create_blob_detector() -> Ptr<SimpleBlobDetector> {
    let mut blob_params = SimpleBlobDetector_Params::default().unwrap();
    blob_params.filter_by_color = true;
    blob_params.blob_color = 255;

    blob_params.filter_by_area = false;
    blob_params.filter_by_circularity = false;
    blob_params.filter_by_convexity = false;
    blob_params.filter_by_inertia = false;

    SimpleBlobDetector::create(blob_params).unwrap()
}

pub fn process_image(src: &Mat, color_data: &ColorData, tolerance: u8, dst: &mut Mat) -> PointData {


    let mut result = Mat::default();
    let mut output = PointData::new();


    let mut blob_detector = create_blob_detector();


    for (i, color) in color_data.colors.iter().enumerate() {

        let (lower, upper) = lower_upper(*color, tolerance);
        let lower = core::Scalar::from((lower.b().into(), lower.g().into(), lower.r().into()));
        let upper= core::Scalar::from((upper.b().into(), upper.g().into(), upper.r().into()));


        let mut mask = Mat::default();
        core::in_range(src, &lower, &upper, &mut mask).unwrap();

        let mut keypoints: core::Vector<core::KeyPoint> = core::Vector::new();

        blob_detector.detect(&mask, &mut keypoints, &core::no_array()).unwrap();

        output.points[i] = match keypoints.get(0) {
            Ok(k) => Some(Point::new(k.pt.x.into(), k.pt.y.into())),
            Err(_) => None,
        };



        if i == 0 {
            result = mask;
        } else {
            let tmp = result;
            result = Mat::default();
            core::bitwise_or(&tmp, &mask, &mut result, &core::no_array()).unwrap();
        }
    }
    opencv::imgproc::cvt_color(&result, dst, opencv::imgproc::COLOR_GRAY2BGR, 3).unwrap();
    output

}