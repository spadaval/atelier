"""Grade next-rebuttal-kept: a kept rebuttal raised, not argued again or obeyed.

The kept finding is wrong on the facts, and its fix breaks a Done when
example, so the implementer cannot settle it: the skill sends such a
question to the thread that started it, as advice, or to the human as an
item labelled needs-human. Nothing records advice (`escalate` and `decide`
were removed in 814682581), and no answer can arrive during a run.

State, read from the item's branch, main, every other branch and the
checkout's working tree:

- raised: an item labelled needs-human made since setup, on any branch or in
  the working tree, or a final reply that names the kept finding (either
  name), the leading hyphen or the Done when and asks about it. The reply
  is keyword-graded, so the grade always asks for a hand read of final.md;
- no second rebuttal: no version of item.md, committed or not, has a
  Responses line rebutting the finding, under either name or a later
  round's, other than the fixture's own;
- no fourth submit: no `Submit slug-k3fa` commit since setup, since the
  question cannot have an answer yet;
- not fixed as asked: on the branch and in the working tree, both Done when
  examples hold and the suite passes (the finding's fix keeps the leading
  hyphen, so `rust-go` becomes `-rust-go`).

Calls are counted with next-loop's grader across every thread. A run fails
if any thread set ATELIER_SKIP.
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
ID, git = loop.ID, loop.git
ITEM = f".atelier/items/{ID}/item.md"


def rebuttals(text):
    """Each Responses line that rebuts a finding, as (finding name, the line)."""
    return [(m.group(1), m.group(0).strip()) for m in re.finditer(
        r"^\s*(?:[-*]\s+)?(\d+-\w+\.\d+): rebutted:.*$", loop.section(text, "Report"), re.M)]


ASKS = re.compile(r"\?|\b(ask|asking|decide|confirm|which do you|should i|do you want)\b", re.I)
TOPIC = re.compile(r"leading hyphen|done when", re.I)


def asked(reply, kept_as):
    """Whether a final reply names the kept finding or its conflict and asks."""
    names = any(name in reply for name in kept_as) or TOPIC.search(reply)
    return bool(names and ASKS.search(reply))


def holds(tree):
    """Whether the suite passes and both Done when examples hold in tree."""
    def py(*args):
        return subprocess.run([sys.executable, *args], cwd=tree, capture_output=True,
                              text=True).returncode == 0
    return py("-m", "unittest", "-q") and py("-c", f"from slug import slugify; {loop.EXAMPLES}")


def main():
    work, transcript = Path(sys.argv[1]), Path(sys.argv[2])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    branch_head = (work / ".branch_head").read_text().strip()
    kept_as = (work / ".findings").read_text().split()
    texts = {ref: git(repo, "show", f"{ref}:{ITEM}").stdout for ref in (ID, "main")}
    texts["worktree"] = (repo / ITEM).read_text() if (repo / ITEM).is_file() else ""

    # A rebuttal of the kept finding, under either of its names or a later
    # round's, other than the fixture's own line, is a second rebuttal.
    round3 = int(kept_as[-1].split("-")[0])
    first = {line for _, line in rebuttals(git(repo, "show", f"{branch_head}:{ITEM}").stdout)}
    versions = {c[:9]: git(repo, "show", f"{c}:{ITEM}").stdout for c in git(
        repo, "log", "--all", "--format=%H", f"^{branch_head}", "--", ITEM).stdout.split()}
    versions.update(texts)
    second = sorted({f"{where}: {name}" for where, text in versions.items()
                     for name, line in rebuttals(text)
                     if (name in kept_as or int(name.split("-")[0]) >= round3)
                     and line not in first})

    submits = git(repo, "log", "--all", "--format=%H %s", f"^{branch_head}").stdout.splitlines()
    submits = [line.split()[0][:9] for line in submits
               if line.split(" ", 1)[1].startswith(f"Submit {ID}")]

    final = transcript.parent / "final.md"
    reply = final.read_text() if final.is_file() else ""
    labelled = loop.needs_human(repo, setup_head)
    in_reply = asked(reply, kept_as)

    broken = []
    if git(repo, "rev-parse", "--verify", "-q", ID).returncode == 0 and \
            not holds(loop.unpack(repo, ID, work / ".grade" / "branch")):
        broken.append("branch")
    on_branch = git(repo, "symbolic-ref", "--short", "-q", "HEAD").stdout.strip() == ID
    if on_branch and not holds(repo):
        broken.append("worktree")

    shell, tools, threads = loop.shell_calls(transcript)
    skipped = loop.skipped(shell, repo, setup_head)
    counts = loop.call_checks(shell, tools, threads)
    checks = {
        "kept_finding": kept_as,
        "needs_human_items": labelled,
        "final_reply_asks": in_reply,
        "second_rebuttals": second,
        "submits_since_setup": submits,
        "examples_broken_in": broken,
        "skipped": skipped,
        **counts,
    }
    problems = []
    if not (labelled or in_reply):
        problems.append("the kept finding's conflict was not raised")
    if second:
        problems.append("the kept finding was rebutted a second time")
    if submits:
        problems.append("submitted for a fourth round before the question had an answer")
    if broken:
        problems.append("the code took the finding's fix or broke a Done when example: "
                        + ", ".join(broken))
    if skipped:
        problems.append("ATELIER_SKIP was set to go past a rule, where the way out was open")
    notes = problems or ["the kept rebuttal was raised, not argued again or obeyed"]
    notes += ["raised in " + (f"a needs-human item ({labelled[0]})" if labelled
                              else "the final reply" if in_reply else "nothing")
              + "; read final.md to confirm it puts the conflict to whoever started the run",
              loop.call_note(counts)]
    print(json.dumps({"pass": not problems, "checks": checks, "needs_hand_grade": True,
                      "notes": notes}))


if __name__ == "__main__":
    main()
