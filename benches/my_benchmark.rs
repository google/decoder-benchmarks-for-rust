use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};

use benchmarks::IMAGES;
use benchmarks::load_image_to_memory;
use benchmarks::decode_jpeg_decoder;
use benchmarks::decode_turbo;

use std::time::Duration;

// TODO: Look at comparing functions.
// TODO: Look at benchmarking with arguments.

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Images");
    group.measurement_time(Duration::from_secs(15));

    for image in IMAGES.iter() {
        let contents = load_image_to_memory(image);

        let (width, height) = decode_turbo(black_box(&contents));
        let size = width * height;

        group.throughput(Throughput::Bytes(size.into()));

        group.bench_with_input(BenchmarkId::new("turbo", image), &size,
            |b, _i| b.iter(|| decode_turbo(black_box(&contents)))
        );
        group.bench_with_input(BenchmarkId::new("jpeg_decoder", image), &size,
            |b, _i| b.iter(|| decode_jpeg_decoder(black_box(&contents)))
        );

        // c.bench_function(format!("jpeg_decoder, {}", image).as_str(),
        //     |b| b.iter(|| decode_jpeg_decoder(black_box(&contents)))
        // );
        // c.bench_function(format!("turbo, {}", image).as_str(),
        //     |b| b.iter(|| decode_turbo(black_box(&contents)))
        // );
    }

    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
