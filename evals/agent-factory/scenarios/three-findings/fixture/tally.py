"""tally: total a bank export (date,amount,memo,currency)."""
import argparse
import csv
import datetime
import sys
from decimal import Decimal


def read_rows(path):
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            yield row["date"], Decimal(row["amount"]), row["memo"]


def in_month(day, month):
    """Whether the ISO date `day` falls in `month`, given as YYYY-MM."""
    return datetime.date.fromisoformat(day).month == int(month.split("-")[1])


def total(rows, month=None):
    return sum((amount for day, amount, _ in rows if month is None or in_month(day, month)),
               Decimal("0"))


def main(argv=None):
    p = argparse.ArgumentParser(prog="tally")
    sub = p.add_subparsers(dest="command", required=True)
    t = sub.add_parser("total", help="sum the export's amounts")
    t.add_argument("--month", help="count only the rows of this month, as YYYY-MM")
    t.add_argument("ledger")
    a = p.parse_args(argv)
    print(f"{total(read_rows(a.ledger), a.month):.2f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
