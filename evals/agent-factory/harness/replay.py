#!/usr/bin/env python3
"""Replay a change from this repository's history as a review case.

`docs/architecture/quality/scorecard-baseline/defects.md` traces defects in
atelier that were found after the merge that caused them. A replay case
reviews the causing change again, on today's binary, and asks whether the
review finds the defect. Its directory holds the usual scenario files and a
case.toml, which names:

- `row`: the defect's row in defects.md's Atelier traces, and `defect_item`,
  the bug item that recorded it;
- `item`, the item that made the change, `base` and `change`, the commits
  before and at it, and `fix`, the commit that fixed the defect;
- `defect`: the defect in a sentence, which only the judge reads;
- `id` and `title`: the replayed item's new ID and its title; `goal`,
  `done_when` (a list), `context` and `report`: the item's Goal, Done when,
  Context and Notes, and its Close Reason, as they stood at the change.

build() makes the fixture: this repository's tree at `base`, read with `git
archive` from the checkout running the harness, which needs its history
(CI's evals job fetches it all). The old tracker's files (DROPPED) are left
out, and so are AGENTS.md's sections on the old tool's process
(PROCESS_SECTIONS), since today's binary tracks the fixture with new
records: one item, `id`, with the Goal and Done when it had. The change,
`git diff base change` outside DROPPED, is committed on the item's branch
with its own message, and the item is submitted for review by its
implementer. The fixture has no remote, its history ends at the change, and
nothing of case.toml but the item's own text enters it.

A replay run is confined (harness/confine.py): it cannot read this host's
other checkouts or the case files, and cannot fetch this repository. As a
backstop, leak() finds a run whose transcript shows the later fix (its
commit, its subject, or the bug item's ID) or a web search, and the harness
makes that run invalid. judge_context() gives the judge the defect, the
change's diff and the fix's diff; the reviewer is told nothing.
"""
import json
import os
import re
import subprocess
import sys
import tomllib
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
# The old tracker's records, its skill and its hooks, left out of a fixture.
DROPPED = (".atelier", ".agents", ".claude", ".codex")
# AGENTS.md's sections on the old tool's process, left out of a fixture.
PROCESS_SECTIONS = ("Current Mission", "Process Sources", "Merge Policy")
IMPLEMENTER = "thread-5e1f0a7c"
IDENTITY = {"GIT_AUTHOR_NAME": "Dana Dev", "GIT_AUTHOR_EMAIL": "dana@example.com",
            "GIT_COMMITTER_NAME": "Dana Dev", "GIT_COMMITTER_EMAIL": "dana@example.com"}
FIELDS = ("row", "defect_item", "item", "base", "change", "fix", "defect", "id", "title",
          "goal", "done_when", "report")
# The most of a diff the judge is given, each; the rest is cut and it is told so.
MAX_DIFF = 60_000


def load(case: Path) -> dict:
    """The case's case.toml, with every field FIELDS names."""
    data = tomllib.loads((Path(case) / "case.toml").read_text())
    missing = [f for f in FIELDS if f not in data]
    if missing:
        raise ValueError(f"{case}/case.toml lacks {', '.join(missing)}")
    return data


def git(*args, cwd=REPO, env=None, check=True, **kw) -> str:
    out = subprocess.run(["git", "-C", str(cwd), *args], capture_output=True, text=True,
                         env=env, **kw)
    if check and out.returncode != 0:
        raise RuntimeError(f"git {' '.join(args)}: {out.stderr.strip()}")
    return out.stdout


def has_history(case: dict):
    """Why this checkout cannot build the case's fixture, or None."""
    for name in ("base", "change", "fix"):
        if subprocess.run(["git", "-C", str(REPO), "cat-file", "-e", f"{case[name]}^{{commit}}"],
                          capture_output=True).returncode != 0:
            return (f"this checkout lacks {name} commit {case[name][:9]}: fetch the "
                    "repository's whole history (git fetch --unshallow)")
    return None


def strip_process(text: str) -> str:
    """AGENTS.md without its PROCESS_SECTIONS."""
    parts = re.split(r"(?m)^(?=## )", text)
    return "".join(p for p in parts
                   if not any(p.startswith(f"## {name}\n") for name in PROCESS_SECTIONS))


