use simd_benches::FnGroup;

use criterion::{black_box, criterion_group, criterion_main, AxisScale, PlotConfiguration};
use criterion::{BenchmarkId, Criterion, Throughput};

fn load_dataset() -> Vec<(String, Vec<u8>)> {
    let dir = std::fs::read_dir("dataset/wikipedia_mars").unwrap();
    let mut ans = Vec::new();
    for entry in dir {
        let entry = entry.unwrap();
        let name = entry.file_name().to_str().unwrap().to_string();
        let content = std::fs::read(entry.path()).unwrap();
        ans.push((name, content));
    }
    ans.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
    ans
}

pub fn bench_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("utf8-check");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

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
        for (case, input) in &dataset {
            group.throughput(Throughput::Bytes(input.len() as u64));
            let id = BenchmarkId::new(name, case);

            group.bench_with_input(id, input.as_slice(), |b, src| b.iter(|| f(black_box(src))));
        }
    }
}

criterion_group!(benches, bench_check);
criterion_main!(benches);
