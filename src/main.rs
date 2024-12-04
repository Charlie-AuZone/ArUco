// use opencv::aruco::
use opencv::prelude::*;
fn main() {
    println!("{:?}", opencv::aruco::DICT_4X4_100);
    let detectors = opencv::objdetect::ArucoDetector::new();
}
