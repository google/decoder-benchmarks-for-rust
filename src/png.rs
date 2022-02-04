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

extern crate spng;

use std::io::Cursor;

pub const IMAGES: [&str; 9] = [
    "images/venice-2000x3000.png",
    "images/venice-1500x2250.png",
    "images/venice-1000x1500.png",
    "images/venice-500x750.png",
    "images/venice-200x300.png",
    "images/venice-100x150.png",
    "images/venice-50x75.png",
    "images/venice-20x30.png",
    "images/venice-10x15.png",
];

pub fn decode_spng(contents: &[u8]) -> (u32, u32) {
    let cursor = Cursor::new(contents);
    let decoder = spng::Decoder::new(cursor);
    let (info, mut reader) = decoder.read_info().expect("could not read info");
    let output_buffer_size = reader.output_buffer_size();
    let mut out = vec![0; output_buffer_size];
    reader.next_frame(&mut out).expect("could not decode image");
    (info.width, info.height)
}

pub fn decode_png(contents: &[u8]) -> (u32, u32) {
    let decoder = png::Decoder::new(contents);
    let mut reader = decoder.read_info().expect("could not read info");
    let output_buffer_size = reader.output_buffer_size();
    let mut out = vec![0; output_buffer_size];
    reader.next_frame(&mut out).expect("could not decode image");
    reader.info().size()
}