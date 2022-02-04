// Copyright 2021 Google LLC
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

use decoder_benchmarks_for_rust::read_file;
use decoder_benchmarks_for_rust::jpeg::{decode_jpeg_decoder, decode_turbojpeg, IMAGES as JPEGS};
use decoder_benchmarks_for_rust::png::{decode_png, decode_spng, IMAGES as PNGS};

fn main() {
    for image in JPEGS.iter() {
        let vec = read_file(image);
        println!("{} turbo       : {:?}", image, decode_turbojpeg(&vec));
        println!("{} jpeg_decoder: {:?}", image, decode_jpeg_decoder(&vec));
    }

    for image in PNGS.iter() {
        let data = read_file(image);
        println!("{} spng        : {:?}", image, decode_spng(&data));
        println!("{} png         : {:?}", image, decode_png(&data));
    }
}
