dev:
    cargo fmt
    cargo clippy
    cargo build --benches --profile bench --all-features

quick-bench:
    RUSTFLAGS='-Ctarget-cpu=native' cargo run -p simd-benches --bin simd-benches --profile bench --features unstable

wasi-bench:
    ./scripts/wasi-bench.sh
