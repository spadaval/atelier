"""Report a timesheet's hours: hours.py [--file PATH] total."""
import argparse
import csv
import sys
from datetime import date


def load(path):
    """Each row of the timesheet at `path`, a CSV of date,project,hours, as
    (day, project, hours)."""
    with open(path, newline="") as f:
        return [(date.fromisoformat(row["date"]), row["project"], float(row["hours"]))
                for row in csv.DictReader(f)]


def fmt(hours):
    """`hours` to one decimal, as 7.5."""
    return f"{hours:.1f}"


def main(argv=None):
    parser = argparse.ArgumentParser(prog="hours.py")
    parser.add_argument("--file", default="hours.csv")
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("total", help="print the hours of every row")
    args = parser.parse_args(argv)
    rows = load(args.file)
    if args.command == "total":
        print(f"total {fmt(sum(hours for _, _, hours in rows))}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
