use std::os::raw::c_void;
use std::time::Instant;

use glob::glob;

use image::buffer::ConvertBuffer;
use image::GrayImage;
use opencv::core::{MatTraitConstManual, Mat_AUTO_STEP, Point2f, Scalar, Vector, CV_8UC1};
use opencv::objdetect::{
    draw_detected_markers, get_predefined_dictionary_i32, ArucoDetector, DetectorParameters,
    RefineParameters, DICT_4X4_50,
};
use opencv::prelude::{ArucoDetectorTraitConst, DetectorParametersTrait};

// use image::{codecs::png::PngDecoder, DynamicImage};
fn main() {
    let dict = get_predefined_dictionary_i32(DICT_4X4_50).unwrap();
    let mut detect_param = DetectorParameters::default().unwrap();
    detect_param.set_use_aruco3_detection(false);
    detect_param.set_adaptive_thresh_win_size_min(5);
    detect_param.set_adaptive_thresh_win_size_max(5);
    detect_param.set_min_marker_perimeter_rate(0.01);

    detect_param.set_min_marker_distance_rate(0.3);
    detect_param.set_min_corner_distance_rate(0.15);
    detect_param.set_min_marker_length_ratio_original_img(0.1);
    detect_param.set_perspective_remove_ignored_margin_per_cell(0.33);
    detect_param.set_min_side_length_canonical_img(32);
    let refine_param = RefineParameters::new_def().unwrap();
    let detector = ArucoDetector::new(&dict, &detect_param, refine_param).unwrap();

    for file in glob("./*.png").expect("Failed to read glob pattern") {
        let mut corners: Vector<Vector<Point2f>> = Vector::new();
        let mut rejected: Vector<Vector<Point2f>> = Vector::new();
        let mut marker_ids: Vector<i32> = Vector::new();
        if file.is_err() {
            continue;
        }
        let file = file.unwrap();
        let img = image::ImageReader::open(file.clone())
            .unwrap()
            .decode()
            .unwrap();
        let img = img.to_rgb8();
        let img: GrayImage = img.convert();
        let mut data = img.to_vec();
        let start = Instant::now();
        let mut img_mat = unsafe {
            opencv::prelude::Mat::new_rows_cols_with_data_unsafe(
                img.height() as i32,
                img.width() as i32,
                CV_8UC1,
                data.as_mut_ptr() as *mut c_void,
                Mat_AUTO_STEP,
            )
            .unwrap()
        };
        let img_prep = start.elapsed();
        let start = Instant::now();
        let detectors =
            detector.detect_markers(&img_mat, &mut corners, &mut marker_ids, &mut rejected);
        let detect_time = start.elapsed();
        match detectors {
            Ok(_) => {
                println!("Detect was okay {:?}", file);
                println!("{:?}", marker_ids.as_slice())
            }
            Err(e) => println!("Error while detecting: {:?}", e),
        }
        println!("Image prepare time: {:?}", img_prep);
        println!("Detect marker time: {:?}", detect_time);

        draw_detected_markers(
            &mut img_mat,
            &corners,
            &marker_ids,
            Scalar::new(0., 255., 0., 255.),
        )
        .unwrap();
        let output = img_mat.data_bytes().unwrap().to_vec();
        let img = GrayImage::from_raw(img.width(), img.height(), output).unwrap();
        let newname = format!(
            "found/{}.png",
            file.with_extension("").as_os_str().to_string_lossy()
        );
        let _ = img.save(newname);
    }
}
