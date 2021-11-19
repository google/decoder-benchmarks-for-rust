extern crate jpeg_decoder as jpeg;

use std::fs::File;
use std::io::Read;
use turbojpeg::decompress_image;

pub const IMAGES: [&str; 4] = [
    "images/venice-2000x3000.jpg",
    "images/venice-1000x1500.jpg",
    "images/venice-500x750.jpg",
    "images/venice-100x150.jpg",
];

// TODO: Do these functions do the same thing?
// TODO: We probably want to separate out the file reading.
// TODO: Check we're actually loading things properly.

// turbojpeg takes a &[u8] and returns a Result<ImageBuffer<P, Vec<u8>>>
// Where is ImageBuffer from? The image crate.

pub fn load_image_to_memory(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).expect("failed to open file");
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).expect("could not read file");

    contents
}

pub fn decode_turbo(contents: &[u8]) -> (u32, u32) {
    let image: image::RgbImage = decompress_image(&contents).expect("count not decompress image");
    image.dimensions()
}

pub fn decode_jpeg_decoder(contents: &[u8]) -> (u16, u16) {
    let mut decoder = jpeg::Decoder::new(contents);
    let _pixels = decoder.decode().expect("failed to decode image");
    // decoder.info() requires decode() to have been run first.
    let info = decoder.info().expect("failed to read image info");
    (info.width, info.height)
}
