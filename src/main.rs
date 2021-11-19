use benchmarks::IMAGES;
use benchmarks::load_image_to_memory;
use benchmarks::decode_jpeg_decoder;
use benchmarks::decode_turbo;

fn main() {
    for image in IMAGES.iter() {
        let vec = load_image_to_memory(image);
        println!("{} turbo       : {:?}", image, decode_turbo(&vec));
        println!("{} jpeg_decoder: {:?}", image, decode_jpeg_decoder(&vec));
    }
}

// RUSTFLAGS="-L/usr/local/opt/jpeg-turbo/lib" cargo build
// https://stackoverflow.com/questions/40833078/how-do-i-specify-the-linker-path-in-rust

// brew install jpeg-turbo
// https://github.com/libjpeg-turbo/libjpeg-turbo/issues/447
// https://chromium.googlesource.com/chromium/deps/libjpeg_turbo/+/HEAD/BUILDING.md

/*
jpeg-turbo is keg-only, which means it was not symlinked into /usr/local,
because libjpeg-turbo is not linked to prevent conflicts with the standard libjpeg.

If you need to have jpeg-turbo first in your PATH, run:
  echo 'export PATH="/usr/local/opt/jpeg-turbo/bin:$PATH"' >> ~/.zshrc

For compilers to find jpeg-turbo you may need to set:
  export LDFLAGS="-L/usr/local/opt/jpeg-turbo/lib"
  export CPPFLAGS="-I/usr/local/opt/jpeg-turbo/include"

For pkg-config to find jpeg-turbo you may need to set:
  export PKG_CONFIG_PATH="/usr/local/opt/jpeg-turbo/lib/pkgconfig"
*/