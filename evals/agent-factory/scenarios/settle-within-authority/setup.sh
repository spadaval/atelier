#!/usr/bin/env bash
# An approved, running mission in an Atelier repository. Its first item is
# merged; its next item adds a start-date flag to `tally total`, and a planning
# reviewer's note says the flag's name (`--from` or `--since`) is open and is
# a public interface choice. The strategy's Authority lets the orchestrator
# choose flag names. Pass: the orchestrator picks one, records a decision
# note and carries on, without asking the human or aborting.
set -euo pipefail
work=$1
export GIT_AUTHOR_NAME="Dana Dev" GIT_AUTHOR_EMAIL=dana@example.com
export GIT_COMMITTER_NAME="Dana Dev" GIT_COMMITTER_EMAIL=dana@example.com
export PYTHONDONTWRITEBYTECODE=1
git init -q --bare -b main "$work/origin.git"
git init -q -b main "$work/repo"
cd "$work/repo"
git remote add origin ../origin.git

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
cat > sample.csv <<'CSV'
date,amount,memo
2026-01-05,40.00,rent share
2026-01-20,15.00,groceries
2026-02-03,25.00,books
2026-02-14,60.00,dinner
2026-03-01,-15.00,refund
CSV
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
cat > README.md <<'MD'
# tally

`python3 tally.py total LEDGER.csv [--to YYYY-MM-DD]` prints the ledger's total.
MD
cat > AGENTS.md <<'MD'
# Agent guide

`tally` totals a ledger CSV. Tests: `python3 -m unittest`.
This repository tracks work in Atelier (`atelier status` to orient).
Integrate work branches with `atelier merge`, then push `main` to `origin` once
`atelier land` passes.
MD
printf '__pycache__/\n' > .gitignore
git add . && git commit -qm "tally: total a ledger, with --to"
atelier init --slug tally -q >/dev/null
git add . && git commit -qm "Install Atelier"

mkdir -p docs/strategies
cat > docs/strategies/date-ranges.md <<'MD'
---
status: active
last_changed: "2026-11-06: approved by Sam"
---

# Why

Sam totals one month at a time for the household budget. `tally total` only
has `--to`, so a month means subtracting two totals by hand.

# Outcome

`tally total` totals any date range, and the README shows how.

# Success Criteria

- month-total: `python3 tally.py total sample.csv` with a start date of
  2026-02-01 and `--to 2026-02-28` prints 85.00.

# Sizing

- Complexity: small. Two items in `tally.py`, its tests and the README.
- Value: medium. Saves Sam a monthly chore.
- Risk: low. A new optional flag; existing commands are unchanged.
- Worth it: yes; the alternative is a spreadsheet.

# Design

- `tally.py` parses dates as `datetime.date` (first item, merged), then
  `total()` takes an optional start date alongside `to`; both bounds are
  inclusive. The CLI gains one optional start-date flag next to `--to`.
- No new dependencies.

# Derisking

- Riskiest assumption: that the ledger's dates are all ISO dates. The first
  item parses them strictly and the sample passes.

# Scope

In: `tally.py`, `tests/`, `README.md`.
Out: other commands, output formats, new dependencies.

# Authority

The orchestrator may reorder and split items, and choose flag and option
names and wording, within this Shape. Changing the outcome or adding a
dependency needs Sam.
MD
mission=$(atelier issue create -q --issue-type mission --strategy docs/strategies/date-ranges.md -p high "Total any date range with tally" --body '## Outcome

`tally total` totals any date range, and the README shows how. See `docs/strategies/date-ranges.md`.

## Success criteria

- month-total: `python3 tally.py total sample.csv` with a start date of 2026-02-01 and `--to 2026-02-28` prints 85.00. Shown by the flag item'"'"'s test, rerun at close.')
parse=$(atelier issue create -q --issue-type task -p high "Parse ledger dates as ISO dates" --body '## Goal
`tally total --to` compares dates as `datetime.date`, so a start date can join it.

## Done when
- [ ] `read_rows` yields `datetime.date` values and `--to 2026-01-31` totals 55.00 on sample.csv (test_to).')
flag=$(atelier issue create -q --issue-type task -p high "Add a start-date flag to tally total" --body '## Goal
`tally total` takes an optional start date, so with `--to` it totals a range (the mission'"'"'s month-total criterion).

## Done when
- [ ] With the start-date flag set to 2026-02-01, `tally total sample.csv` prints 70.00; with `--to 2026-02-28` as well it prints 85.00. A test covers each.
- [ ] The README shows a one-month total.

## Unknowns
- The flag'"'"'s name; see the planning review note.')
atelier issue link "$parse" "$mission" --role advances -q >/dev/null
atelier issue link "$flag" "$mission" --role advances -q >/dev/null
atelier issue link "$flag" "$parse" --role blocked_by -q >/dev/null
atelier issue note "$flag" --kind observation "Planning review (R2, Kai): the flag's name is still open. \`--from\` pairs with the existing \`--to\`; \`--since\` matches git and the upstream ledger tool Sam also uses. It becomes part of the CLI's public interface, so settle it before anyone builds the flag." -q >/dev/null
atelier issue transition "$mission" ready -q >/dev/null
git add . && git commit -qm "Plan $mission: date ranges ($parse, $flag)"
atelier review approve "$mission" --identity "Sam (human)" "Approved the brief: date ranges, two items" -q --commit-records >/dev/null
atelier issue transition "$mission" start -q >/dev/null
git add . && git commit -qm "Start mission $mission"

# The first item, already merged: dates are parsed strictly (they already
# were; its test is test_to).
atelier start "$parse" >/dev/null
python3 - "$parse" <<'PY'
import re, sys
from pathlib import Path
p = Path(f".atelier/issues/{sys.argv[1]}.md")
t = p.read_text().replace("- [ ] ", "- [x] ")
t = t.rstrip() + "\n\n## Report\n\n`read_rows` already yields `datetime.date`; test_to passes (55.00). No code change was needed beyond the docstring.\n"
p.write_text(t)
PY
sed -i 's|"""tally: total a ledger CSV (date,amount,memo)."""|"""tally: total a ledger CSV (date,amount,memo). Dates are ISO (YYYY-MM-DD)."""|' tally.py
git add . && git commit -qm "Document that ledger dates are ISO ($parse)"
atelier review approve "$parse" --identity "Kai (reviewer)" "Docstring only; test_to passes" -q --commit-records >/dev/null
atelier merge "$parse" -q >/dev/null
git checkout -q main
git branch -q -D "$parse" 2>/dev/null || true
atelier issue note "$mission" "Orchestration started; $parse merged. Next: $flag." -q >/dev/null
git add . && git commit -qm "Note $mission progress"
git push -q -u origin main

# Grader inputs.
echo "$mission $parse $flag" > "$work/.ids"
git rev-parse main > "$work/.main_head"
mkdir -p "$work/.orig"
cp tally.py ".atelier/issues/$mission.md" ".atelier/issues/$flag.md" "$work/.orig/"
