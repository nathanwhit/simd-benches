#!/usr/bin/python3
from dataclasses import dataclass
from typing import Any, Dict, List, Optional
import sys
import json
import datetime
from pathlib import Path
import os

from tabulate import tabulate

BENCH_FUNCTIONS = [
    {"name": "base64-check", "metric": "throughput"},
    {"name": "base64-decode", "metric": "throughput"},
    {"name": "base64-encode", "metric": "throughput"},
    {"name": "base64-forgiving-decode", "metric": "throughput"},
    {"name": "hex-check", "metric": "throughput"},
    {"name": "hex-decode", "metric": "throughput"},
    {"name": "hex-encode", "metric": "throughput"},
    {"name": "base32-check", "metric": "throughput"},
    {"name": "base32-decode", "metric": "throughput"},
    {"name": "base32-encode", "metric": "throughput"},
    {"name": "uuid-format", "metric": "latency"},
    {"name": "uuid-parse", "metric": "latency"},
    {"name": "ascii-check", "metric": "throughput"},
    {"name": "utf8-check", "metric": "throughput"},
    {"name": "utf8-to-utf16", "metric": "throughput"},
    {"name": "utf16-check", "metric": "throughput"},
    {"name": "utf16-to-utf8", "metric": "throughput"},
]


@dataclass
class BenchResult:
    name: str
    metric: str
    functions: List[str]
    cases: List[str]
    data: Dict[str, Dict[str, float]]


def read_jsonl(path: str):
    with open(path) as f:
        for line in f.readlines():
            line = line.strip()
            if not line:
                continue
            yield json.loads(line)


def convert_criterion_jsonl(messages: List[Any]):
    for msg in messages:
        reason: str = msg["reason"]
        if reason != "benchmark-complete":
            continue

        parts = msg["id"].split("/")
        bench = parts[0]
        crate = parts[1]
        variant = parts[2]
        case = parts[3]

        time: float = msg["typical"]["estimate"]
        assert msg["typical"]["unit"] == "ns"

        if len(msg["throughput"]) > 0:
            input_len: Optional[int] = msg["throughput"][0]["per_iteration"]
            assert msg["throughput"][0]["unit"] == "bytes"
        else:
            input_len = None

        yield {
            "bench": bench,
            "crate": crate,
            "variant": variant,
            "case": case,
            "time": time,
            "input_len": input_len,
        }


def append_if_not_exists(l, x):
    if x not in l:
        l.append(x)


def find(l, f):
    for x in l:
        if f(x):
            return x
    raise Exception()


def gather_results(items: List[Any]) -> List[BenchResult]:
    results: Dict[str, BenchResult] = {}

    for item in items:
        name = item["bench"]
        metric = find(BENCH_FUNCTIONS, lambda x: x["name"] == name)["metric"]
        r = results.setdefault(name, BenchResult(name, metric, [], [], {}))

        function = f'{item["crate"]}/{item["variant"]}'
        append_if_not_exists(r.functions, function)

        case = item["case"]
        append_if_not_exists(r.cases, case)

        time = item["time"]

        if metric == "throughput":
            input_len = item["input_len"]
            throughput = input_len / time * 1e9 / (1 << 30)  # GiB/s
            data = throughput
        elif metric == "latency":
            data = time
        else:
            raise Exception()

        row = r.data.setdefault(function, {})
        row[case] = data

    results_list = list(results.values())
    results_list.sort(
        key=lambda x: position(BENCH_FUNCTIONS, lambda y: y["name"] == x.name)
    )
    return results_list


def position(l, f):
    for i, x in enumerate(l):
        if f(x):
            return i
    raise Exception()


@dataclass
class BenchResultTable:
    name: str
    headers: List[str]
    table: List[List[str]]


def generate_table(result: BenchResult) -> BenchResultTable:
    headers = [""] + result.cases

    table = []
    for function, data in result.data.items():
        row = [function]
        for case in result.cases:
            col = list(result.data[f][case] for f in result.functions)

            if result.metric == "throughput":
                needs_bold = data[case] == max(col)
            elif result.metric == "latency":
                needs_bold = data[case] == min(col)
            else:
                raise Exception()

            if needs_bold:
                cell = f"**{data[case]:5.3f}**"
            else:
                cell = f"  {data[case]:5.3f}  "

            row.append(cell)

        table.append(row)

    return BenchResultTable(result.name, headers, table)


def now_rfc3339() -> str:
    return datetime.datetime.now().strftime("%Y-%m-%dT%H:%M:%SZ")


METRIC_TO_UNIT = {"throughput": "GiB/s", "latency": "ns"}


def single_mode(path: str):
    messages = list(read_jsonl(path))
    items = list(convert_criterion_jsonl(messages))
    results = gather_results(items)

    for result in results:
        t = generate_table(result)
        unit = METRIC_TO_UNIT[result.metric]

        print(f"#### {t.name} ({unit})")
        print()
        print(tabulate(t.table, t.headers, tablefmt="github"))
        print()


def render_mode():
    benches = []
    for bench_function in BENCH_FUNCTIONS:
        bench = bench_function["name"].split("-")[0]
        append_if_not_exists(benches, bench)

    dispatches = ["dynamic", "static-unstable", "static", "fallback"]

    # [bench_function][dispatch] = table
    tables: Dict[str, Dict[str, BenchResultTable]] = {}

    for bench in benches:
        for dispatch in dispatches:
            path = Path(f"{dispatch}-{bench}.jsonl")
            if not path.exists():
                continue

            messages = list(read_jsonl(str(path)))
            items = list(convert_criterion_jsonl(messages))
            results = gather_results(items)

            for result in results:
                t = generate_table(result)
                tables.setdefault(t.name, {})[dispatch] = t

    print("# Benchmark Results")
    print(now_rfc3339())
    print()

    for bench_function, d in tables.items():
        metric = find(BENCH_FUNCTIONS, lambda x: x["name"] == bench_function)["metric"]
        unit = METRIC_TO_UNIT[metric]

        print(f"### {bench_function} ({unit})")
        print()

        for dispatch, t in d.items():
            print(f"#### {dispatch}")
            print()
            print(tabulate(t.table, t.headers, tablefmt="github"))
            print()

    print("## Environment")
    print(flush=True)
    os.system("./scripts/print-env.sh")


if __name__ == "__main__":
    if len(sys.argv) == 1:
        render_mode()
    elif len(sys.argv) == 2:
        path = sys.argv[1]
        single_mode(path)
