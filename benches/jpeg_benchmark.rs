use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};

use image_decoder_benchmarks::{
    IMAGES,
    load_image_to_memory,
    decode_jpeg_decoder,
    decode_turbojpeg,
};

use std::time::Duration;

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Images");
    group.measurement_time(Duration::from_secs(15));

    for image in IMAGES.iter() {
        let contents = load_image_to_memory(image);

        let size = contents.len();
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("turbo", image), &size,
            |b, _i| b.iter(|| decode_turbojpeg(black_box(&contents)))
        );
        group.bench_with_input(BenchmarkId::new("jpeg_decoder", image), &size,
            |b, _i| b.iter(|| decode_jpeg_decoder(black_box(&contents)))
        );
    }

    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
