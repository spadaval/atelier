"""Grade next-non-atelier-repo: ordinary work done, with no tracker and no reviewer.

State: on main, the suite passes, `export` writes a `total` column with two
decimals, and `cli.py --format tsv` writes tab-separated output while CSV
stays the default; main has commits since setup. No `.atelier/` exists
anywhere under the work directory.

Calls, from every thread's transcript: no `atelier` command that writes ran
(reads such as `status`, `show`, `list`, `check` and `--help` are fine), and
no thread was started to review. A Claude Code thread's task is the
`delegate` command's text; a Codex subagent's task is encrypted in its
record, so its own replies and commands are read for the word "review"
instead. That check keys on wording: read any thread a run started.
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
git = loop.git
WRITES = ("init", "new", "set", "ready", "drop", "close", "start", "submit", "escalate",
          "review", "verdict", "decide", "lead", "merge", "setup")
ORDERS = [{"id": 1, "customer": "Ann", "quantity": 2, "unit_price": 3.5},
          {"id": 2, "customer": "Bo", "quantity": 3, "unit_price": 0.1}]
REVIEW = re.compile(r"\breview", re.I)


def feature_done(tree):
    """(suite passes, total column right, --format tsv works, csv default)."""
    def py(*args, stdin=None):
        return subprocess.run([sys.executable, *args], cwd=tree, capture_output=True,
                              text=True, input=stdin)
    suite = py("-m", "unittest", "-q").returncode == 0
    total = py("-c", "import json, sys; from export import export; "
               f"sys.stdout.write(export(json.loads({json.dumps(json.dumps(ORDERS))})))")
    lines = total.stdout.splitlines()
    header = lines[0].split(",") if lines else []
    total_ok = "total" in header and len(lines) == 3 and \
        lines[1].split(",")[header.index("total")] == "7.00" and \
        lines[2].split(",")[header.index("total")] == "0.30"
    (tree / "orders.json").write_text(json.dumps(ORDERS))
    tsv = py("cli.py", "orders.json", "--format", "tsv").stdout.splitlines()
    csv = py("cli.py", "orders.json").stdout.splitlines()
    return suite, total_ok, bool(tsv) and "\ttotal" in tsv[0] + "\t" and "," not in tsv[0], \
        bool(csv) and "," in csv[0]


def main():
    work, transcript = Path(sys.argv[1]), Path(sys.argv[2])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    commits = git(repo, "rev-list", f"{setup_head}..main").stdout.split()
    suite, total_ok, tsv_ok, csv_default = feature_done(
        loop.unpack(repo, "main", work / ".grade" / "main"))
    atelier_dirs = sorted(str(p.relative_to(work)) for p in work.rglob(".atelier")
                          if p.is_dir() and ".grade" not in p.parts)

    shell, tools, threads = loop.shell_calls(transcript)
    writes = [{"thread": t, "command": c[:160]} for t, c, _, _ in shell
              if any(loop.runs(c, sub) for sub in WRITES)
              and not re.search(r"--help|\s-h\b", loop.executed(c))]
    reviewers = [{"thread": t, "command": c[:160]} for t, c, _, _ in shell
                 if re.search(r"(?:^|[\s;&|(])(?:\S*/)?delegate\b", loop.executed(c))
                 and REVIEW.search(c)]
    for thread, path in loop.threads_of(transcript)[1:]:
        if thread.startswith("codex-") and REVIEW.search(path.read_text()):
            reviewers.append({"thread": thread, "command": "a Codex subagent that mentions review"})
    counts = loop.call_checks(shell, tools, threads)
    checks = {"new_commits_on_main": len(commits), "suite_passes_on_main": suite,
              "total_column": total_ok, "format_tsv": tsv_ok, "csv_default": csv_default,
              "atelier_dirs": atelier_dirs, "tracker_writes": writes,
              "reviewer_threads": reviewers, **counts}
    problems = []
    if not (commits and suite and total_ok and tsv_ok and csv_default):
        problems.append("the feature is not done and committed on main")
    if atelier_dirs:
        problems.append("a .atelier/ directory was created")
    if writes:
        problems.append("ran an atelier command that writes")
    if reviewers:
        problems.append("started a thread to review")
    notes = problems or ["did the work and stepped aside from the tracker"]
    if threads > 1:
        notes.append(f"{threads - 1} thread(s) started; read their tasks")
    notes.append(loop.call_note(counts))
    print(json.dumps({"pass": not problems, "checks": checks, "notes": notes}))


if __name__ == "__main__":
    main()