def item_record(case: dict) -> str:
    """The replayed item's record, ready, with the Goal, Done when and
    Context it had."""
    lines = "\n".join("- [ ] " + d.strip().replace("\n", "\n  ") for d in case["done_when"])
    body = f"## Goal\n\n{case['goal'].strip()}\n\n## Done when\n\n{lines}\n"
    if case.get("context", "").strip():
        body += f"\n## Context\n\n{case['context'].strip()}\n"
    return ("---\nschema: atelier.item\nschema_version: 3\n"
            f"id: {case['id']}\ntitle: {json.dumps(case['title'])}\nstatus: ready\n"
            "mission: null\nlabels: []\npriority: medium\nblocked_by: []\nimplementers: []\n"
            "ready:\n  by: dana\n  at: 2026-11-09T09:00:00Z\ntrivial: null\ncaused_by: null\n"
            f"created_at: 2026-11-09T09:00:00Z\n---\n\n{body}")


def build(case_dir: Path, work: Path, atelier: Path) -> None:
    """Make the case's fixture in `work`: work/repo, on the item's branch in
    review, and work/bin/atelier, a copy of `atelier`."""
    case = load(case_dir)
    missing = has_history(case)
    if missing:
        raise SystemExit(missing)
    work = Path(work)
    repo, bin_dir = work / "repo", work / "bin"
    bin_dir.mkdir(parents=True, exist_ok=True)
    (bin_dir / "atelier").write_bytes(Path(atelier).read_bytes())
    (bin_dir / "atelier").chmod(0o755)
    repo.mkdir(parents=True)
    archive = subprocess.run(["git", "-C", str(REPO), "archive", case["base"], "--", ".",
                              *[f":(exclude){d}" for d in DROPPED]],
                             capture_output=True, check=True).stdout
    subprocess.run(["tar", "-x", "-C", str(repo)], input=archive, check=True)
    agents = repo / "AGENTS.md"
    if agents.is_file():
        agents.write_text(strip_process(agents.read_text()))

    env = {k: v for k, v in os.environ.items() if k not in (
        "ATELIER_SKIP", "CLAUDE_CODE_SESSION_ID", "CODEX_SESSION_ID", "CODEX_THREAD_ID",
        "T3CODE_THREAD_ID", "GIT_DIR", "GIT_WORK_TREE")}
    env.update(IDENTITY, PATH=f"{bin_dir}:{env['PATH']}", ATELIER_NO_UPDATE_CHECK="1")

    def sh(*cmd, thread=None, **kw):
        return subprocess.run(cmd, cwd=repo, check=True, capture_output=True, text=True,
                              env={**env, **({"T3CODE_THREAD_ID": thread} if thread else {})},
                              **kw).stdout

    sh("git", "init", "-q", "-b", "main")
    sh("git", "config", "user.name", "Eval Agent")
    sh("git", "config", "user.email", "agent@example.com")
    (repo / ".atelier" / "items" / case["id"]).mkdir(parents=True)
    (repo / ".atelier" / "config.toml").write_text(
        'schema_version = 3\nslug = "atelier"\nbase_branch = "main"\n')
    (repo / ".atelier" / "items" / case["id"] / "item.md").write_text(item_record(case))
    sh("git", "add", "-A")
    sh("git", "commit", "-qm", "Track work in Atelier")
    (work / ".main_head").write_text(sh("git", "rev-parse", "HEAD"))
    (work / ".setup_head").write_text(sh("git", "rev-parse", "HEAD"))

    sh("atelier", "start", case["id"], thread=IMPLEMENTER)
    sh("git", "switch", "-q", case["id"])
    patch = git("diff", "--binary", case["base"], case["change"], "--", ".",
                *[f":(exclude){d}" for d in DROPPED])
    sh("git", "apply", "--index", "-", input=patch)
    message = git("log", "-1", "--format=%B", case["change"])
    sh("git", "commit", "-q", "-F", "-", input=message)
    record = repo / ".atelier" / "items" / case["id"] / "item.md"
    record.write_text(record.read_text().replace("- [ ]", "- [x]").rstrip("\n")
                      + f"\n\n## Report\n\n{case['report'].strip()}\n")
    sh("git", "commit", "-qam", f"Report on {case['id']}")
    sh("atelier", "submit", thread=IMPLEMENTER)
    (work / ".branch_head").write_text(sh("git", "rev-parse", "HEAD"))


def markers(case: dict) -> list:
    """Strings a run sees only if it reached the later fix: the fix
    commit's abbreviated name and subject, and the bug item's ID."""
    subject = git("log", "-1", "--format=%s", case["fix"]).strip()
    return [case["fix"][:7], subject, case["defect_item"]]


