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

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};

use decoder_benchmarks_for_rust::{
    IMAGES,
    load_image_to_memory,
    decode_jpeg_decoder,
    decode_turbojpeg,
};

use std::time::Duration;

fn bench_jpegs(c: &mut Criterion) {
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

criterion_group!(benches, bench_jpegs);
criterion_main!(benches);
