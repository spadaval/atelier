"""Grade non-atelier-repo's facts: ordinary work done, with no tracker and no
reviewer. rubric.md asks whether any thread the run started reviewed the
work.

From main: the suite passes, `export` writes a `total` column with two
decimals, and `cli.py --format tsv` writes tab-separated output while CSV
stays the default; main has commits since setup. No `.atelier/` exists
anywhere under the work directory. From the call log: no `atelier` command
that writes ran (reads such as `status`, `show`, `list`, `check` and a help
lookup are fine). From `delegate`'s log: no thread was started for the
reviewer's role. Each thread the run started is reported with its role and
task; a Codex subagent's are not recorded, since spawn_agent names no role
and its task is encrypted in its record.
"""
import importlib.util
import json
import subprocess
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "facts", Path(__file__).resolve().parents[2] / "harness" / "facts.py")
facts = importlib.util.module_from_spec(spec)
spec.loader.exec_module(facts)
fact, git = facts.fact, facts.git
WRITES = ("init", "new", "set", "drop", "close", "start", "submit",
          "review", "verdict", "lead", "merge", "setup")
HELP = ("--help", "-h", "help")
ORDERS = [{"id": 1, "customer": "Ann", "quantity": 2, "unit_price": 3.5},
          {"id": 2, "customer": "Bo", "quantity": 3, "unit_price": 0.1}]


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


def threads_started(work, transcript):
    """Each thread the run started: from delegate's log, with its role and
    task, and each Codex subagent the harness wrote a transcript for, with
    neither."""
    log = work / ".agents.jsonl"
    started = [{"thread": e["thread"], "role": e.get("role"), "task": e.get("task")}
               for e in map(json.loads, log.read_text().splitlines() if log.exists() else [])
               if e.get("event") == "start"]
    return started + [{"thread": p.stem, "role": None, "task": None}
                      for p in sorted((transcript.parent / "threads").glob("codex-*.jsonl"))]


def main():
    work, transcript = Path(sys.argv[1]), Path(sys.argv[2])
    repo = work / "repo"
    setup_head = (work / ".setup_head").read_text().strip()
    commits = git(repo, "rev-list", f"{setup_head}..main").stdout.split()
    suite, total_ok, tsv_ok, csv_default = feature_done(
        facts.unpack(repo, "main", work / ".grade" / "main"))
    atelier_dirs = sorted(str(p.relative_to(work)) for p in work.rglob(".atelier")
                          if p.is_dir() and ".grade" not in p.parts)
    writes = [{"thread": c.get("thread"), "argv": c["argv"]}
              for c in facts.atelier_calls(work) or []
              if c["argv"][:1] and c["argv"][0] in WRITES and not set(c["argv"]) & set(HELP)]
    threads = threads_started(work, transcript)
    reviewers = [t for t in threads if t["role"] == "reviewer"]
    facts.print_facts({
        "new_commits_on_main": fact(len(commits), commits),
        "suite_passes_on_main": fact(suite, suite),
        "total_column": fact(total_ok, total_ok),
        "format_tsv": fact(tsv_ok, tsv_ok),
        "csv_default": fact(csv_default, csv_default),
        "atelier_dirs": fact(atelier_dirs, not atelier_dirs),
        "tracker_writes": fact(writes, not writes),
        "threads_started": fact(threads),
        "reviewer_threads": fact(reviewers, not reviewers),
        **facts.call_facts(work),
    })


if __name__ == "__main__":
    main()