def web_use(event: dict) -> bool:
    """Whether a transcript event is the agent's own web search or fetch."""
    item = event.get("item") or {}
    if "web_search" in str(item.get("type") or ""):
        return True
    for block in (event.get("message") or {}).get("content") or []:
        if isinstance(block, dict) and block.get("type") in ("tool_use", "server_tool_use") \
                and block.get("name") in ("WebSearch", "WebFetch", "web_search", "web_fetch"):
            return True
    return False


def leak(case_dir: Path, transcripts: list):
    """Why a run's transcripts (its first agent's and its threads') show it
    reached the later fix, or None: a marker in any of them, or a web search
    or fetch of its own."""
    case = load(case_dir)
    found = markers(case)
    for path in transcripts:
        path = Path(path)
        if not path.is_file():
            continue
        text = path.read_text(errors="replace")
        for marker in found:
            if marker in text:
                return f"{path.name} shows {marker!r}, which only the later fix holds"
        for line in text.splitlines():
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue
            if isinstance(event, dict) and web_use(event):
                return f"{path.name} shows a web search or fetch of the agent's own"
    return None


def cut(text: str) -> str:
    return text if len(text) <= MAX_DIFF else \
        text[:MAX_DIFF] + f"\n[... cut: the diff is {len(text)} characters]\n"


def judge_context(case_dir: Path) -> str:
    """What the judge is told of a replay case besides its rubric: the known
    defect, the change under review and the fix's diff, both outside the old
    tracker's files."""
    case = load(case_dir)
    outside = ["--", ".", *[f":(exclude){d}" for d in DROPPED]]
    change = git("diff", case["base"], case["change"], *outside)
    fix = git("show", "--format=%s%n%n%b", case["fix"], *outside)
    return (f"\n\n# The known defect\n\n{case['defect'].strip()}\n\n"
            f"It was found after the change merged (defects.md, Atelier row {case['row']}) "
            f"and fixed by a later commit, {case['fix'][:9]}.\n\n"
            f"# The change under review\n\n```diff\n{cut(change)}```\n\n"
            f"# The later fix\n\n```diff\n{cut(fix)}```\n")


def grade(case_dir: Path, work: Path) -> None:
    """Print a replay run's facts. The grade rests on a round-1 review by a
    thread that never implemented the item, and no rule passed with
    ATELIER_SKIP; whether a finding names the defect is the rubric's. The
    rest is reported: the reviews, their fix-now findings, files changed
    outside the reviews, whether main moved, and the `atelier` calls."""
    sys.path.insert(0, str(Path(__file__).resolve().parent))
    import facts
    case, work = load(case_dir), Path(work)
    repo = work / "repo"
    branch_head = (work / ".branch_head").read_text().strip()
    rounds = facts.round_reviews(repo, case["id"], branch_head)
    reviewed = any(r["independent"] for r in rounds)
    fix_now = [f for r in rounds for f in r["findings"]
               if (f["recommendation"] or "").lower() == "fix now"]
    other, main_moved = facts.outside_review(work, repo, case["id"], branch_head)
    skips = facts.skips(work, repo, (work / ".setup_head").read_text().strip())
    facts.print_facts({
        "round_1_reviews": facts.fact(rounds),
        "a_round_1_review_by_another_thread": facts.fact(reviewed, reviewed),
        "fix_now_findings": facts.fact(len(fix_now)),
        "other_files_changed": facts.fact(other),
        "main_moved": facts.fact(main_moved),
        "skips": facts.fact(skips, not skips),
        **facts.call_facts(work),
    })


def cargo_fetch(repo: Path, cargo_home: Path) -> None:
    """Download the fixture's crates into the cache a confined run builds
    from offline. It builds nothing."""
    cargo_home.mkdir(parents=True, exist_ok=True)
    env = {**os.environ, "CARGO_HOME": str(cargo_home)}
    subprocess.run(["cargo", "fetch", "--locked"], cwd=repo, env=env, check=True,
                   capture_output=True)


def main() -> int:
    if len(sys.argv) == 4 and sys.argv[1] == "setup":
        build(Path(sys.argv[2]), Path(sys.argv[3]),
              Path(os.environ.get("EVAL_ATELIER") or sys.exit("set EVAL_ATELIER")))
        return 0
    if len(sys.argv) == 4 and sys.argv[1] == "grade":
        grade(Path(sys.argv[2]), Path(sys.argv[3]))
        return 0
    sys.exit("usage: replay.py setup|grade <case dir> <work dir>")


if __name__ == "__main__":
    sys.exit(main())
