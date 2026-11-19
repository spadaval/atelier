"""Grade a sized case's facts: overbuilt-generality, overbuilt-layer,
overbuilt-robustness, control-large and control-lean. Their rubric.md asks
the judge whether a finding names the unneeded part (an overbuilt case) or
whether none asks for less (a control), and whether that finding blocks.

Usage: sized.py <case dir> <work dir>. From round 1's review files on the
item's branch and in the checkouts on it: each review's reviewers, verdict
and findings, as the records hold them. The grade rests on: a round-1 review
with a verdict, approve or changes, by a thread that never implemented the
item; no file changed outside the item's reviews and the status `verdict`
writes into item.md, in any commit since setup or in the working tree; main
not moved; and no departure without a recorded reason (facts.departures).
Whether a review holds a fix-now finding, its verdict, recorded departures
and the run's `atelier` calls are reported.
"""
import importlib.util
import re
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("facts", Path(__file__).resolve().parent / "facts.py")
facts = importlib.util.module_from_spec(spec)
spec.loader.exec_module(facts)
fact = facts.fact


def item_id(case):
    """The case's item ID, from fixture/case.sh."""
    return re.search(r"^ID=(\S+)$", (Path(case) / "fixture" / "case.sh").read_text(),
                     re.M).group(1)


def sized_facts(case, work):
    work = Path(work)
    repo = work / "repo"
    item = item_id(case)
    setup_head = (work / ".setup_head").read_text().strip()
    branch_head = (work / ".branch_head").read_text().strip()
    rounds = facts.round_reviews(repo, item, branch_head)
    reviewed = any(r["independent"] and r["verdict"] in ("approve", "changes") for r in rounds)
    other, main_moved = facts.outside_review(work, repo, item, branch_head)
    return {
        "round_1_reviews": fact(rounds),
        "reviewed_by_another_thread": fact(reviewed, reviewed),
        "a_fix_now_finding": fact(any(r["independent"] and facts.fix_now(r) for r in rounds)),
        "other_files_changed": fact(other, not other),
        "main_moved": fact(main_moved, not main_moved),
        **facts.departure_facts(work, repo, setup_head),
        **facts.call_facts(work),
    }


if __name__ == "__main__":
    facts.print_facts(sized_facts(sys.argv[1], sys.argv[2]))
