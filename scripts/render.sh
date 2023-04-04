#!/bin/bash -ex

# shellcheck disable=SC2012
benches=$(ls benches | sed -e 's/\.rs$//')

for dispatch in dynamic static-unstable fallback
do
    for bench in $benches
    do
        cat "$dispatch-$bench.jsonl" >> "$dispatch.jsonl"
    done
    python3 ./scripts/analyze.py "$dispatch.jsonl" > "$dispatch.md"
done

echo > result.md

echo "# Benchmark Results" > result.md
date -u --rfc-3339=s >> "result.md"

for dispatch in dynamic static-unstable fallback
do
    {
        echo "## $dispatch"
        echo
        cat "$dispatch.md" 
    } >> result.md
done

echo "## Environment" >> result.md
./scripts/print-env.sh >> result.md

mkdir dist
grip result.md --export dist/index.html --user-content --wide
./scripts/hack.py dist/index.html
