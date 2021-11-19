use image_decoder_benchmarks::{
  IMAGES,
  load_image_to_memory,
  decode_jpeg_decoder,
  decode_turbojpeg,
};

fn main() {
    for image in IMAGES.iter() {
        let vec = load_image_to_memory(image);
        println!("{} turbo       : {:?}", image, decode_turbojpeg(&vec));
        println!("{} jpeg_decoder: {:?}", image, decode_jpeg_decoder(&vec));
    }
}