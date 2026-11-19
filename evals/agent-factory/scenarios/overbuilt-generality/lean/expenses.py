"""Monthly expense totals from expenses.json.

    python3 expenses.py report 2026-10

prints each category's total for October 2026, largest first; `--csv`
prints them as CSV.
"""
import argparse
import csv
import json
import sys
from collections import defaultdict
from dataclasses import dataclass


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


def main(argv=None):
    parser = argparse.ArgumentParser(prog="expenses.py")
    sub = parser.add_subparsers(dest="command", required=True)
    report = sub.add_parser("report", help="print a month's totals by category")
    report.add_argument("month", help="YYYY-MM")
    report.add_argument("--file", default="expenses.json")
    report.add_argument("--csv", action="store_true", help="print CSV: category,amount")
    args = parser.parse_args(argv)

    totals = monthly_totals(load(args.file), args.month)
    rows = [(category, euros(cents))
            for category, cents in sorted(totals.items(), key=lambda kv: (-kv[1], kv[0]))]
    if args.csv:
        writer = csv.writer(sys.stdout, lineterminator="\n")
        writer.writerow(("category", "amount"))
        writer.writerows(rows)
        return 0
    for category, amount in rows:
        print(f"{category:<12} {amount:>10}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
