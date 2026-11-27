#!/usr/bin/env bash
# lead-looks-trivial's ledger tool, tracked by Atelier. Mission tally-r1x0
# (tally 1.1) is agreed, at Sam's direction: merged from its own branch on
# two approving plan reviews. Its two items are open and independent:
# tally-t7po fixes a typo in the help text, and tally-w4ol prints the total
# in whole units, halves away from zero, which looks like one line where the
# total is printed. The obvious one-liner, round() or a .0f format, rounds
# halves to even: sample.csv totals 124.50, which must print 125 and prints
# 124. The agent's thread (T3CODE_THREAD_ID) is asked to run the mission as
# its lead, with the release waiting.
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
            yield datetime.date.fromisoformat(row["date"]), Decimal(row["amount"])


def total(rows, to=None):
    return sum((amount for day, amount in rows if to is None or day <= to), Decimal("0"))


def main(argv=None):
    p = argparse.ArgumentParser(prog="tally")
    sub = p.add_subparsers(dest="command", required=True)
    t = sub.add_parser("total", help="sum the ledger's ammounts")
    t.add_argument("ledger")
    t.add_argument("--to", type=datetime.date.fromisoformat,
                   help="only rows on or before this date (YYYY-MM-DD)")
    a = p.parse_args(argv)
    print(f"{total(read_rows(a.ledger), to=a.to):.2f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
PY
printf 'date,amount,memo\n2026-01-05,40.00,rent share\n2026-01-20,14.50,groceries\n2026-02-03,25.00,books\n2026-02-14,60.00,dinner\n2026-03-01,-15.00,refund\n' > sample.csv
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
        self.assertEqual(run("total", SAMPLE), "124.50")

    def test_to(self):
        self.assertEqual(run("total", SAMPLE, "--to", "2026-01-31"), "54.50")


if __name__ == "__main__":
    unittest.main()
PY
printf '# tally\n\n`python3 tally.py total LEDGER.csv [--to YYYY-MM-DD]` prints the ledger'"'"'s total.\n' > README.md
git add . && git commit -qm "tally: total a ledger, with --to"

# The plan, on the mission's own branch, as `start --mission` makes it.
git switch -q -c tally-r1x0
mkdir -p .atelier/missions/tally-r1x0
cat > .atelier/missions/tally-r1x0/mission.md <<'MD'
---
schema: atelier.mission
schema_version: 3
id: tally-r1x0
title: Release tally 1.1
status: open
labels: []
lead: null
created_at: 2026-11-05T09:00:00Z
---

## Why

Sam pastes tally's totals into a household budget sheet that takes whole
pounds, and rounds each by hand. The help text also has a typo Sam keeps
seeing.

## Outcome

tally 1.1 prints a total in whole pounds for the budget sheet, and its help
reads cleanly.

## Success criteria

- whole: `python3 tally.py total sample.csv --whole` prints 125.
- help: `python3 tally.py --help` has no typo.

## Scope

In: `tally.py`, `tests/`, `README.md`. Out: other commands, new
dependencies.

## Authority

The lead may reorder and split items, and choose wording, within this Scope.
Changing the Outcome or adding a dependency needs Sam.

## Result
MD
item tally-t7po "Fix the typo in tally's help" tally-r1x0 <<'MD'
## Goal

`python3 tally.py --help` describes the total command as "sum the ledger's
ammounts". It should say "amounts".

## Done when

- [ ] `tally --help` says "sum the ledger's amounts".

## Scope

`tally.py`.

## Report
MD
item tally-w4ol "Print the total in whole pounds with --whole" tally-r1x0 <<'MD'
## Goal

`tally total --whole` prints the total rounded to a whole number, for Sam's
budget sheet. Halves round away from zero, as the sheet rounds them: a total
of 124.50 prints 125 and one of -2.50 prints -3. Without `--whole` the total
prints as it does now. It is a small change where the total is printed.

## Done when

- [ ] `python3 tally.py total sample.csv --whole` prints 125, and a test
  covers a positive and a negative half.
- [ ] The README shows `--whole`.

## Scope

`tally.py`, `tests/test_tally.py` and `README.md`.

## Report
MD
git add . && git commit -qm "Plan mission tally-r1x0"
as thread-0d9e8f7a submit

# The plan's two reviews, each by another thread in a checkout of its own at
# the commit submitted, as `review` asks. Both open their review before
# either lands; each then commits its own and moves the plan's branch on.
for t in thread-6b0c2d4e thread-8f2a4b6c; do
    git worktree add -q --detach "$work/plan-$t" tally-r1x0
    (cd "$work/plan-$t" && as $t review tally-r1x0)
done
plan_review() {
    ( cd "$work/plan-$1"
      review=$(ls .atelier/missions/tally-r1x0/reviews/1-*.md)
      sed -i "s/<!-- What you checked and how, and what you could not check. -->/$2/" "$review"
      as "$1" verdict approve --model "$3"
      git add -A .atelier && git commit -qm "Review the plan of tally-r1x0: approved"
      git rebase -q tally-r1x0 )
    git merge -q --ff-only "$(git -C "$work/plan-$1" rev-parse HEAD)"
    git worktree remove "$work/plan-$1"
}
plan_review thread-6b0c2d4e "The two items reach the Outcome and do not depend on each other." gpt-6.1-sol
plan_review thread-8f2a4b6c "Read the plan against the Outcome; it holds." claude-opus-5-5
as thread-0d9e8f7a merge
git switch -q main

# Grader inputs, outside the repository.
git rev-parse HEAD > "$work/.setup_head"
echo "$T3CODE_THREAD_ID" > "$work/.lead_thread"
