#!/bin/bash -ex

./scripts/analyze.py > result.md

mkdir dist
grip result.md --export dist/index.html --user-content --wide --title simd-benches --context Nugine/simd-benches
./scripts/hack.py dist/index.html
