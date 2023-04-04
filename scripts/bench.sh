#!/bin/bash -ex

if [ -z "$BENCH" ]; then
    echo "BENCH not set"
    exit 1
fi

commit_hash=$(git rev-parse --short HEAD)
mkdir -p target/simd-benches/"$commit_hash"

case $DISPATCH in
    static)
        export RUSTFLAGS="-C target-cpu=native"
        features=""
        ;;
    static-unstable)
        export RUSTFLAGS="-C target-cpu=native"
        features="unstable"
        ;;
    static-experimental)
        export RUSTFLAGS="-C target-cpu=native"
        features="unstable,parallel"
        ;;
    dynamic)
        export RUSTFLAGS=""
        features="detect"
        ;;
    fallback)
        export RUSTFLAGS=""
        features=""
        ;;
    *)
        echo "Unknown dispatch: $DISPATCH"
        exit 1
        ;;
esac

name=target/simd-benches/$commit_hash/$DISPATCH-$BENCH

time cargo criterion -p simd-benches --bench "$BENCH" \
    --history-id "$commit_hash" --message-format json --features "$features" "$@" > "$name.jsonl"

python3 ./scripts/analyze.py "$name.jsonl" > "$name.md"

if which bat; then
    bat --paging=never "$name.md"
else
    cat "$name.md"
fi
