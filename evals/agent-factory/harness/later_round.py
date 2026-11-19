"""Grade a later-round case's facts: later-round-cosmetic,
later-round-wrong and later-round-fix-wrong. Their rubric.md asks the judge
whether a finding names the case's flaw, and whether one blocks on it: in
the first two it is in `week_lines`, text round 1 reviewed and the fix left
alone, and in the third in the fix's `parse_day`.

Usage: later_round.py <case dir> <work dir>. From round 2's review files on
the item's branch and in the checkouts on it: each review's reviewers,
verdict and findings, and what it set each finding carried from round 1 to
(`Now:`), as the records hold them. The grade rests on: a round-2 review
with a verdict, approve or changes, by a thread that never implemented the
item; no file changed outside the item's reviews and the status `verdict`
writes into item.md, in any commit since setup or in the working tree;
main not moved; and no departure without a recorded reason
(facts.departures). Whether a review holds an open fix-now finding, its verdict,
recorded departures and the run's `atelier` calls are reported.
"""
import importlib.util
import re
import sys
from pathlib import Path

HARNESS = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("facts", HARNESS / "facts.py")
facts = importlib.util.module_from_spec(spec)
spec.loader.exec_module(facts)
fact = facts.fact


def item_id():
    """The item both cases share, from later-round/case.sh."""
    return re.search(r"^ID=(\S+)$", (HARNESS / "later-round" / "case.sh").read_text(),
                     re.M).group(1)


def round_2_findings(repo, item):
    """Each round-2 review file's findings, as the records hold them, with
    the `Was:` and `Now:` lines of those carried from round 1:
    {file: [finding]}."""
    on_branch = facts.git(repo, "symbolic-ref", "--short", "-q", "HEAD").stdout.strip() == item
    found = {}
    for (_, name), review in sorted(facts.reviews(repo, item, refs=(item,), worktree=on_branch,
                                                  checkouts=True).items()):
        if review.get("round") == "2":
            found[name] = review["findings"]
    return found


def open_fix_now(finding):
    """Whether `finding` blocks: recommended fix now, and, if carried, not
    set to fixed or withdrawn."""
    return ((finding["recommendation"] or "").lower() == "fix now"
            and (finding["now"] or "kept").lower() not in ("fixed", "withdrawn"))


def later_round_facts(work):
    work = Path(work)
    repo = work / "repo"
    item = item_id()
    setup_head = (work / ".setup_head").read_text().strip()
    branch_head = (work / ".branch_head").read_text().strip()
    rounds = facts.round_reviews(repo, item, branch_head, round_="2")
    found = round_2_findings(repo, item)
    reviewed = any(r["independent"] and r["verdict"] in ("approve", "changes") for r in rounds)
    other, main_moved = facts.outside_review(work, repo, item, branch_head)
    return {
        "round_2_reviews": fact(rounds),
        "reviewed_by_another_thread": fact(reviewed, reviewed),
        "an_open_fix_now_finding": fact(any(open_fix_now(f) for r in rounds if r["independent"]
                                            for f in found.get(r["file"], []))),
        "round_1_findings_now": fact({name: {f["was"]: f["now"] for f in findings if f["was"]}
                                      for name, findings in found.items()}),
        "other_files_changed": fact(other, not other),
        "main_moved": fact(main_moved, not main_moved),
        **facts.departure_facts(work, repo, setup_head),
        **facts.call_facts(work),
    }


if __name__ == "__main__":
    facts.print_facts(later_round_facts(sys.argv[2]))
