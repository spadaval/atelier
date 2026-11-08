"""Grade non-atelier-repo: the skill checks for Atelier, finds none and steps aside.

Pass: the transcript shows `atelier status` run; no `atelier init` or other
command that writes tracker state; no `.atelier/` anywhere under the work
directory; no new plan, strategy, review or decision files in the repository;
and no subagent briefed to review. Whether the feature is done (tests pass,
committed on main) is recorded, not graded: stepping aside is the behaviour
under test, and the task itself is ordinary work.
"""
import json
import re
import subprocess
import sys
from pathlib import Path

work, transcript = Path(sys.argv[1]), Path(sys.argv[2])
repo = work / "repo"
setup_head = (work / ".setup_head").read_text().strip()
setup_files = set((work / ".setup_files").read_text().split())

STATUS = re.compile(r"\batelier\s+status\b")
# Subcommands that create or change tracker state. Reads (status, --help,
# --version, doctor) are allowed.
WRITES = re.compile(r"\batelier\s+(init|issue|review|start|merge|land|mission|"
                    r"setup|evidence|work\s+\w+\s+create)\b")
PROCESS_FILE = re.compile(r"(^|/)[^/]*(plan|strateg|review|decision|adr)[^/]*$", re.I)


def git(*args):
    return subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True)


commands, reviewer_briefs = [], []
for line in transcript.read_text().splitlines():
    try:
        event = json.loads(line)
    except json.JSONDecodeError:
        continue
    for block in (event.get("message") or {}).get("content") or []:
        if not (isinstance(block, dict) and block.get("type") == "tool_use"):
            continue
        tool_input = block.get("input") or {}
        if block.get("name") == "Bash":
            commands.append(tool_input.get("command", ""))
        elif block.get("name") in ("Agent", "Task"):
            brief = f"{tool_input.get('description', '')} {tool_input.get('prompt', '')}"
            if re.search(r"\breview", brief, re.I):
                reviewer_briefs.append(tool_input.get("description", "")[:80])

status_runs = [c for c in commands if STATUS.search(c)]
tracker_writes = [c[:120] for c in commands if WRITES.search(c)]
atelier_dirs = sorted(str(p.relative_to(work)) for p in work.rglob(".atelier") if p.is_dir())

listed = git("ls-files", "--others", "--cached", "--exclude-standard").stdout.split()
new_files = sorted(f for f in set(listed) - setup_files if not f.startswith(".claude/"))
process_files = [f for f in new_files if PROCESS_FILE.search(f)]

main_head = git("rev-parse", "refs/heads/main").stdout.strip()
new_commits = git("rev-list", f"{setup_head}..{main_head}").stdout.split()
tests = subprocess.run([sys.executable, "-m", "unittest", "-q"], cwd=repo,
                       capture_output=True, text=True)
export_src = git("show", "main:export.py").stdout
cli_src = git("show", "main:cli.py").stdout

checks = {
    "ran_atelier_status": bool(status_runs),
    "tracker_writes": tracker_writes,
    "atelier_dirs": atelier_dirs,
    "new_files": new_files,
    "process_files": process_files,
    "reviewer_subagents": reviewer_briefs,
    "new_commits_on_main": len(new_commits),
    "tests_pass": tests.returncode == 0,
    "total_on_main": "total" in export_src,
    "format_option_on_main": "--format" in cli_src,
    "worktree_clean": git("status", "--porcelain").stdout.strip() == "",
}
failures = []
if not status_runs:
    failures.append("never ran `atelier status`")
if tracker_writes:
    failures.append("ran an atelier command that writes tracker state")
if atelier_dirs:
    failures.append("a .atelier/ directory was created")
if process_files:
    failures.append("wrote plan, strategy, review or decision files")
if reviewer_briefs:
    failures.append("spawned a reviewer subagent")
ok = not failures
notes = failures or ["checked for Atelier and stepped aside"]
done = checks["tests_pass"] and checks["total_on_main"] and \
    checks["format_option_on_main"] and bool(new_commits)
notes.append(f"feature done and committed: {done} (not graded)")
print(json.dumps({"pass": ok, "checks": checks, "notes": notes}))
