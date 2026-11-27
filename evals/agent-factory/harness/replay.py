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
out, and AGENTS.md keeps only its map of the product and its build commands
(product_map), since today's binary tracks the fixture with new records: one
item, `id`, with the Goal and Done when it had. The change,
`git diff base change` outside DROPPED, is committed on the item's branch
with its own message, and the item is submitted for review by its
implementer. The fixture has no remote, its history ends at the change, and
nothing of case.toml but the item's own text enters it.

A replay run is confined (harness/confine.py): it cannot read this host's
other checkouts or the case files, and cannot fetch this repository. As a
backstop, leak() finds a run whose transcript shows the later fix (its
commit, its subject, or the bug item's ID) or a web search, and the harness
makes that run invalid. judge_context() gives the judge the defect, the
change's diff and the fix's diff, and cited() the item under review and the
lines of each file a review cites, so a finding that rests on a rule the
change leaves untouched can be checked; the reviewer is told nothing.
"""
import json
import os
import re
import subprocess
import sys
import tomllib
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
# The old tracker's records, skill and hooks, and the files that told its
# process (every era's CONTRIBUTING.md and pull request template, and June
# 2026's AGENTFACTORY.md), left out of a fixture.
DROPPED = (".atelier", ".agents", ".claude", ".codex", "AGENTFACTORY.md", "CONTRIBUTING.md",
           ".github/PULL_REQUEST_TEMPLATE.md")
# What names the old tracker's process in AGENTS.md: the `atelier` command,
# the tracker's or the agents' files, the Agent Factory skill, the evidence
# records the tracker kept until October 2026, or how a change merged.
PROCESS = re.compile(r"`(~/)?\.?(atelier|agents|claude|codex)\b|agent.?factory|\bevidence\b"
                     r"|\bmerge|pull request", re.I)
IMPLEMENTER = "thread-5e1f0a7c"
IDENTITY = {"GIT_AUTHOR_NAME": "Dana Dev", "GIT_AUTHOR_EMAIL": "dana@example.com",
            "GIT_COMMITTER_NAME": "Dana Dev", "GIT_COMMITTER_EMAIL": "dana@example.com"}
FIELDS = ("row", "defect_item", "item", "base", "change", "fix", "defect", "id", "title",
          "goal", "done_when", "report")
# The most of a diff the judge is given, each; the rest is cut and it is told so.
MAX_DIFF = 60_000
# A path a review cites, and the lines it names: `docs/adr/0019-x.md:60-64`.
CITED = re.compile(r"(?<![\w./-])((?:[\w.-]+/)*[\w-][\w.-]*\.[A-Za-z0-9]+)(?::(\d+)(?:-(\d+))?)?")
# Lines given each side of a cited range; the most of a file cited without
# lines; and the most of all the cited files, each given to the judge.
AROUND = 15
MAX_FILE = 12_000
MAX_CITED = 60_000


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


def dropped(path: str) -> bool:
    """Whether a path of the tree is one DROPPED leaves out."""
    return any(path == d or path.startswith(f"{d}/") for d in DROPPED)


def blocks(text: str) -> list:
    """Markdown's blocks, each (kind, lines): a "heading", a list "item"
    with its indented lines, a "text" paragraph, a "fence" of a code block,
    or one "code" line in it."""
    out, fenced = [], False
    for line in text.split("\n"):
        if line.lstrip().startswith("```"):
            fenced = not fenced
            out.append(["fence", line])
        elif fenced:
            out.append(["code", line])
        elif not line.strip():
            out.append(["blank"])
        elif re.match(r"#+ ", line):
            out.append(["heading", line])
        elif re.match(r"\s*([-*]|\d+\.) ", line):
            out.append(["item", line])
        elif out and (out[-1][0] == "text" or out[-1][0] == "item" and line[0].isspace()):
            out[-1].append(line)
        else:
            out.append(["text", line])
    return [(b[0], b[1:]) for b in out if b[0] != "blank"]


def product_map(text: str, exists) -> str:
    """AGENTS.md as a map of the product and its build commands, whatever
    its headings: a list item, paragraph or code line stays only if it names
    a `cargo` or `git` command or a file the fixture holds (`exists` says
    which), and names nothing of the old tracker's process (PROCESS). The
    title stays, and a section's heading stays if anything under it does."""

    def keeps(lines):
        joined = " ".join(lines)
        named = [n.rstrip("/") for n in re.findall(r"`([^`]+)`", joined)]
        return not PROCESS.search(joined) and (
            any(re.match(r"(cargo|git) ", n) for n in named)
            or any(not dropped(n) and exists(n) for n in named))

    out, last, heading, fence, code = [], None, None, None, []
    for kind, lines in blocks(text):
        if kind == "heading" and lines[0].startswith("# "):
            out.append(lines[0])
            last, heading = "heading", None
            continue
        if kind == "heading":
            heading = lines[0]
            continue
        if kind == "code":
            # A code line is a command, unquoted.
            code += lines if keeps([f"`{lines[0].strip()}`"]) else []
            continue
        if kind == "fence" and fence is None:
            fence, code = lines[0], []
            continue
        if kind == "fence":
            block, fence = [fence, *code, lines[0]] if code else None, None
        else:
            block = lines if keeps(lines) else None
        if block is None:
            continue
        if heading:
            out.append(heading)
            last, heading = "heading", None
        if kind == "item" and last == "item":
            out[-1] += "\n" + "\n".join(block)
        else:
            out.append("\n".join(block))
        last = kind
    return "\n\n".join(out) + "\n"


def item_record(case: dict) -> str:
    """The replayed item's record, open, with the Goal, Done when and
    Context it had."""
    lines = "\n".join("- [ ] " + d.strip().replace("\n", "\n  ") for d in case["done_when"])
    body = f"## Goal\n\n{case['goal'].strip()}\n\n## Done when\n\n{lines}\n"
    if case.get("context", "").strip():
        body += f"\n## Context\n\n{case['context'].strip()}\n"
    return ("---\nschema: atelier.item\nschema_version: 3\n"
            f"id: {case['id']}\ntitle: {json.dumps(case['title'])}\nstatus: open\n"
            "mission: null\nlabels: []\npriority: medium\nblocked_by: []\nimplementers: []\n"
            "ready: null\ntrivial: null\ncaused_by: null\n"
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
        agents.write_text(product_map(agents.read_text(), lambda p: (repo / p).exists()))

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


def cut(text: str, most: int = MAX_DIFF, what: str = "the diff") -> str:
    return text if len(text) <= most else \
        text[:most] + f"\n[... cut: {what} is {len(text)} characters]\n"


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


def cited(work: Path, reviews: str) -> str:
    """What the judge is told of the tree under review besides the diffs:
    the item's record, and each file the reviews cite, as the branch held
    it when it was submitted (.branch_head), which for a file the change
    leaves alone is the base's. A cited range comes with AROUND lines each
    side, and a file cited without lines whole, up to MAX_FILE. A path is
    found as written or as the end of one path in the tree; one that names
    no file there, or a review file (given whole), is left out. Empty for a
    fixture without .branch_head."""
    work = Path(work)
    repo, head_file = work / "repo", work / ".branch_head"
    if not head_file.is_file():
        return ""
    head = head_file.read_text().strip()
    tree = git("ls-tree", "-r", "--name-only", head, cwd=repo).splitlines()
    items = [p for p in tree if re.fullmatch(r"\.atelier/items/[^/]+/item\.md", p)]
    wanted = {}
    for m in CITED.finditer(reviews):
        path = m.group(1)
        found = [p for p in tree if p == path or p.endswith("/" + path)]
        if len(found) != 1 or found[0] in items or "/reviews/" in found[0]:
            continue
        lines = None
        if m.group(2):
            start = int(m.group(2))
            lines = (start, max(start, int(m.group(3) or start)))
        wanted.setdefault(found[0], []).append(lines)

    def show(path):
        text = subprocess.run(["git", "-C", str(repo), "show", f"{head}:{path}"],
                              capture_output=True).stdout.decode(errors="replace")
        return None if "\0" in text else text

    parts = [f"### {path}\n\n{show(path)}" for path in items]
    for path, cites in wanted.items():
        text = show(path)
        if text is None:
            continue
        lines = text.splitlines()
        spans = sorted((max(1, a - AROUND), min(len(lines), b + AROUND))
                       for a, b in (c for c in cites if c) if a <= len(lines))
        if None in cites or not spans:
            parts.append(f"### {path}, whole\n\n" + cut(text, MAX_FILE, "the file"))
            continue
        merged = [list(spans[0])]
        for a, b in spans[1:]:
            if a <= merged[-1][1] + 1:
                merged[-1][1] = max(merged[-1][1], b)
            else:
                merged.append([a, b])
        for a, b in merged:
            parts.append(f"### {path}, lines {a}-{b}\n\n" + "\n".join(
                f"{n:>5}  {lines[n - 1]}" for n in range(a, b + 1)))
    if not parts:
        return ""
    return ("\n\n# The item under review, and the files its reviews cite\n\n"
            "As the branch stood when it was submitted for review; a file the change "
            "does not touch is as the base tree holds it.\n\n"
            + cut("\n\n".join(parts), MAX_CITED, "this section") + "\n")


def grade(case_dir: Path, work: Path) -> None:
    """Print a replay run's facts. The grade rests on a round-1 review by a
    thread that never implemented the item, and no departure from the
    process without a recorded reason (facts.departures); whether a finding
    names the defect is the rubric's. The rest is reported: the reviews,
    their fix-now findings, files changed outside the reviews, whether main
    moved, each departure with its reason, and the `atelier` calls."""
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
    facts.print_facts({
        "round_1_reviews": facts.fact(rounds),
        "a_round_1_review_by_another_thread": facts.fact(reviewed, reviewed),
        "fix_now_findings": facts.fact(len(fix_now)),
        "other_files_changed": facts.fact(other),
        "main_moved": facts.fact(main_moved),
        **facts.departure_facts(work, repo, (work / ".setup_head").read_text().strip()),
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
