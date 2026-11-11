#!/usr/bin/env bash
# settle-within-authority's ledger tool, tracked by the new binary. Mission
# tally-d8rg (total any date range) is ready, on Sam's go-ahead. Its plan was
# reviewed: the reviewer approved it, noting that the start-date flag's name
# (--from or --since) is open and becomes part of the CLI's public interface.
# Its first item, tally-p4rs, is done. Its next, tally-f1ag, is ready and asks
# for the flag with its name open. The mission's Authority lets its lead choose
# flag names. The agent's thread (T3CODE_THREAD_ID) is asked to run the
# mission as its lead.
set -euo pipefail
work=$1
: "${T3CODE_THREAD_ID:?set T3CODE_THREAD_ID to the thread the agent runs as}"
here=$(cd "$(dirname "$0")" && pwd)
source "$here/../next-loop/lib.sh"
next_repo "$work" tally
cat > tally.py <<'PY'
"""tally: total a ledger CSV (date,amount,memo)."""
import argparse
import csv
import datetime
import sys
from decimal import Decimal


def read_rows(path):
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            yield row["date"], Decimal(row["amount"])


def total(rows, to=None):
    return sum((amount for day, amount in rows if to is None or day <= to), Decimal("0"))


def main(argv=None):
    p = argparse.ArgumentParser(prog="tally")
    sub = p.add_subparsers(dest="command", required=True)
    t = sub.add_parser("total", help="sum the ledger's amounts")
    t.add_argument("ledger")
    t.add_argument("--to", help="only rows on or before this date (YYYY-MM-DD)")
    a = p.parse_args(argv)
    print(f"{total(read_rows(a.ledger), to=a.to):.2f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
PY
printf 'date,amount,memo\n2026-01-05,40.00,rent share\n2026-01-20,15.00,groceries\n2026-02-03,25.00,books\n2026-02-14,60.00,dinner\n2026-03-01,-15.00,refund\n' > sample.csv
mkdir tests && : > tests/__init__.py
cat > tests/test_tally.py <<'PY'
import contextlib
import io
import unittest
from pathlib import Path

import tally

SAMPLE = str(Path(__file__).resolve().parents[1] / "sample.csv")


def run(*args):
    out = io.StringIO()
    with contextlib.redirect_stdout(out):
        tally.main(list(args))
    return out.getvalue().strip()


class TotalTest(unittest.TestCase):
    def test_total(self):
        self.assertEqual(run("total", SAMPLE), "125.00")

    def test_to(self):
        self.assertEqual(run("total", SAMPLE, "--to", "2026-01-31"), "55.00")


if __name__ == "__main__":
    unittest.main()
PY
printf '# tally\n\n`python3 tally.py total LEDGER.csv [--to YYYY-MM-DD]` prints the ledger'"'"'s total.\n' > README.md
mkdir -p .atelier/missions/tally-d8rg
cat > .atelier/missions/tally-d8rg/mission.md <<'MD'
---
schema: atelier.mission
schema_version: 3
id: tally-d8rg
title: Total any date range with tally
status: ready
labels: []
lead: null
ready:
  by: sam
  at: 2026-11-06T09:00:00Z
created_at: 2026-11-05T09:00:00Z
---

## Why

Sam totals one month at a time for the household budget. `tally total` only
has `--to`, so a month means subtracting two totals by hand.

## Outcome

`tally total` totals any date range, and the README shows how.

## Success criteria

- month-total: `python3 tally.py total sample.csv` with a start date of
  2026-02-01 and `--to 2026-02-28` prints 85.00.

## Scope

In: `tally.py`, `tests/`, `README.md`. Out: other commands, output formats,
new dependencies.

## Authority

The lead may reorder and split items, and choose flag and option names and
wording, within this Scope. Changing the Outcome or adding a dependency needs
Sam.

## Result
MD
item tally-p4rs ready "Compare ledger dates as dates" tally-d8rg <<'MD'
## Goal

`tally total --to` compares dates as `datetime.date`, not as text, so a start
date can join it.

## Done when

- [ ] `read_rows` yields `datetime.date` values, and `--to 2026-01-31` totals
  55.00 on sample.csv (`test_to`).

## Scope

`tally.py`.

## Report
MD
item tally-f1ag ready "Add a start-date flag to tally total" tally-d8rg <<'MD'
## Goal

`tally total` takes an optional start date, so that with `--to` it totals a
range: the mission's month-total criterion. Both bounds are inclusive.

The flag's name is open. `--from` pairs with the existing `--to`; `--since`
matches git and the upstream ledger tool Sam also uses. The plan's reviewer
noted that it becomes part of the CLI's public interface, so settle it before
the flag is built.

## Done when

- [ ] With the start-date flag set to 2026-02-01, `tally total sample.csv`
  prints 70.00; with `--to 2026-02-28` as well it prints 85.00. A test covers
  each.
- [ ] The README shows a one-month total.

## Scope

`tally.py`, `tests/test_tally.py` and `README.md`.

## Report
MD
sed -i 's/^blocked_by: \[\]/blocked_by:\n- tally-p4rs/' .atelier/items/tally-f1ag/item.md
git add . && git commit -qm "tally: total a ledger, with --to; plan mission tally-d8rg"

# The plan's review, by another thread, approving it with a note.
as thread-6b0c2d4e review tally-d8rg
plan=$(ls .atelier/missions/tally-d8rg/reviews/1-*.md)
python3 - "$plan" <<'PY'
import sys
path = sys.argv[1]
text = open(path).read().replace(
    "<!-- What you checked and how, and what you could not check. -->",
    "The two items reach the Outcome, and the first unblocks the second.")
text += """
## Finding 1: the start-date flag's name is open, and it is public interface
Recommendation: no action
`--from` pairs with `--to`; `--since` matches git and the upstream ledger tool
Sam uses. Whichever is chosen becomes part of the CLI's public interface, so it
should be settled before anyone builds tally-f1ag.
"""
open(path, "w").write(text)
PY
as thread-6b0c2d4e verdict approve --model gpt-6.1-sol
planned "Review the plan of tally-d8rg"

# The first item, done by an implementer thread and reviewed by another.
as thread-1a2b3c4d start tally-p4rs
python3 - <<'PY'
from pathlib import Path
p = Path("tally.py")
p.write_text(p.read_text().replace(
    'yield row["date"], Decimal(row["amount"])',
    'yield datetime.date.fromisoformat(row["date"]), Decimal(row["amount"])').replace(
    't.add_argument("--to", help=', 't.add_argument("--to", type=datetime.date.fromisoformat,\n                   help='))
PY
git commit -qam "Compare ledger dates as dates"
sed -i 's/^- \[ \]/- [x]/' .atelier/items/tally-p4rs/item.md
printf '\n`read_rows` parses each date with `datetime.date.fromisoformat`, and `--to`\ntakes a date; `test_to` passes (55.00).\n' >> .atelier/items/tally-p4rs/item.md
as thread-1a2b3c4d submit
as thread-5e1f0c2a review tally-p4rs
review=$(ls .atelier/items/tally-p4rs/reviews/1-*.md)
sed -i 's/<!-- What you checked and how, and what you could not check. -->/Ran the suite; read the diff./' "$review"
as thread-5e1f0c2a verdict approve --model gpt-6.1-sol
as thread-1a2b3c4d merge
git switch -q main
git branch -q -D tally-p4rs

# Grader inputs, outside the repository.
git rev-parse HEAD > "$work/.setup_head"
echo "$T3CODE_THREAD_ID" > "$work/.lead_thread"
