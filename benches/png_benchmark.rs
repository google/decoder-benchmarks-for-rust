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

use criterion::{black_box, Criterion, BenchmarkId, Throughput};

use decoder_benchmarks_for_rust::read_file;
use decoder_benchmarks_for_rust::png::{decode_png, decode_spng, IMAGES};

use std::time::Duration;

pub fn bench_pngs(c: &mut Criterion) {
    // TODO: Rename Images in the other one.
    let mut group = c.benchmark_group("pngs");
    group.measurement_time(Duration::from_secs(15));

    for image in IMAGES.iter() {
        let contents = read_file(image);

        let size = contents.len();
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("png", image), &size,
                               |b, _i| b.iter(|| decode_png(black_box(&contents)))
        );
        group.bench_with_input(BenchmarkId::new("spng", image), &size,
                               |b, _i| b.iter(|| decode_spng(black_box(&contents)))
        );
    }

    group.finish();
}
