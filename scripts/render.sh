#!/bin/bash -ex

# shellcheck disable=SC2012
benches=$(ls benches | sed -e 's/\.rs$//')

echo "# Benchmark Results" > result.md
date -u --rfc-3339=s >> "result.md"

for bench in $benches
do
    echo "## $bench" >> result.md
    echo >> result.md
    for dispatch in dynamic static-unstable fallback
    do
        {
            echo "### $dispatch"
            echo
            python3 ./scripts/analyze.py "$dispatch-$bench.jsonl"
        } >> result.md
    done
done

echo "## Environment" >> result.md
./scripts/print-env.sh >> result.md

mkdir dist
grip result.md --export dist/index.html --user-content --wide --title simd-benches --context Nugine/simd-benches
./scripts/hack.py dist/index.html
