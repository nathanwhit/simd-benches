use simd_benches::FnGroup;

use criterion::{black_box, criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, Throughput};
use once_cell::sync::Lazy;

fn load_dataset() -> &'static [(String, String)] {
    static DATASET: Lazy<Vec<(String, String)>> = Lazy::new(simd_benches::wikipedia_mars);
    DATASET.as_slice()
}

pub fn bench_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("utf8-check");

    let dataset = load_dataset();

    let functions: &FnGroup<fn(&[u8])> = &[
        ("simdutf8/auto", |src: &[u8]| {
            assert!(simdutf8::basic::from_utf8(src).is_ok());
        }),
        ("simdutf/auto", |src: &[u8]| {
            assert!(simdutf::validate_utf8(src));
        }),
        ("encoding_rs/fallback", |src: &[u8]| {
            assert!(encoding_rs::Encoding::utf8_valid_up_to(src) == src.len()); //
        }),
        ("std/fallback", |src: &[u8]| {
            assert!(std::str::from_utf8(src).is_ok());
        }),
    ];

    for &(name, f) in functions {
        for (case, input) in dataset {
            let input = input.as_bytes();

            group.throughput(Throughput::Bytes(input.len() as u64));
            let id = BenchmarkId::new(name, case);

            group.bench_with_input(id, input, |b, src| b.iter(|| f(black_box(src))));
        }
    }
}

pub fn bench_to_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("utf8-to-utf16");

    let dataset = load_dataset();

    let functions: &FnGroup<fn(&str, &mut [u16])> = &[
        ("simdutf/auto", |src, dst| {
            simd_benches::simdutf_utf8_to_utf16(src, dst);
        }),
        ("std/fallback", |src, dst| {
            simd_benches::std_utf8_to_utf16(src, dst);
        }),
    ];

    let max_len = dataset.iter().map(|(_, s)| s.len()).max().unwrap();
    let mut dst: Vec<u16> = vec![0; max_len * 2];

    for &(name, f) in functions {
        for (case, input) in dataset {
            group.throughput(Throughput::Bytes(input.len() as u64));
            let id = BenchmarkId::new(name, case);

            group.bench_with_input(id, input.as_str(), |b, src| {
                b.iter(|| f(black_box(src), black_box(dst.as_mut_slice())))
            });
        }
    }
}

criterion_group!(benches, bench_check, bench_to_utf16);
criterion_main!(benches);
