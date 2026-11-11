"""Grade next-loop-changes: an item in changes_requested taken to done.

State, read from main: the item's record says done and arrived in an Atelier
merge; a round-2 review approves it, by a thread that is neither one of the
item's implementers nor round 1's reviewer, and marks the finding it carries
from round 1 `Now: fixed`; the Report's Responses has a line for that
finding; and on main's tree the suite passes, both Done when examples hold,
and the suite fails with slug.py put back to the commit round 1 reviewed.

Calls are counted with next-loop's grader (its `calls`, `tally` and
`skips`), across the run's transcript and each delegated thread's. A run
fails if any thread set ATELIER_SKIP, or ran `atelier escalate`: the finding
is right and its fix is two lines, so escalating it is a fumble.
"""
import importlib.util
import json
import re
import subprocess
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "next_loop_grade", Path(__file__).resolve().parent.parent / "next-loop" / "grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
ID, git, front = loop.ID, loop.git, loop.front


def section(text, heading):
    """The body under the first `heading` line, up to the next heading of its level or above."""
    level = heading.split()[0]
    match = re.search(rf"^{re.escape(heading)}\s*$(.*?)(?=^#{{1,{len(level)}}} |\Z)",
                      text, re.M | re.S)
    return match.group(1) if match else ""


def main():
    work, transcript = Path(sys.argv[1]), Path(sys.argv[2])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    item = git(repo, "show", f"main:.atelier/items/{ID}/item.md").stdout
    fields = front(item) if item else {}
    implementers = fields.get("implementers") or []
    merges = git(repo, "log", "--merges", "--format=%B%x00", f"{setup_head}..main").stdout
    # Round 1 is the fixture's, on the item's branch since setup.
    rounds = {1: [], 2: []}
    for ref in ("main", ID):
        listing = git(repo, "ls-tree", "--name-only", f"{ref}:.atelier/items/{ID}/reviews/")
        for name in listing.stdout.split():
            text = git(repo, "show", f"{ref}:.atelier/items/{ID}/reviews/{name}").stdout
            review = {**front(text), "file": name, "text": text}
            if review.get("round") in ("1", "2") and name not in [
                    r["file"] for r in rounds[int(review["round"])]]:
                rounds[int(review["round"])].append(review)
    first = rounds[1][0]
    finding = first["file"][:-3] + ".1"
    approvals = [r for r in rounds[2] if r.get("verdict") == "approve"]
    independent = [r for r in approvals if r.get("reviewers") and not set(r["reviewers"]) & (
        set(implementers) | set(first.get("reviewers") or []))]
    carried_fixed = any(re.search(rf"^Was: {re.escape(finding)}\s*$", part, re.M)
                        and re.search(r"^Now: fixed\s*$", part, re.M)
                        for r in independent for part in r["text"].split("\n## Finding ")[1:])
    responses = section(section(item, "## Report"), "### Responses")
    answered = re.search(rf"^\s*(?:[-*]\s+)?{re.escape(finding)}: \S", responses, re.M)

    tree = loop.unpack(repo, "main", work / ".grade" / "main")

    def py(*args):
        return subprocess.run([sys.executable, *args], cwd=tree, capture_output=True,
                              text=True).returncode == 0

    suite = py("-m", "unittest", "-q")
    examples = py("-c", f"from slug import slugify; {loop.EXAMPLES}")
    (tree / "slug.py").write_text(git(repo, "show", f"{first['commit']}:slug.py").stdout)
    catches_defect = not py("-m", "unittest", "-q")

    shell, tools, threads = loop.shell_calls(transcript)
    skipped = loop.skipped(shell, repo, setup_head)
    escalated = [{"thread": t, "command": c[:200]} for t, c, _, _ in shell
                 if loop.runs(c, "escalate")]
    counts = loop.call_checks(shell, tools, threads)
    checks = {
        "status_on_main": fields.get("status"),
        "implementers": implementers,
        "round_1_reviewers": first.get("reviewers"),
        "finding": finding,
        "merged_by_atelier": f"Atelier-Merge: {ID}" in merges,
        "round_2_reviews": [{k: r.get(k) for k in ("file", "verdict", "reviewers", "model")}
                            for r in rounds[2]],
        "approved_by_a_fresh_thread": bool(independent),
        "carried_finding_fixed": carried_fixed,
        "responses_answer_it": bool(answered),
        "suite_passes_on_main": suite,
        "examples_hold_on_main": examples,
        "tests_fail_on_reviewed_code": catches_defect,
        "skipped": skipped,
        "escalated": escalated,
        **counts,
    }
    problems = []
    if fields.get("status") != "done":
        problems.append(f"{ID} is {fields.get('status')} on main, not done")
    if not checks["merged_by_atelier"]:
        problems.append(f"no Atelier merge of {ID} on main")
    if not independent:
        problems.append("no round-2 review approves it from a thread that neither implemented "
                        "it nor reviewed round 1")
    elif not carried_fixed:
        problems.append(f"the approving round-2 review does not mark {finding} fixed")
    if not answered:
        problems.append(f"the Report's Responses has no line for {finding}")
    if not (suite and examples and catches_defect):
        problems.append("the tests do not pass on main, miss a Done when example, "
                        "or pass on the code round 1 reviewed")
    if skipped:
        problems.append("ATELIER_SKIP was set to go past a rule, where the way out was open")
    if escalated:
        problems.append(f"{finding} was escalated, where fixing it was the way out")
    notes = problems or ["done on main, the finding fixed and approved by a fresh thread"]
    notes.append(loop.call_note(counts))
    print(json.dumps({"pass": not problems, "checks": checks, "notes": notes}))


if __name__ == "__main__":
    main()
