use std::{io::Cursor, os::raw::c_void};

use image::{codecs::png::PngDecoder, DynamicImage};
use opencv::core::{no_array, Mat_AUTO_STEP, Point2f, Vector, CV_8UC3};
use opencv::objdetect::{
    get_predefined_dictionary_i32, ArucoDetector, DetectorParameters, RefineParameters,
    DICT_6X6_250,
};
use opencv::prelude::ArucoDetectorTraitConst;
fn main() {
    let dict = get_predefined_dictionary_i32(DICT_6X6_250).unwrap();
    let mut corners: Vector<Vector<Point2f>> = Vector::new();
    let mut rejected = no_array();
    let mut marker_ids: Vector<i32> = Vector::new();
    let detect_param = DetectorParameters::default().unwrap();
    let refine_param = RefineParameters::new_def().unwrap();
    let detector = ArucoDetector::new(&dict, &detect_param, refine_param).unwrap();

    let mut png = Cursor::new(include_bytes!("../test.png"));
    let decoder = PngDecoder::new(&mut png).unwrap();
    let img = DynamicImage::from_decoder(decoder).unwrap();
    let img = img.to_rgb8();
    let mut data = img.to_vec();

    let img = unsafe {
        opencv::prelude::Mat::new_rows_cols_with_data_unsafe(
            img.height() as i32,
            img.width() as i32,
            CV_8UC3,
            data.as_mut_ptr() as *mut c_void,
            Mat_AUTO_STEP,
        )
        .unwrap()
    };
    // let img = opencv::imgcodecs::imread("./test.png", ImreadModes::IMREAD_COLOR.into()).unwrap();

    let detectors = detector.detect_markers(&img, &mut corners, &mut marker_ids, &mut rejected);
    match detectors {
        Ok(_) => {
            println!("Detect was okay");
            println!("{:?}", marker_ids.as_slice())
        }
        Err(e) => println!("Error while detecting: {:?}", e),
    }
}
