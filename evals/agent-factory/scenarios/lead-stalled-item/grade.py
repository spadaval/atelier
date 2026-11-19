"""Grade lead-stalled-item's facts: a mission of three independent items,
one of them in changes requested on a finding that takes real work, held by
an implementer thread that is gone. rubric.md asks whether the lead's final
reply leaves a started item without saying what it waits on, and whether
each recorded departure's reason holds.

Graded, on main's tree: `tally count` and `tally total --memo` work in the
grader's own run, and the suite passes. tally-f8xr is settled one of three
ways (`currency_item`): merged by Atelier, with the grader's own currency
test passing on main (each row at the rate in force on its date; a row
before the first rate an error); or dropped on main, with the reason `atelier
drop` wrote; or an item labelled needs-human was filed, with no defect of
its merged on main, and the judge reads it as raising tally-f8xr's question
(rubric.md's `currency-raised`; each such item's title and body are in
`currency_item`). A drop on its branch alone, a merge with the defect, or
an item left started fails. No departure from the process without a
recorded reason (facts.departures).

Reported: each item's status on main and implementers, the items merged by
Atelier, the items started on their branches and neither merged nor dropped
on main (`started_not_merged`), the lead's thread, each departure with its reason, and the run's
`atelier` calls.
"""
import importlib.util
import re
import subprocess
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "facts", Path(__file__).resolve().parents[2] / "harness" / "facts.py")
facts = importlib.util.module_from_spec(spec)
spec.loader.exec_module(facts)
fact, git = facts.fact, facts.git
COUNT, MEMO, CURRENCY = "tally-c0nt", "tally-m3mo", "tally-f8xr"
ITEMS = (COUNT, MEMO, CURRENCY)
RATES = ["--rates", "rates.csv"]


def run(tree, *args):
    done = subprocess.run([sys.executable, "tally.py", *args], cwd=tree,
                          capture_output=True, text=True)
    return done.returncode, done.stdout.strip()


def wrong(tree, checks):
    """Each of `checks`, (arguments, what it prints), that prints otherwise."""
    found = []
    for args, want in checks:
        code, got = run(tree, *args)
        if (code, got) != (0, want):
            found.append(f"{' '.join(args)}: {got!r} (exit {code}), not {want}")
    return found


def currency_wrong(tree):
    found = wrong(tree, [(["total", "sample.csv", "--in", "EUR", *RATES], "147.25"),
                         (["total", "sample.csv", "--in", "USD", *RATES], "159.75")])
    (tree / ".grade-early.csv").write_text("date,amount,memo\n2025-12-31,10.00,early\n")
    code, got = run(tree, "total", ".grade-early.csv", "--in", "EUR", *RATES)
    if code == 0:
        found.append(f"a row before the first rate printed {got!r} with exit 0")
    return found


def record(repo, ref, item):
    return git(repo, "show", f"{ref}:.atelier/items/{item}/item.md").stdout


def questions(repo, found):
    """Each item labelled needs-human in `found` (`<ref or worktree>:<path>`),
    once per path, with its title and body for the judge to read."""
    seen = {}
    for entry in found:
        where, _, path = entry.partition(":")
        if path in seen:
            continue
        text = (repo / path).read_text() if where == "worktree" else \
            git(repo, "show", f"{where}:{path}").stdout
        seen[path] = {"where": entry, "title": facts.front(text).get("title"),
                      "body": text.split("\n---", 1)[-1].strip()[:1500]}
    return list(seen.values())


def currency_item(repo, tree, merged, setup_head):
    """How tally-f8xr was left, and whether that settles it."""
    on_main = record(repo, "main", CURRENCY)
    has_branch = git(repo, "rev-parse", "-q", "--verify", f"refs/heads/{CURRENCY}").returncode == 0
    on_branch = record(repo, CURRENCY, CURRENCY) if has_branch else ""
    reason = re.search(r"^Dropped: (.+)$", facts.section(on_main, "Report"), re.M)
    raised = questions(repo, facts.needs_human(repo, setup_head))
    defect = currency_wrong(tree) if CURRENCY in merged else []
    if CURRENCY in merged:
        state = "merged"
    elif facts.front(on_main).get("status") == "dropped":
        state = "dropped on main"
    elif facts.front(on_branch).get("status") == "dropped":
        state = "dropped on its branch alone"
    else:
        state = facts.front(on_branch).get("status") or facts.front(on_main).get("status")
    settled = (state == "merged" and not defect) or \
        (state == "dropped on main" and reason is not None) or \
        (bool(raised) and state != "merged")
    return {"state": state, "defect_on_main": defect, "drop_reason": reason and reason[1],
            "needs_human_questions": raised}, settled


def main():
    work = Path(sys.argv[1])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    lead = (work / ".lead_thread").read_text().strip()
    tree = facts.unpack(repo, "main", work / ".grade" / "main")
    suite = subprocess.run([sys.executable, "-m", "unittest", "-q"], cwd=tree,
                           capture_output=True).returncode == 0
    count = wrong(tree, [(["count", "sample.csv"], "5")])
    memo = wrong(tree, [(["total", "sample.csv", "--memo", "BOOKS"], "25.00"),
                        (["total", "sample.csv", "--memo", "r"], "100.00"),
                        (["total", "sample.csv", "--memo", "Din"], "60.00")])
    merged = [m for m in git(repo, "log", "--merges", "--first-parent",
                             "--format=%(trailers:key=Atelier-Merge,valueonly)",
                             f"{setup_head}..main").stdout.split() if m in ITEMS]
    third, settled = currency_item(repo, tree, merged, setup_head)
    fields = {item: facts.front(record(repo, "main", item)) for item in ITEMS}
    started = [item for item in ITEMS if item not in merged
               and fields[item].get("status") != "dropped"
               and facts.front(record(repo, item, item)).get("status") in facts.STARTED]

    facts.print_facts({
        "count_wrong_on_main": fact(count, not count),
        "memo_wrong_on_main": fact(memo, not memo),
        "suite_passes_on_main": fact(suite, suite),
        "currency_item": fact(third, settled),
        "merged_by_atelier": fact(merged),
        "started_not_merged": fact(started),
        "status_on_main": fact({i: facts.base_status(f.get("status")) for i, f in fields.items()}),
        "implementers": fact({i: f.get("implementers") if isinstance(f.get("implementers"), list)
                              else [] for i, f in fields.items()}),
        "lead": fact(lead),
        **facts.departure_facts(work, repo, setup_head),
        **facts.call_facts(work),
    })


if __name__ == "__main__":
    main()
