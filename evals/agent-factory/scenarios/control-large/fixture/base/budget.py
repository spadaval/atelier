"""A household ledger kept in ledger.json.

    python3 budget.py summary

prints how many transactions the ledger holds, and the money in and out.
"""
import argparse
import json
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Transaction:
    date: str  # YYYY-MM-DD
    cents: int  # negative for money out
    payee: str


def load_ledger(path):
    path = Path(path)
    if not path.exists():
        return []
    return [Transaction(**t) for t in json.loads(path.read_text())]


def save_ledger(path, transactions):
    rows = [{"date": t.date, "cents": t.cents, "payee": t.payee} for t in transactions]
    Path(path).write_text(json.dumps(rows, indent=2) + "\n")


def euros(cents):
    sign = "-" if cents < 0 else ""
    return f"{sign}{abs(cents) // 100}.{abs(cents) % 100:02d}"


def main(argv=None):
    parser = argparse.ArgumentParser(prog="budget.py")
    parser.add_argument("--ledger", default="ledger.json")
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("summary", help="count the transactions, and the money in and out")
    args = parser.parse_args(argv)

    ledger = load_ledger(args.ledger)
    money_in = sum(t.cents for t in ledger if t.cents > 0)
    money_out = sum(t.cents for t in ledger if t.cents < 0)
    print(f"{len(ledger)} transactions, in {euros(money_in)}, out {euros(money_out)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
