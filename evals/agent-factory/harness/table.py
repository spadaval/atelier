#!/usr/bin/env python3
"""Print a matrix's cells from results.jsonl: passes, median seconds and tokens.

Usage: table.py <results.jsonl or a directory holding one> ...

A cell is a setup, a case and an arm. Passes count the graded runs; a run
graded null, an invalid one included, is counted apart and left out of the
medians. Tokens are a run's tokens in and out summed over its threads, and
"-" where a run predates their record. A result from before setups is put
in the setup named by its agent and model. A run graded null that a later
result in the file made again (the same setup, case, arm and index), as a
matrix run again into the same output does, is left out.
"""
import json
import statistics
import sys
from pathlib import Path


def setup_of(r: dict) -> str:
    return r.get("setup") or f"{r.get('agent', 'claude')}:{r.get('model', '?')}"


def amount(n) -> str:
    if n is None:
        return "-"
    for size, unit in ((1e6, "M"), (1e3, "k")):
        if n >= size:
            return f"{n / size:.1f}{unit}"
    return str(round(n))


def median(values: list):
    return statistics.median(values) if values else None


def current(results: list) -> list:
    """The results less each run graded null that a later result made again."""
    later, out = set(), []
    for r in reversed(results):
        run = (setup_of(r), r.get("scenario"), r.get("arm"), r.get("index"))
        if r.get("pass") is None and run in later:
            continue
        later.add(run)
        out.append(r)
    return out[::-1]


def rows(results: list) -> list:
    """One row per cell, sorted by setup, case and arm."""
    cells = {}
    for r in current(results):
        cells.setdefault((setup_of(r), r.get("scenario"), r.get("arm")), []).append(r)
    out = []
    for (setup, case, arm), runs in sorted(cells.items(), key=lambda c: tuple(map(str, c[0]))):
        graded = [r for r in runs if r.get("pass") is not None]
        counted = [r for r in graded if r.get("tokens")]
        out.append({
            "setup": setup, "case": case, "arm": arm,
            "passes": sum(r["pass"] is True for r in graded), "graded": len(graded),
            "null": len(runs) - len(graded),
            "seconds": median([r["seconds"] for r in graded if r.get("seconds") is not None]),
            "tokens_in": median([r["tokens"]["in"] for r in counted]),
            "tokens_out": median([r["tokens"]["out"] for r in counted]),
        })
    return out


def table(results: list) -> str:
    """The cells as a Markdown table."""
    lines = ["| Setup | Case | Arm | Passes | Null | Median s | Median tokens in | Median tokens out |",
             "| --- | --- | --- | --- | --- | --- | --- | --- |"]
    for c in rows(results):
        seconds = "-" if c["seconds"] is None else str(round(c["seconds"]))
        lines.append(f"| {c['setup']} | {c['case']} | {c['arm']} | "
                     f"{c['passes']} of {c['graded']} | {c['null']} | {seconds} | "
                     f"{amount(c['tokens_in'])} | {amount(c['tokens_out'])} |")
    return "\n".join(lines)


def read(paths: list) -> list:
    results = []
    for path in map(Path, paths):
        path = path / "results.jsonl" if path.is_dir() else path
        results += [json.loads(line) for line in path.read_text().splitlines() if line.strip()]
    return results


def main() -> None:
    if len(sys.argv) < 2 or sys.argv[1] in ("-h", "--help"):
        sys.exit(__doc__.strip())
    print(table(read(sys.argv[1:])))


if __name__ == "__main__":
    main()
