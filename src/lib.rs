extern crate jpeg_decoder as jpeg;
extern crate turbojpeg as turbo;

use std::fs::File;
use std::io::Read;

pub const IMAGES: [&str; 9] = [
    "images/venice-2000x3000.jpg",
    "images/venice-1500x2250.jpg",
    "images/venice-1000x1500.jpg",
    "images/venice-500x750.jpg",
    "images/venice-200x300.jpg",
    "images/venice-100x150.jpg",
    "images/venice-50x75.jpg",
    "images/venice-20x30.jpg",
    "images/venice-10x15.jpg",
];

pub fn load_image_to_memory(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).expect("failed to open file");
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).expect("could not read file");

    contents
}

pub fn decode_turbojpeg(contents: &[u8]) -> (u32, u32) {
    let image: image::RgbImage = turbo::decompress_image(&contents)
        .expect("count not decompress image");
    image.dimensions()
}

pub fn decode_jpeg_decoder(contents: &[u8]) -> (u16, u16) {
    let mut decoder = jpeg::Decoder::new(contents);
    let _pixels = decoder.decode().expect("failed to decode image");
    // decoder.info() requires decode() to have been run first.
    let info = decoder.info().expect("failed to read image info");
    (info.width, info.height)
}
