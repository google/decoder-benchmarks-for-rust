use criterion::{black_box, criterion_group, criterion_main, Criterion};

use benchmarks::IMAGES;
use benchmarks::load_image_to_memory;
use benchmarks::decode_jpeg_decoder;
use benchmarks::decode_turbo;

// TODO: Look at comparing functions.
// TODO: Look at benchmarking with arguments.

fn bench_fibs(c: &mut Criterion) {
    for image in IMAGES.iter() {
        let contents = load_image_to_memory(image);

        c.bench_function(format!("jpeg_decoder, {}", image).as_str(),
            |b| b.iter(|| decode_jpeg_decoder(black_box(&contents)))
        );
        c.bench_function(format!("turbo, {}", image).as_str(),
            |b| b.iter(|| decode_turbo(black_box(&contents)))
        );
    }
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
