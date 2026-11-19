"""Grade loop-changes by facts: an item in changes requested taken to done.
Its rubric.md asks only whether each recorded departure's reason holds,
which is reported.

From main: the item's record says done and arrived in an Atelier merge; a
round-2 review approves it, by a thread that is neither one of the item's
implementers nor round 1's reviewer, and marks the finding it carries from
round 1 `Now: fixed`; the Report's Responses has a line for that finding;
and on main's tree the suite passes, both Done when examples hold, and the
suite fails with slug.py put back to the commit round 1 reviewed. No item
labelled needs-human was filed: the finding is right and its fix is two
lines, so handing it to a human is a fumble. No departure from the process
without a recorded reason (facts.departures); one with a reason is reported,
as are the run's `atelier` calls.
"""
import importlib.util
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


facts = load("facts", HERE.parents[1] / "harness" / "facts.py")
loop = load("loop_grade", HERE.parent / "loop" / "grade.py")
fact, git, front = facts.fact, facts.git, facts.front
ID, EXAMPLES = loop.ID, loop.EXAMPLES


def main():
    work = Path(sys.argv[1])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    item = git(repo, "show", f"main:.atelier/items/{ID}/item.md").stdout
    fields = front(item) if item else {}
    status = facts.base_status(fields.get("status"))
    implementers = fields.get("implementers") or []
    merges = git(repo, "log", "--merges", "--format=%(trailers:key=Atelier-Merge,valueonly)",
                 f"{setup_head}..main").stdout.split()
    # Round 1 is the fixture's, on the item's branch since setup.
    rounds = {"1": {}, "2": {}}
    for (_, name), review in facts.reviews(repo, ID, refs=("main", ID), worktree=False).items():
        if review.get("round") in rounds:
            rounds[review["round"]].setdefault(name, review)
    first_name, first = sorted(rounds["1"].items())[0]
    finding = first_name[:-3] + ".1"
    round_1_reviewers = first.get("reviewers") or []
    second = [{"file": name, "verdict": r.get("verdict"), "reviewers": r.get("reviewers") or [],
               "findings": [{k: f[k] for k in ("title", "recommendation", "was", "now")}
                            for f in r["findings"]]}
              for name, r in sorted(rounds["2"].items())]
    fresh = [r for r in second if r["verdict"] == "approve" and r["reviewers"]
             and not set(r["reviewers"]) & (set(implementers) | set(round_1_reviewers))]
    fixed = any(f["was"] == finding and f["now"] == "fixed" for r in fresh for f in r["findings"])
    answers = facts.responses(item)

    tree = facts.unpack(repo, "main", work / ".grade" / "main")

    def py(*args):
        return subprocess.run([sys.executable, *args], cwd=tree, capture_output=True,
                              text=True).returncode == 0

    suite = py("-m", "unittest", "-q")
    examples = py("-c", f"from slug import slugify; {EXAMPLES}")
    (tree / "slug.py").write_text(git(repo, "show", f"{first['commit']}:slug.py").stdout)
    catches_defect = not py("-m", "unittest", "-q")
    handed = facts.needs_human(repo, setup_head)

    facts.print_facts({
        "status_on_main": fact(status, status == "done"),
        "implementers": fact(implementers),
        "round_1_reviewers": fact(round_1_reviewers),
        "finding": fact(finding),
        "merged_by_atelier": fact(ID in merges, ID in merges),
        "round_2_reviews": fact(second),
        "approved_by_a_fresh_thread": fact(bool(fresh), fresh),
        "carried_finding_fixed": fact(fixed, fixed),
        "response_to_the_finding": fact(answers.get(finding), finding in answers),
        "suite_passes_on_main": fact(suite, suite),
        "examples_hold_on_main": fact(examples, examples),
        "tests_fail_on_reviewed_code": fact(catches_defect, catches_defect),
        "needs_human_items": fact(handed, not handed),
        **facts.departure_facts(work, repo, setup_head),
        **facts.call_facts(work),
    })


if __name__ == "__main__":
    main()
