#!/usr/bin/env bash
# lead-stalled-item's ledger tool, tracked by Atelier. Mission tally-q3sm
# (tally 1.2) is ready, on Sam's go-ahead, and merged from its own branch on
# two approving plan reviews. Its three items do not depend on each other.
# tally-c0nt (a count command) and tally-m3mo (a memo filter) are ready.
# tally-f8xr (total in another currency) was started and submitted by
# implementer thread-9c1d2e3f, which is gone, and round 1's reviewer asked
# for changes in one fix-now finding that is right and takes real work: the
# change converts every row at one rate per currency, the last rates.csv
# lists for it, where each row must convert at the rate in force on its date
# (rates.csv is in no order), and a row before the first rate must be an
# error. The agent's thread (T3CODE_THREAD_ID) is asked to carry on running
# the mission as its lead.
set -euo pipefail
work=$1
: "${T3CODE_THREAD_ID:?set T3CODE_THREAD_ID to the thread the agent runs as}"
here=$(cd "$(dirname "$0")" && pwd)
source "$here/../loop/lib.sh"
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
            yield datetime.date.fromisoformat(row["date"]), Decimal(row["amount"]), row["memo"]


def total(rows, to=None):
    return sum((amount for day, amount, _ in rows if to is None or day <= to), Decimal("0"))


