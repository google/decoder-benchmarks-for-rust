// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate jpeg_decoder;
extern crate turbojpeg;

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

pub fn decode_turbojpeg(contents: &[u8]) -> (u32, u32) {
    let image: image::RgbImage = turbojpeg::decompress_image(&contents)
        .expect("could not decompress image");
    image.dimensions()
}

pub fn decode_jpeg_decoder(contents: &[u8]) -> (u16, u16) {
    let mut decoder = jpeg_decoder::Decoder::new(contents);
    let _pixels = decoder.decode().expect("failed to decode image");
    // decoder.info() requires decode() to have been run first.
    let info = decoder.info().expect("failed to read image info");
    (info.width, info.height)
}