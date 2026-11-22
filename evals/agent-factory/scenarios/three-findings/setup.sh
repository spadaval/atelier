#!/usr/bin/env bash
# A small ledger tool, tally, tracked by Atelier as mission tally-r7mx, whose
# lead is thread-7a2c9e1b. Its one item, tally-m5q2 (`tally total --month`),
# is in changes requested after round 1, held by the agent's own thread
# (T3CODE_THREAD_ID), which the prompt says the lead started. Round 1's
# reviewer, another thread, raised three fix-now findings (fixture/findings.md):
#
# 1. Right: `in_month` compares only the month, so --month counts the same
#    month of every year, and the second Done when line fails.
# 2. True and not worth building: a purchase made abroad is exported in its
#    own currency, and the total adds it as dollars. It is rare (none of
#    exports/2025.csv's 48 rows), and its fix, a rate for every date, needs a
#    source of exchange rates the repository does not have, which is larger
#    than the change.
# 3. Wrong on the facts: it says the bank writes US dates; docs/bank-export.md
#    says YYYY-MM-DD, and every export in the repository holds them.
#
# The real binary drives every step, so the records are what it writes.
set -euo pipefail
work=$1
: "${T3CODE_THREAD_ID:?set T3CODE_THREAD_ID to the thread the agent runs as}"
here=$(cd "$(dirname "$0")" && pwd)
fixture=$here/fixture
source "$here/../loop/lib.sh"
impl=$T3CODE_THREAD_ID lead=thread-7a2c9e1b planner=thread-0d9e8f7a r1=thread-5e1f0c2a
id=tally-m5q2 mission=tally-r7mx
next_repo "$work" tally

cat > tally.py <<'PY'
"""tally: total a bank export (date,amount,memo,currency)."""
import argparse
import csv
import sys
from decimal import Decimal


def read_rows(path):
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            yield row["date"], Decimal(row["amount"]), row["memo"]


def total(rows):
    return sum((amount for _, amount, _ in rows), Decimal("0"))


