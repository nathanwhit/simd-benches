use simd_benches::{map_collect, FnGroup};

use criterion::{black_box, criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, Throughput};
use once_cell::sync::Lazy;
use widestring::{Utf16Str, Utf16String};

fn load_dataset() -> &'static [(String, Utf16String)] {
    static DATASET: Lazy<Vec<(String, Utf16String)>> = Lazy::new(|| {
        map_collect(simd_benches::wikipedia_mars(), |(name, content)| {
            (name, Utf16String::from(content))
        })
    });
    DATASET.as_slice()
}

pub fn bench_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("utf16-check");

    let dataset = load_dataset();

    let functions: &FnGroup<fn(&[u16])> = &[
        ("simdutf/auto", |src: &[u16]| {
            assert!(simdutf::validate_utf16(src));
        }),
        // ("encoding_rs/auto", |src: &[u16]| {
        //     assert!(encoding_rs::mem::utf16_valid_up_to(src) == src.len()); //
        // }),
        ("widestring/fallback", |src: &[u16]| {
            assert!(widestring::Utf16Str::from_slice(src).is_ok());
        }),
    ];

    for &(name, f) in functions {
        for (case, input) in dataset {
            let input = input.as_slice();

            group.throughput(Throughput::Bytes(input.len() as u64 * 2));
            let id = BenchmarkId::new(name, case);

            group.bench_with_input(id, input, |b, src| b.iter(|| f(black_box(src))));
        }
    }
}

pub fn bench_to_utf8(c: &mut Criterion) {
    let mut group = c.benchmark_group("utf16-to-utf8");

    let dataset = load_dataset();

    let functions: &FnGroup<fn(&Utf16Str, &mut [u8])> = &[
        ("simdutf/auto", |src, dst| {
            simd_benches::simdutf_utf16_to_utf8(src, dst);
        }),
        ("encoding_rs/fallback", |src, dst| {
            simd_benches::encoding_rs_utf16_to_utf8(src, dst);
        }),
        ("widestring/fallback", |src, dst| {
            simd_benches::widestring_utf16_to_utf8(src, dst);
        }),
    ];

    let max_len = dataset.iter().map(|(_, s)| s.len()).max().unwrap();
    let mut dst: Vec<u8> = vec![0; max_len * 4];

    for &(name, f) in functions {
        for (case, input) in dataset {
            group.throughput(Throughput::Bytes(input.len() as u64 * 2));
            let id = BenchmarkId::new(name, case);

            group.bench_with_input(id, input.as_utfstr(), |b, src| {
                b.iter(|| f(black_box(src), black_box(dst.as_mut_slice())))
            });
        }
    }
}

criterion_group!(benches, bench_check, bench_to_utf8);
criterion_main!(benches);