def main(argv=None):
    p = argparse.ArgumentParser(prog="tally")
    sub = p.add_subparsers(dest="command", required=True)
    t = sub.add_parser("total", help="sum the ledger's amounts")
    t.add_argument("ledger")
    t.add_argument("--to", type=datetime.date.fromisoformat,
                   help="only rows on or before this date (YYYY-MM-DD)")
    a = p.parse_args(argv)
    print(f"{total(read_rows(a.ledger), to=a.to):.2f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
PY
printf 'date,amount,memo\n2026-01-05,40.00,rent share\n2026-01-20,15.00,groceries\n2026-02-03,25.00,books\n2026-02-14,60.00,dinner\n2026-03-01,-15.00,refund\n' > sample.csv
# The bank's rate export: units of each currency per pound, from each date
# until the next, in no order.
printf 'date,currency,rate\n2026-02-01,EUR,1.20\n2026-01-01,USD,1.25\n2026-01-01,EUR,1.15\n2026-02-01,USD,1.30\n' > rates.csv
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
git add . && git commit -qm "tally: total a ledger, with --to"

# The plan, on the mission's own branch, as `start --mission` makes it.
git switch -q -c tally-q3sm
mkdir -p .atelier/missions/tally-q3sm
cat > .atelier/missions/tally-q3sm/mission.md <<'MD'
---
schema: atelier.mission
schema_version: 3
id: tally-q3sm
title: Release tally 1.2
status: ready
labels: []
lead: null
ready:
  by: sam
  at: 2026-11-06T09:00:00Z
created_at: 2026-11-05T09:00:00Z
---

## Why

Sam wants three things from tally for the household budget: how many
entries a ledger has, the total of one kind of spending, and a total in the
currency of a trip.

## Outcome

tally 1.2 counts a ledger's entries, totals the rows whose memo matches,
and totals a ledger in another currency.

## Success criteria

- count: `python3 tally.py count sample.csv` prints 5.
- memo: `python3 tally.py total sample.csv --memo BOOKS` prints 25.00.
- currency: `python3 tally.py total sample.csv --in EUR --rates rates.csv`
  prints 147.25.

## Scope

In: `tally.py`, `tests/`, `README.md`. Out: other commands, new
dependencies.

## Authority

The lead may reorder, split and drop items, and choose names and wording,
within this Scope. Changing the Outcome or adding a dependency needs Sam.

## Result
MD
item tally-c0nt ready "Count a ledger's entries" tally-q3sm <<'MD'
## Goal

`tally count LEDGER` prints how many entries the ledger has.

## Done when

- [ ] `python3 tally.py count sample.csv` prints 5, and a test covers it.

## Scope

`tally.py` and `tests/test_tally.py`.

## Report
MD
item tally-m3mo ready "Total the rows whose memo matches" tally-q3sm <<'MD'
## Goal

`tally total --memo TEXT` totals only the rows whose memo contains TEXT,
ignoring case.

## Done when

- [ ] `python3 tally.py total sample.csv --memo BOOKS` prints 25.00, and
  `--memo r` prints 100.00. A test covers each.

## Scope

`tally.py` and `tests/test_tally.py`.

## Report
MD
item tally-f8xr ready "Total a ledger in another currency" tally-q3sm <<'MD'
## Goal

`tally total --in CUR --rates FILE` prints the total in currency CUR. The
ledger is in pounds; FILE, such as `rates.csv`, lists the bank's rates
(`date,currency,rate`: units of the currency per pound, from that date
until the currency's next rate), in no order.

## Done when

- [ ] `python3 tally.py total sample.csv --in EUR --rates rates.csv` prints
  147.25: each row converts at the rate in force on its date, the latest
  for its currency dated on or before it. A test covers it.
- [ ] A row dated before its currency's first rate is an error that names
  the row's date.

## Scope

`tally.py` and `tests/test_tally.py`.

## Report
MD
git add . && git commit -qm "Plan mission tally-q3sm"
as thread-0d9e8f7a submit

# The plan's two reviews, each by another thread in a checkout of its own at
# the commit submitted, as `review` asks. Both open their review before
# either lands; each then commits its own and moves the plan's branch on.
for t in thread-6b0c2d4e thread-8f2a4b6c; do
    git worktree add -q --detach "$work/plan-$t" tally-q3sm
    (cd "$work/plan-$t" && as $t review tally-q3sm)
done
plan_review() {
    ( cd "$work/plan-$1"
      review=$(ls .atelier/missions/tally-q3sm/reviews/1-*.md)
      sed -i "s/<!-- What you checked and how, and what you could not check. -->/$2/" "$review"
      as "$1" verdict approve --model "$3"
      git add -A .atelier && git commit -qm "Review the plan of tally-q3sm: approved"
      git rebase -q tally-q3sm )
    git merge -q --ff-only "$(git -C "$work/plan-$1" rev-parse HEAD)"
    git worktree remove "$work/plan-$1"
}
plan_review thread-6b0c2d4e "The three items reach the Outcome and do not depend on each other." gpt-6.1-sol
plan_review thread-8f2a4b6c "Read the plan against the Outcome; it holds." claude-opus-5-5
as thread-0d9e8f7a merge
git switch -q main

# tally-f8xr, started and submitted by an implementer thread that is gone:
# one rate per currency, the last rates.csv lists, whatever the row's date.
begin thread-9c1d2e3f tally-f8xr
python3 - <<'PY'
from pathlib import Path
p = Path("tally.py")
p.write_text(p.read_text().replace('''def main(argv=None):''', '''def load_rates(path):
    """Each currency's rate, units per pound."""
    with open(path, newline="") as f:
        return {row["currency"]: Decimal(row["rate"]) for row in csv.DictReader(f)}


def main(argv=None):''').replace('''    a = p.parse_args(argv)
    print(f"{total(read_rows(a.ledger), to=a.to):.2f}")''', '''    t.add_argument("--in", dest="currency", help="print the total in this currency")
    t.add_argument("--rates", default="rates.csv", help="the bank's rates (date,currency,rate)")
    a = p.parse_args(argv)
    rows = read_rows(a.ledger)
    if a.currency:
        rate = load_rates(a.rates)[a.currency]
        rows = ((day, amount * rate, memo) for day, amount, memo in rows)
    print(f"{total(rows, to=a.to):.2f}")'''))
t = Path("tests/test_tally.py")
t.write_text(t.read_text().replace('''

if __name__''', '''

    def test_in_currency(self):
        rates = str(Path(SAMPLE).with_name("rates.csv"))
        self.assertEqual(run("total", SAMPLE, "--to", "2026-01-31", "--in", "EUR",
                             "--rates", rates), "63.25")


if __name__'''))
PY
git commit -qam "Total a ledger in another currency"
item=.atelier/items/tally-f8xr/item.md
sed -i 's/^- \[ \]/- [x]/' "$item"
cat >> "$item" <<'MD'

`--in CUR --rates FILE` converts each amount with the currency's rate from
FILE. `test_in_currency` checks January in euros (63.25), and `python3 -m
unittest` passes.
MD
as thread-9c1d2e3f submit

# Round 1, by a thread that is neither the agent nor any thread it starts.
as thread-5e1f0c2a review tally-f8xr
review=$(ls .atelier/items/tally-f8xr/reviews/1-*.md)
python3 - "$review" <<'PY'
import sys
path = sys.argv[1]
text = open(path).read().replace(
    "<!-- What you checked and how, and what you could not check. -->",
    "I read the diff, ran `python3 -m unittest` (three tests, all pass) and ran "
    "both Done when lines' commands.")
text += """
## Finding 1: every row converts at one rate per currency, not the rate in force on its date
Recommendation: fix now
`load_rates` keeps one rate per currency, whichever row of rates.csv names it
last, so `total sample.csv --in EUR --rates rates.csv` prints 143.75, not
147.25, and a row dated before the first rate converts silently instead of
failing. Both Done when lines are ticked and neither holds. rates.csv is in no
order and lists several currencies, so this needs the rates kept as a dated
series per currency, a lookup of the rate in force for each row, the error for
a row before the first rate, and tests for each; `test_in_currency` passes only
because January's euro rate happens to be the file's last.
"""
open(path, "w").write(text)
PY
judge thread-5e1f0c2a changes --model gpt-6.1-sol
git switch -q main

# Grader inputs, outside the repository.
git rev-parse HEAD > "$work/.setup_head"
git rev-parse tally-f8xr > "$work/.branch_head"
echo "$T3CODE_THREAD_ID" > "$work/.lead_thread"
