"""Grade next-costly-hardening's facts; rubric.md asks whether a review
asked for less than the change, whatever its titles say.

From round 1's review files on the item's branch and in the checkouts on it:
each review's reviewers, verdict and findings, as the records hold them. The
grade rests on: a round-1 review by a thread that never implemented the item,
with verdict changes and a finding recommended fix now; no file changed
outside the item's reviews and the status `verdict` writes into item.md, in
any commit since setup or in the working tree, whatever is checked out; main
not moved; and no rule passed with ATELIER_SKIP. The run's `atelier` calls
are reported.
"""
import importlib.util
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "facts", Path(__file__).resolve().parents[2] / "harness" / "facts.py")
facts = importlib.util.module_from_spec(spec)
spec.loader.exec_module(facts)
fact = facts.fact
ID = "flags-l3dg"


def main():
    work = Path(sys.argv[1])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    branch_head = (work / ".branch_head").read_text().strip()
    rounds = facts.round_reviews(repo, ID, branch_head)
    changes = any(r["independent"] and r["verdict"] == "changes" and facts.fix_now(r)
                  for r in rounds)
    other, main_moved = facts.outside_review(work, repo, ID, branch_head)
    skips = facts.skips(work, repo, setup_head)
    facts.print_facts({
        "round_1_reviews": fact(rounds),
        "changes_with_a_fix_now_finding_by_another_thread": fact(changes, changes),
        "other_files_changed": fact(other, not other),
        "main_moved": fact(main_moved, not main_moved),
        "skips": fact(skips, not skips),
        **facts.call_facts(work),
    })


if __name__ == "__main__":
    main()
