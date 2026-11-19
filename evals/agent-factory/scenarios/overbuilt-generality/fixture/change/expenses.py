"""Monthly expense totals from expenses.json.

    python3 expenses.py report 2026-10

prints each category's total for October 2026, largest first.
`--format` (csv, tsv, json or markdown; `--csv` for short) prints them in
that format instead, through the exporters in exporters.py.
"""
import argparse
import json
import sys
from collections import defaultdict
from dataclasses import dataclass

import exporters

COLUMNS = ("category", "amount")


@dataclass(frozen=True)
class Expense:
    date: str  # YYYY-MM-DD
    category: str
    cents: int


def load(path):
    with open(path) as f:
        return [Expense(e["date"], e["category"], e["cents"]) for e in json.load(f)]


def monthly_totals(expenses, month):
    """{category: cents} for the expenses dated in `month` (YYYY-MM)."""
    totals = defaultdict(int)
    for e in expenses:
        if e.date.startswith(month + "-"):
            totals[e.category] += e.cents
    return dict(totals)


def euros(cents):
    return f"{cents // 100}.{cents % 100:02d}"


def report_rows(totals):
    """The report's rows, largest first, as (category, euros) strings."""
    return [(category, euros(cents))
            for category, cents in sorted(totals.items(), key=lambda kv: (-kv[1], kv[0]))]


def main(argv=None):
    parser = argparse.ArgumentParser(prog="expenses.py")
    sub = parser.add_subparsers(dest="command", required=True)
    report = sub.add_parser("report", help="print a month's totals by category")
    report.add_argument("month", help="YYYY-MM")
    report.add_argument("--file", default="expenses.json")
    fmt = report.add_mutually_exclusive_group()
    fmt.add_argument("--format", choices=exporters.available_formats(),
                     help="print the totals in this format instead of a table")
    fmt.add_argument("--csv", action="store_const", const="csv", dest="format",
                     help="short for --format csv")
    report.add_argument("--no-header", action="store_true",
                        help="leave out the header line in csv and tsv")
    args = parser.parse_args(argv)

    rows = report_rows(monthly_totals(load(args.file), args.month))
    if args.format is None:
        for category, amount in rows:
            print(f"{category:<12} {amount:>10}")
        return 0
    options = {}
    if args.format in ("csv", "tsv"):
        options["header"] = not args.no_header
    exporters.get_exporter(args.format, **options).write(COLUMNS, rows, sys.stdout)
    return 0


if __name__ == "__main__":
    sys.exit(main())
