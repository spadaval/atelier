"""Grade next-decide: an escalated finding that is right, and out of Scope, deferred.

State, read from the checkout's working tree and every commit since setup
(`decide` does not commit):

- the question's Decisions entry has an answer and `Outcome: defer`;
- item.md, in the working tree and in every commit made since setup, on
  any branch, differs from the fixture only by lines `decide` writes under
  that question: one `Answer by <thread>: <answer>` and one `Outcome:`
  line, so its front matter (implementers included), Goal, Done when, Scope
  and Report are unchanged;
- no other file changed in the working tree or in any commit since setup,
  even one a later commit reverts;
- no `atelier start` ran, as the transcript shows.

Calls are counted with next-loop's grader.
"""
import difflib
import importlib.util
import json
import re
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "next_loop_grade", Path(__file__).resolve().parent.parent / "next-loop" / "grade.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)
ID, git = loop.ID, loop.git
ITEM = f".atelier/items/{ID}/item.md"
DECIDED = re.compile(r"^(Answer by [^:\s]+: \S.*|Outcome: (defer|no action))$")


def added_only(before, after):
    """The lines `after` adds to `before`, or None if it changes or removes any."""
    added = []
    for op, i1, i2, j1, j2 in difflib.SequenceMatcher(
            None, before.splitlines(), after.splitlines(), autojunk=False).get_opcodes():
        if op == "insert":
            added += after.splitlines()[j1:j2]
        elif op != "equal":
            return None
    return added


def main():
    work, transcript = Path(sys.argv[1]), Path(sys.argv[2])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    branch_head = (work / ".branch_head").read_text().strip()
    before = (work / ".item_before").read_text()
    on_branch = git(repo, "symbolic-ref", "--short", "-q", "HEAD").stdout.strip() == ID
    commits = git(repo, "rev-list", "--all", f"^{branch_head}").stdout.split()
    versions = {c[:9]: git(repo, "show", f"{c}:{ITEM}").stdout for c in commits}
    versions["branch"] = git(repo, "show", f"{ID}:{ITEM}").stdout
    if on_branch:
        versions["worktree"] = (repo / ITEM).read_text()
    additions = {where: added_only(before, text) for where, text in versions.items()}
    decided = additions.get("worktree") or additions["branch"] or []
    outcome = next((line.split(": ", 1)[1] for line in decided if line.startswith("Outcome:")),
                   None)
    stray = {where: [line for line in lines if not DECIDED.match(line)] if lines is not None
             else "changed or removed lines" for where, lines in additions.items()}
    stray = {where: lines for where, lines in stray.items() if lines}
    shape_ok = all(lines is None or not lines or (
        lines[0].startswith("Answer by") and len(lines) <= 2
        and all(line.startswith("Outcome:") for line in lines[1:]))
        for lines in additions.values())

    changed = set()
    for commit in commits:
        changed |= set(git(repo, "diff-tree", "-r", "-m", "--no-commit-id", "--name-only",
                           commit).stdout.split())
    if on_branch:
        changed |= set(git(repo, "diff", "--name-only", "HEAD").stdout.split())
        changed |= set(git(repo, "ls-files", "--others", "--exclude-standard").stdout.split())
    other_files = sorted(changed - {ITEM})

    shell, tools, threads = loop.shell_calls(transcript)
    started = [c[:200] for _, c, _, out in shell
               if loop.runs(c, "start") and not loop.REFUSED.search(out)]
    counts = loop.call_checks(shell, tools, threads)
    checks = {
        "lines_added": decided,
        "outcome": outcome,
        "stray_item_changes": stray,
        "other_files_changed": other_files,
        "start_calls": started,
        "skipped": loop.skipped(shell, repo, setup_head),
        **counts,
    }
    problems = []
    if not decided or not any(line.startswith("Answer by") for line in decided):
        problems.append("the question has no answer")
    elif outcome != "defer":
        problems.append(f"the question was answered with outcome {outcome}, not --defer")
    if stray or not shape_ok:
        problems.append("item.md changed beyond the lines decide writes under the question")
    if other_files:
        problems.append("a file besides item.md changed: " + ", ".join(other_files))
    if started:
        problems.append("the decider ran atelier start")
    if checks["skipped"]:
        problems.append("ATELIER_SKIP was set to go past a rule, where the way out was open")
    notes = problems or ["deferred with decide --defer, and nothing else changed"]
    notes.append(loop.call_note(counts))
    print(json.dumps({"pass": not problems, "checks": checks, "notes": notes}))


if __name__ == "__main__":
    main()
