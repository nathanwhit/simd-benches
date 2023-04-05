dev:
    cargo fmt
    cargo clippy
    cargo build --benches --profile bench --all-features

quick-bench:
    RUSTFLAGS='-Ctarget-cpu=native' cargo run -p simd-benches --bin simd-benches --profile bench --features unstable

wasi-bench:
    ./scripts/wasi-bench.sh

bench dispatch bench *ARGS:
    DISPATCH={{dispatch}} BENCH={{bench}} ./scripts/bench.sh \
        --plotting-backend disabled -- --warm-up-time 1 --measurement-time 1 {{ARGS}}
