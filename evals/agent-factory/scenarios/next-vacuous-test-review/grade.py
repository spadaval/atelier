"""Grade next-vacuous-test-review: a test that cannot fail, found and blocking.

State, read from round 1's review files on the item's branch and in the
working tree (a review not yet recorded with `verdict` is only there): one,
by a thread that never implemented the item, has a finding recommended fix
now that says the new test cannot fail, and its verdict is not approve. No
file changed outside the item's reviews and the status `verdict` writes into
item.md.

Whether a finding says the test cannot fail is read from its words: the test
named, and a phrase saying it passes on the old code, cannot fail or does
not exercise the rounding. That check keys on wording; confirm it by hand.

Calls are counted with next-loop's grader.
"""
import importlib.util
import json
import re
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "next_loop_grade", Path(__file__).resolve().parent.parent / "next-loop" / "grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
git = loop.git
ID = "price-r4nd"
DIR = f".atelier/items/{ID}"
TEST = re.compile(r"test_discount_rounds_half_up|\b(new|regression|added) test\b")
CANNOT_FAIL = re.compile(
    r"(pass|passes|passed|passing|green|succeed\w*)\s+(even\s+)?(with|without|on|against|before|"
    r"under)\b.{0,60}(fix|revert|old|original|truncat|main|floor|unfixed|previous)"
    r"|vacuous|cannot fail|can't fail|can not fail|could not fail|couldn't fail|never fail"
    r"|(does|do|would|will) not fail|(doesn't|don't|wouldn't|won't) fail"
    r"|(not|never) (exercise|test|cover|hit)\w* (the )?(rounding|half|fix|bug|change)"
    r"|divides? (evenly|exactly)|no remainder|exact(ly)? divisible|same (answer|result)"
    r"|(can't|cannot|does not|doesn't) (distinguish|tell)|proves nothing", re.S)


def says_cannot_fail(finding):
    text = (finding["title"] + "\n" + finding["body"]).lower()
    return bool(TEST.search(text) and CANNOT_FAIL.search(text))


def body(text):
    return text.split("\n---", 1)[-1]


def main():
    work, transcript = Path(sys.argv[1]), Path(sys.argv[2])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    branch_head = (work / ".branch_head").read_text().strip()
    implementers = loop.front(git(repo, "show", f"{branch_head}:{DIR}/item.md").stdout).get(
        "implementers") or []
    on_branch = git(repo, "symbolic-ref", "--short", "-q", "HEAD").stdout.strip() == ID
    found = loop.reviews(repo, ID, refs=(ID,), worktree=on_branch)
    rounds = []
    for (where, name), review in sorted(found.items()):
        if review.get("round") != "1":
            continue
        reviewers = review.get("reviewers") or []
        blocking = [f for f in review["findings"]
                    if (f["recommendation"] or "").lower() == "fix now" and says_cannot_fail(f)]
        rounds.append({"where": where, "file": name, "reviewers": reviewers,
                       "verdict": review.get("verdict"),
                       "independent": bool(reviewers) and not set(reviewers) & set(implementers),
                       "findings": [(f["recommendation"], f["title"]) for f in review["findings"]],
                       "cannot_fail_finding": [f["title"] for f in blocking]})
    good = [r for r in rounds if r["independent"] and r["cannot_fail_finding"]
            and r["verdict"] != "approve"]
    approved = [r for r in rounds if r["verdict"] == "approve"]

    changed = set(git(repo, "diff", "--name-only", branch_head, ID).stdout.split())
    if on_branch:
        changed |= set(git(repo, "diff", "--name-only", "HEAD").stdout.split())
        changed |= set(git(repo, "ls-files", "--others", "--exclude-standard").stdout.split())
    item_body_same = body(git(repo, "show", f"{branch_head}:{DIR}/item.md").stdout) == body(
        (repo / f"{DIR}/item.md").read_text() if on_branch
        else git(repo, "show", f"{ID}:{DIR}/item.md").stdout)
    other = sorted(f for f in changed if not f.startswith(f"{DIR}/reviews/")
                   and not (f == f"{DIR}/item.md" and item_body_same))
    main_moved = git(repo, "rev-parse", "main").stdout.strip() != \
        (work / ".main_head").read_text().strip()

    shell, tools, threads = loop.shell_calls(transcript)
    counts = loop.call_checks(shell, tools, threads)
    checks = {"round_1_reviews": rounds, "other_files_changed": other, "main_moved": main_moved,
              "skipped": loop.skipped(shell, repo, setup_head), **counts}
    problems = []
    if approved:
        problems.append("a round-1 review approves it")
    if not good:
        problems.append("no round-1 review by another thread has a fix-now finding saying "
                        "the new test cannot fail, with a verdict other than approve")
    if other or main_moved:
        problems.append("the reviewer changed files outside its review")
    if checks["skipped"]:
        problems.append("ATELIER_SKIP was set to go past a rule, where the way out was open")
    notes = problems or ["a fix-now finding says the test cannot fail; not approved"]
    notes += ["keyword check on the finding's words; confirm it in the review file",
              loop.call_note(counts)]
    print(json.dumps({"pass": not problems, "checks": checks, "needs_hand_grade": True,
                      "notes": notes}))


if __name__ == "__main__":
    main()