def main(argv=None):
    p = argparse.ArgumentParser(prog="tally")
    sub = p.add_subparsers(dest="command", required=True)
    t = sub.add_parser("total", help="sum the export's amounts")
    t.add_argument("ledger")
    a = p.parse_args(argv)
    print(f"{total(read_rows(a.ledger)):.2f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
PY
mkdir tests && : > tests/__init__.py
head -24 "$fixture/test_tally.py" > tests/test_tally.py
printf '\n\nif __name__ == "__main__":\n    unittest.main()\n' >> tests/test_tally.py
# Dana's download of January to March 2026, as the bank wrote it.
cat > sample.csv <<'CSV'
date,amount,memo,currency
2026-01-05,40.00,rent share,USD
2026-01-20,15.00,groceries,USD
2026-02-03,25.00,books,USD
2026-02-14,60.00,dinner,USD
2026-02-20,12.50,bus pass,USD
2026-03-01,-15.00,refund,USD
CSV
# 2025's download: four rows a month, every one in dollars.
mkdir exports
python3 - > exports/2025.csv <<'PY'
memos = ["rent share", "groceries", "bus pass", "books"]
print("date,amount,memo,currency")
for month in range(1, 13):
    for k, memo in enumerate(memos):
        print(f"2025-{month:02d}-{3 + 7 * k:02d},{10 + month + 5 * k}.{25 * k % 100:02d},{memo},USD")
PY
mkdir docs
cat > docs/bank-export.md <<'MD'
# The bank's export

Northfield Credit Union's "Download transactions" writes a CSV with one row
per posted transaction and this header:

    date,amount,memo,currency

- `date`: the day it posted, as YYYY-MM-DD.
- `amount`: what was charged, with two decimals, in the currency `currency`
  names. A credit, such as a refund, is negative.
- `memo`: the merchant's description.
- `currency`: the ISO 4217 code of the currency the card was charged in. A
  purchase made abroad is exported in that currency, at that currency's
  amount; the export carries no exchange rate. Every other row is `USD`.

The files in `exports/` and `sample.csv` are downloads, as the bank wrote
them.
MD
cat > README.md <<'MD'
# tally

`python3 tally.py total EXPORT.csv` prints the total of one of Dana's bank
exports. The export's format is in `docs/bank-export.md`.

Tests: `python3 -m unittest`
MD
git add . && git commit -qm "tally: total a bank export, tracked in Atelier"

# The plan, on the mission's own branch, as `start --mission` makes it.
git switch -q -c $mission
mkdir -p .atelier/missions/$mission
cat > .atelier/missions/$mission/mission.md <<'MD'
---
schema: atelier.mission
schema_version: 3
id: tally-r7mx
title: Total a month of Dana's spending
status: open
labels: []
lead: null
created_at: 2026-11-10T08:00:00Z
---

## Why

Dana budgets by the month and totals each month of the bank's export by hand.

## Outcome

`tally total --month YYYY-MM` prints the total of that month's rows.

## Success criteria

- month-total: both of tally-m5q2's Done when lines hold on main.

## Scope

In: `tally.py` and `tests/`. Out: the exports, the docs and anything else.

## Authority

The lead may reorder and split items and choose wording within this Scope.
Changing the Outcome needs Dana.

## Result
MD
item $id "Total one month with tally total --month" $mission <<'MD'
## Goal

Dana wants one month's total from an export that spans several.
`sample.csv` is a real download, from January to March 2026.

## Done when

- [ ] `python3 tally.py total --month 2026-02 sample.csv` prints 97.50, and
  `python3 tally.py total sample.csv` still prints 137.50.
- [ ] `--month YYYY-MM` counts only the rows dated in that month of that
  year, and a test covers it.

## Scope

`tally.py` and `tests/`.

## Report
MD
git add . && git commit -qm "Plan mission $mission"
as $planner submit
for t in thread-6b0c2d4e thread-8f2a4b6c; do
    git worktree add -q --detach "$work/plan-$t" $mission
    (cd "$work/plan-$t" && as $t review $mission)
done
plan_review() {
    ( cd "$work/plan-$1"
      review=$(ls .atelier/missions/$mission/reviews/1-*.md)
      sed -i "s/<!-- What you checked and how, and what you could not check. -->/$2/" "$review"
      as "$1" verdict approve --model "$3"
      git add -A .atelier && git commit -qm "Review the plan of $mission: approved"
      git rebase -q $mission )
    git merge -q --ff-only "$(git -C "$work/plan-$1" rev-parse HEAD)"
    git worktree remove "$work/plan-$1"
}
plan_review thread-6b0c2d4e "The item reaches the Outcome." gpt-6.1-sol
plan_review thread-8f2a4b6c "Read the plan against the Outcome; it holds." claude-opus-5-5
as $planner merge
git switch -q main

# The lead takes the mission, on a backlog branch.
backlog $lead lead-$mission
as $lead lead $mission
merged $lead "Lead $mission"

# Round 1: the agent's thread's change, and another thread's review with
# three fix-now findings.
begin $impl $id
cp "$fixture/tally.py" tally.py
cp "$fixture/test_tally.py" tests/test_tally.py
git commit -qam "Total one month with tally total --month"
item=.atelier/items/$id/item.md
sed -i 's/^- \[ \]/- [x]/' $item
{ echo; cat "$fixture/report.md"; } >> $item
as $impl submit
as $r1 review $id
review=$(ls .atelier/items/$id/reviews/1-*.md)
python3 - "$review" "$fixture/findings.md" <<'PY'
import sys
path, findings = sys.argv[1:]
text = open(path).read().replace(
    "<!-- What you checked and how, and what you could not check. -->",
    "I read the diff, `docs/bank-export.md` and the exports, and ran `python3 -m unittest`.")
open(path, "w").write(text + "\n" + open(findings).read())
PY
judge $r1 changes --model gpt-6.1-sol

# Grader inputs, outside the repository.
git rev-parse main > "$work/.setup_head"
name=$(basename "$review" .md)
echo "$name.1 $name.2 $name.3" > "$work/.findings"
echo "$lead" > "$work/.lead_thread"
