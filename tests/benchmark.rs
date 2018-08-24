extern crate cv;

use cv::imgcodecs::*;
use cv::imgproc::*;
use cv::*;
mod utils;
use utils::*;

#[test]
fn bench_mat_new() {
    timed("create new mat", || {
        Mat::new();
    });
}

#[test]
fn bench_decode_lenna() {
    let buf = load_lenna_as_buf();
    timed("decode lenna.png", || {
        Mat::image_decode(&buf, ImageReadMode::Grayscale);
    });
}
