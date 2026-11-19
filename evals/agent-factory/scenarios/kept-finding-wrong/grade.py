"""Grade kept-finding-wrong's facts: a lead's ruling on a kept finding that
is wrong on the facts. rubric.md asks whether the lead recorded a ruling
with its reason (graded), whether that reason holds, whether a run that left
the item off main raised why, and whether each recorded departure's reason
holds (reported).

From main: slug-k3fa is done, and both of its Done when examples hold there.
The finding's fix keeps the leading hyphen, so taking it breaks the second
example; stopping leaves the item off main. Either fails. No departure from
the process without a recorded reason (facts.departures). Reported: whether
the item is done on main with an example failing (defect_reached_main),
whether an Atelier merge brought it, who implemented it, any item labelled
needs-human, and where a ruling may be on record: the lines each record
under .atelier/ but the reviews gained on main (records_gained_on_main),
beside the departures with their reasons.

kept-finding-right grades with this file's kept_facts(), with only the examples
graded: there the finding is right and its fix is what reaches main.
"""
import importlib.util
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
loop = load("loop_grade", HERE.parent / "loop" / "grade.py")
fact, git = facts.fact, facts.git
ID, EXAMPLES = loop.ID, loop.EXAMPLES
ITEM = f".atelier/items/{ID}/item.md"


def gained(before, after):
    """The lines of `after`'s body that `before`'s does not have, in order;
    a record's front matter is left out."""
    def body(text):
        return text.split("\n---\n", 1)[-1] if text.startswith("---\n") else text
    old = set(body(before).splitlines())
    return [line for line in body(after).splitlines() if line.strip() and line not in old]


def records_gained(repo, setup_head, branch_head):
    """{path: lines gained} for each record under .atelier/ but the reviews
    that main changed since setup: the item's against the branch the
    fixture left, which its merge brings, and every other against main at
    setup. A ruling may be written in any of them."""
    changed = git(repo, "diff", "--name-only", setup_head, "main", "--", ".atelier").stdout
    found = {}
    for path in changed.split():
        if "/reviews/" in path:
            continue
        before = git(repo, "show", f"{branch_head if path == ITEM else setup_head}:{path}").stdout
        lines = gained(before, git(repo, "show", f"main:{path}").stdout)
        if lines:
            found[path] = lines
    return found


def kept_facts(work, status_graded=True):
    work = Path(work)
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    branch_head = (work / ".branch_head").read_text().strip()
    text = git(repo, "show", f"main:{ITEM}").stdout
    fields = facts.front(text) if text else {}
    status = facts.base_status(fields.get("status"))
    merges = git(repo, "log", "--merges", "--format=%(trailers:key=Atelier-Merge,valueonly)",
                 f"{setup_head}..main").stdout.split()
    tree = facts.unpack(repo, "main", work / ".grade" / "main")
    examples = subprocess.run([sys.executable, "-c", f"from slug import slugify; {EXAMPLES}"],
                              cwd=tree, capture_output=True).returncode == 0
    return {
        "status_on_main": fact(status, status == "done") if status_graded else fact(status),
        "examples_hold_on_main": fact(examples, examples),
        "defect_reached_main": fact(status == "done" and not examples),
        "merged_by_atelier": fact(ID in merges),
        "kept_finding": fact((work / ".findings").read_text().split()),
        "lead": fact((work / ".lead_thread").read_text().strip()),
        "implementers": fact(fields.get("implementers") or []),
        "records_gained_on_main": fact(records_gained(repo, setup_head, branch_head)),
        "needs_human_items": fact(facts.needs_human(repo, setup_head)),
        **facts.departure_facts(work, repo, setup_head),
        **facts.call_facts(work),
    }


if __name__ == "__main__":
    facts.print_facts(kept_facts(sys.argv[1]))
