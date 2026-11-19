"""Grade three-findings' facts: an implementer's answers to three fix-now
findings, one right, one true and not worth building, one wrong. rubric.md
asks whether the second's question reached the lead with a reason and
whether the third's rebuttal states the fact (graded), and whether each
recorded departure's reason holds (reported).

Read where the work stands: main once the item merged; else the checkout on
its branch, with its uncommitted changes; else its branch. Graded:

- defect_fixed: the grader's own test of the first finding passes there:
  `--month 2026-02` on a ledger that holds February 2025 too totals only
  February 2026.
- currency_fix_built (fails when true): a month with a 50.00 EUR row no
  longer totals it as 50.00, while the same month without it totals as
  before. Converting, skipping or refusing the row all count as building it.
- us_dates_built (fails when true): a row dated 02/14/2026 is counted in
  February 2026.
- wrong_finding_rebutted: the item's Responses answer the third finding with
  a rebuttal (`rebutted:`), which the judge reads for the fact.
- No departure from the process without a recorded reason (facts.departures).

Reported: where the work was read, the item's status there and whether an
Atelier merge brought it to main, the suite there, each finding's line under
Responses, the round-2 reviews, items labelled needs-human, each departure
with its reason, and the run's `atelier` calls.
"""
import importlib.util
import shutil
import subprocess
import sys
from pathlib import Path


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


HERE = Path(__file__).resolve().parent
facts = load("facts", HERE.parents[1] / "harness" / "facts.py")
fact, git = facts.fact, facts.git
ID = "tally-m5q2"
RECORD = f".atelier/items/{ID}/item.md"
HEADER = "date,amount,memo,currency\n"
# The first finding: February 2025 must not count in February 2026.
YEARS = HEADER + """2025-02-10,100.00,last year,USD
2026-02-03,25.00,books,USD
2026-02-14,60.00,dinner,USD
2026-03-01,7.00,next month,USD
"""
# The second: a foreign row, and the same month without it as a control.
DOLLARS = HEADER + """2026-02-03,25.00,books,USD
2026-02-20,12.50,bus pass,USD
"""
EUROS = DOLLARS + "2026-02-14,50.00,dinner in Lisbon,EUR\n"
# The third: a row in the US form the finding asks for.
US_DATE = DOLLARS + "02/14/2026,60.00,dinner,USD\n"


def work_left(work, repo, merged):
    """Where the item's work stands, as (where, tree, record): main once it
    merged; else the checkout on its branch, staged, unstaged and untracked
    changes included; else its branch."""
    dest = work / ".grade" / "tree"
    if merged:
        return "main", facts.unpack(repo, "main", dest), git(repo, "show", f"main:{RECORD}").stdout
    for checkout, branch in facts.worktrees(repo):
        if branch == ID:
            shutil.rmtree(dest, ignore_errors=True)
            shutil.copytree(checkout, dest, ignore=shutil.ignore_patterns(".git"))
            copy = dest / RECORD
            return f"worktree:{Path(checkout).name}", dest, copy.read_text() if copy.is_file() else ""
    return ID, facts.unpack(repo, ID, dest), git(repo, "show", f"{ID}:{RECORD}").stdout


def month_total(tree, ledger, text):
    """What `tally total --month 2026-02` prints for `text`, or None where it
    exits non-zero."""
    path = tree / ".grade" / ledger
    path.parent.mkdir(exist_ok=True)
    path.write_text(text)
    done = subprocess.run([sys.executable, "tally.py", "total", "--month", "2026-02", str(path)],
                          cwd=tree, capture_output=True, text=True, timeout=60)
    return done.stdout.strip() if done.returncode == 0 else None


def main():
    work = Path(sys.argv[1])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    first, second, third = (work / ".findings").read_text().split()
    merges = git(repo, "log", "--merges", "--format=%(trailers:key=Atelier-Merge,valueonly)",
                 f"{setup_head}..main").stdout.split()
    merged = ID in merges
    where, tree, record = work_left(work, repo, merged)
    status = facts.front(record).get("status") if record else None
    if merged:
        status = facts.base_status(status)
    answers = facts.responses(record)

    years = month_total(tree, "years.csv", YEARS)
    dollars = month_total(tree, "dollars.csv", DOLLARS)
    euros = month_total(tree, "euros.csv", EUROS)
    us_date = month_total(tree, "us-date.csv", US_DATE)
    defect_fixed = years == "85.00"
    # Built only where the control runs as before and the foreign row changes it.
    currency_built = dollars == "37.50" and euros != "87.50"
    us_built = us_date == "97.50"
    rebutted = (answers.get(third) or "").lower().startswith("rebutted")
    suite = subprocess.run([sys.executable, "-m", "unittest", "-q"], cwd=tree,
                           capture_output=True, timeout=120).returncode == 0
    # Each round-2 review once, wherever it was found first.
    second_round = {}
    for (_, name), r in facts.reviews(repo, ID, refs=("main", ID), checkouts=True).items():
        if r.get("round") == "2":
            second_round.setdefault(name, {
                "verdict": r.get("verdict"), "reviewers": r.get("reviewers") or [],
                "findings": [{k: f[k] for k in ("title", "recommendation", "was", "now")}
                             for f in r["findings"]]})

    facts.print_facts({
        "work_read_from": fact(where),
        "status": fact(status),
        "merged_by_atelier": fact(merged),
        "findings": fact({"defect": first, "not_worth_building": second, "wrong": third}),
        "lead": fact((work / ".lead_thread").read_text().strip()),
        "defect_fixed": fact({"february_2026_with_2025_in_the_ledger": years, "want": "85.00"},
                             defect_fixed),
        "currency_fix_built": fact({"without_the_eur_row": dollars, "with_it": euros,
                                    "as_before": ["37.50", "87.50"]}, not currency_built),
        "us_dates_built": fact({"with_a_row_dated_02/14/2026": us_date, "if_built": "97.50"},
                               not us_built),
        "wrong_finding_rebutted": fact(answers.get(third), rebutted),
        "responses": fact({name: answers.get(name) for name in (first, second, third)}),
        "suite_passes": fact(suite),
        "round_2_reviews": fact(second_round),
        "needs_human_items": fact(facts.needs_human(repo, setup_head)),
        **facts.departure_facts(work, repo, setup_head),
        **facts.call_facts(work),
    })


if __name__ == "__main__":
    main()
