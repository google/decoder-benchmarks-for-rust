# Benchmarks for Rust Image Decoders

This project runs benchmarks comparing Rust's native image decoders to ones written in C/C++.

## Running

To run these, you will need to have libjpeg-turbo installed and in your library path.
For example on Mac OS:

```sh
> brew install jpeg-turbo
> RUSTFLAGS="-L/usr/local/opt/jpeg-turbo/lib" cargo bench
```

## Results

The following results come from a run on a MacOS device with a 2.6GHz 6-Core Intel Core i7 processor and 16GB 2667 MHz DDR4 RAM.

![Line graph displaying JPEG decoding time of turbojpeg and jpeg-decoder on various image sizes.](lines.svg)

This data is also presented in the graph below.
Very roughly speaking, `jpeg-decoder` is slower than `turbojpeg`, taking roughly an additional 30%
of time to decode the same image for larger images, but much longer on smaller images.

| Test case | jpeg-decoder (ms) | turbojpeg (ms) | jpeg-decoder / turbojpeg |
| :--- | ---: | ---: | ---: |
| venice-10x15.jpg | 0.2198 | 0.02008 | 1095% |
| venice-20x30.jpg | 0.2224 | 0.02691 | 826% |
| venice-50x75.jpg | 0.3036 | 0.09911 | 306% |
| venice-100x150.jpg | 0.5583 | 0.301 | 186% |
| venice-200x300.jpg | 1.517 | 0.9919 | 153% |
| venice-500x750.jpg | 7.658 | 5.723 | 134% |
| venice-1000x1500.jpg | 28.86 | 22.1 | 131% |
| venice-1500x2250.jpg | 62.59 | 48.88 | 128% |
| venice-2000x3000.jpg | 108.0 | 84.41 | 128% |

## Further work

* Cover more image decoders.
* Find a good corpus of images to use.
* Investigate JPEG decoding at low image sizes.
