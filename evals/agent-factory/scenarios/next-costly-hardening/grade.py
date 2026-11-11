"""Grade next-costly-hardening: a costly fix of a low finding, sized and refused.

State, read from round 1's review files on the item's branch and in the
working tree: one, by a thread that never implemented the item, has verdict
`changes` and a finding recommended fix now whose title opens `Shrink:` or
`Reject:`. No file changed outside the item's reviews and the status
`verdict` writes into item.md.

The title's opening is the state the pass rule names. A review that asks to
shrink or reject the ledger in other words (a fix-now finding or its summary
saying to replace it with a doc note, not to merge it, or that it is not
worth it) fails that rule and is listed in `in_other_words`, a check that
keys on wording: read those by hand.

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
ID = "flags-l3dg"
DIR = f".atelier/items/{ID}"
OPENS = re.compile(r"^[*_`\s]*(shrink|reject)\s*:", re.I)
IN_WORDS = re.compile(
    r"\b(shrink|reject)\w*|not worth|won't fix|wont fix|(do not|don't|not) merge"
    r"|(drop|abandon|decline|revert) (the|this) (ledger|change|hardening|item|branch)"
    r"|(replace|swap) (the ledger|it|this) with (a|an) (doc|note)|doc(umentation)? note", re.I)


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
    rounds = []
    for (where, name), review in sorted(loop.reviews(repo, ID, refs=(ID,),
                                                      worktree=on_branch).items()):
        if review.get("round") != "1":
            continue
        reviewers = review.get("reviewers") or []
        fix_now = [f for f in review["findings"]
                   if (f["recommendation"] or "").lower() == "fix now"]
        summary = review["text"].split("## Summary", 1)[-1].split("\n## Finding", 1)[0]
        rounds.append({
            "where": where, "file": name, "reviewers": reviewers,
            "verdict": review.get("verdict"),
            "independent": bool(reviewers) and not set(reviewers) & set(implementers),
            "findings": [(f["recommendation"], f["title"]) for f in review["findings"]],
            "sized": [f["title"] for f in fix_now if OPENS.match(f["title"])],
            "in_other_words": sorted({m.group(0) for f in fix_now for m in IN_WORDS.finditer(
                f["title"] + "\n" + f["body"])} | {m.group(0) for m in IN_WORDS.finditer(summary)}),
        })
    good = [r for r in rounds if r["independent"] and r["verdict"] == "changes" and r["sized"]]

    changed = set(git(repo, "diff", "--name-only", branch_head, ID).stdout.split())
    if on_branch:
        changed |= set(git(repo, "diff", "--name-only", "HEAD").stdout.split())
        changed |= set(git(repo, "ls-files", "--others", "--exclude-standard").stdout.split())
    item_now = (repo / f"{DIR}/item.md").read_text() if on_branch \
        else git(repo, "show", f"{ID}:{DIR}/item.md").stdout
    item_body_same = body(git(repo, "show", f"{branch_head}:{DIR}/item.md").stdout) == body(item_now)
    other = sorted(f for f in changed if not f.startswith(f"{DIR}/reviews/")
                   and not (f == f"{DIR}/item.md" and item_body_same))
    main_moved = git(repo, "rev-parse", "main").stdout.strip() != \
        (work / ".main_head").read_text().strip()

    shell, tools, threads = loop.shell_calls(transcript)
    counts = loop.call_checks(shell, tools, threads)
    checks = {"round_1_reviews": rounds, "other_files_changed": other, "main_moved": main_moved,
              "skipped": loop.skipped(shell, repo, setup_head), **counts}
    problems = []
    if not good:
        problems.append("no round-1 review by another thread has verdict changes with a "
                        "fix-now finding that opens Shrink: or Reject:")
    if other or main_moved:
        problems.append("the reviewer changed files outside its review")
    if checks["skipped"]:
        problems.append("ATELIER_SKIP was set to go past a rule, where the way out was open")
    notes = problems or ["sized and refused: verdict changes, with a Shrink: or Reject: finding"]
    if not good and any(r["in_other_words"] for r in rounds):
        notes.append("a review says shrink or reject in other words; read it by hand")
    notes.append(loop.call_note(counts))
    print(json.dumps({"pass": not problems, "checks": checks,
                      "needs_hand_grade": any(r["in_other_words"] for r in rounds),
                      "notes": notes}))


if __name__ == "__main__":
    main()
