"""Report a timesheet's hours: hours.py [--file PATH] total | week DATE."""
import argparse
import csv
import sys
from datetime import date, timedelta


def load(path):
    """Each row of the timesheet at `path`, a CSV of date,project,hours, as
    (day, project, hours)."""
    with open(path, newline="") as f:
        return [(date.fromisoformat(row["date"]), row["project"], float(row["hours"]))
                for row in csv.DictReader(f)]


def fmt(hours):
    """`hours` to one decimal, as 7.5."""
    return f"{hours:.1f}"


def week_of(day):
    """The Monday and the Sunday of the week that holds `day`."""
    monday = day - timedelta(days=day.weekday())
    return monday, monday + timedelta(days=7)


# week_lines: each case's own, from its fixture/week_lines.py


def main(argv=None):
    parser = argparse.ArgumentParser(prog="hours.py")
    parser.add_argument("--file", default="hours.csv")
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("total", help="print the hours of every row")
    week = commands.add_parser("week", help="print each project's hours in the week of DATE")
    week.add_argument("date", metavar="DATE", help="a day of the week, as YYYY-MM-DD")
    args = parser.parse_args(argv)
    rows = load(args.file)
    if args.command == "total":
        print(f"total {fmt(sum(hours for _, _, hours in rows))}")
    elif args.command == "week":
        for line in week_lines(rows, date.fromisoformat(args.date)):
            print(line)
    return 0


if __name__ == "__main__":
    sys.exit(main())
