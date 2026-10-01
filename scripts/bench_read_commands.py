#!/usr/bin/env python3
"""Time Atelier read commands on a generated 1,000-issue repository.

Proves strategy criterion S4: read commands finish in under 100 ms with a
release build and no cache. Build first with `cargo build --release`.

    scripts/bench_read_commands.py [--binary target/release/atelier] [--runs 10]

The fixture has missions advancing epics, epics with children, blocker
chains, mixed statuses, labels, and evidence attached to done work. Records
are written as Markdown directly, then `atelier check` must pass before any
timing starts. Exits non-zero if any command's median or max exceeds the
budget.
"""

import argparse
import os
import random
import re
import shutil
import statistics
import subprocess
import sys
import tempfile
import time
from datetime import datetime, timedelta, timezone
from pathlib import Path

BUDGET_MS = 100.0
ROOT = Path(__file__).resolve().parent.parent


def ts(base, minutes):
    return (base + timedelta(minutes=minutes)).isoformat()


def ids(prefix, count):
    return [f"atelier-{prefix}{n:03d}" for n in range(count)]


def issue_markdown(issue):
    rel = issue["rel"]

    def bucket(name, items, extra=None):
        if not items:
            return f"  {name}: []\n"
        out = f"  {name}:\n"
        for item in items:
            out += f'  - kind: "issue"\n    id: "{item}"\n'
            if extra:
                out += f'    {extra[0]}: "{extra[1]}"\n'
        return out

    labels = "".join(f'- "{label}"\n' for label in issue["labels"])
    closed = f'closed_at: "{issue["updated"]}"\n' if issue["status"] == "done" else ""
    goal = f"{issue['title']} covers one bounded slice of the benchmark mission."
    proof = "The focused regression test fails without the change and passes after it."
    if issue["type"] == "mission":
        body = f"## Outcome\n\n{goal}\n\n## Success criteria\n\n{proof}\n"
    elif issue["type"] == "epic":
        body = f"## Outcome\n\n{goal}\n\n## Proof\n\n{proof}\n"
    else:
        body = f"## Goal\n\n{goal}\n\n## Done when\n\n{proof}\n\n## Scope\n\nBenchmark fixture.\n\n## Context\n\nGenerated record fixture.\n"
    return (
        "---\n"
        f'{closed}created_at: "{issue["created"]}"\n'
        f'id: "{issue["id"]}"\n'
        f'issue_type: "{issue["type"]}"\n'
        + ("labels:\n" + labels if labels else "labels: []\n")
        + f'priority: "{issue["priority"]}"\n'
        "relationships:\n"
        + bucket("blocks", rel["blocks"])
        + bucket("children", rel["children"])
        + "  attachments: []\n"
        + bucket("relates", rel["advances"], ("type", "advances"))
        + 'schema: "atelier.issue"\n'
        "schema_version: 2\n"
        f'status: "{issue["status"]}"\n'
        f'title: "{issue["title"]}"\n'
        f'updated_at: "{issue["updated"]}"\n'
        "---\n\n"
        + body
    )


def evidence_markdown(evidence_id, target, created, commit):
    return (
        "---\n"
        f'created_at: "{created}"\n'
        f'id: "{evidence_id}"\n'
        'evidence_type: "test"\n'
        f'commit: "{commit}"\n'
        'result: "pass"\n'
        f'captured_at: "{created}"\n'
        'command: "cargo nextest run"\n'
        'exit_status: "0"\n'
        "success: true\n"
        "target:\n"
        '  kind: "issue"\n'
        f'  id: "{target}"\n'
        '  role: "validates"\n'
        "relationships:\n"
        "  blocks: []\n"
        "  children: []\n"
        "  attachments:\n"
        '  - kind: "issue"\n'
        f'    id: "{target}"\n'
        '    role: "validates"\n'
        "  relates: []\n"
        'schema: "atelier.evidence"\n'
        "schema_version: 2\n"
        'status: "pass"\n'
        'title: "cargo nextest run"\n'
        f'updated_at: "{created}"\n'
        "---\n\n"
        "## Summary\n\ncargo nextest run\n\n"
        "## Command\n\n```console\ncargo nextest run\n```\n\n"
        "Exit status: 0\n\n"
        "## Stdout\n\nBytes: 31\nTruncated: no\n\n```text\nSummary: 461 tests run: 461 passed\n```\n\n"
        "## Stderr\n\nBytes: 0\nTruncated: no\n\n```text\n```\n"
    )


def generate(repo, total_issues):
    rng = random.Random(19)
    base = datetime(2026, 9, 1, tzinfo=timezone.utc)
    missions = ids("m", 10)
    epics = ids("e", 60)
    remaining = total_issues - len(missions) - len(epics)
    standalone = ids("s", 90)
    children_per_epic = (remaining - len(standalone)) // len(epics)
    tasks = ids("t", children_per_epic * len(epics))
    issues = {}

    def add(issue_id, issue_type, status, title, minute):
        issues[issue_id] = {
            "id": issue_id,
            "type": issue_type,
            "status": status,
            "title": title,
            "priority": rng.choice(["P0", "P1", "P2", "P2", "P3"]),
            "labels": rng.sample(["cli", "records", "docs", "perf", "review"], 2),
            "created": ts(base, minute),
            "updated": ts(base, minute + rng.randint(0, 5000)),
            "rel": {"blocks": [], "children": [], "advances": []},
        }

    for n, mission in enumerate(missions):
        add(mission, "mission", "in_progress", f"Mission {n}", n)
    for n, epic in enumerate(epics):
        status = rng.choice(["todo", "in_progress", "in_progress", "done"])
        add(epic, "epic", status, f"Epic {n}", 100 + n)
        missions_for_epic = missions[n % len(missions)]
        issues[missions_for_epic]["rel"]["advances"].append(epic)
    for n, task in enumerate(tasks):
        epic = epics[n // children_per_epic]
        epic_done = issues[epic]["status"] == "done"
        status = "done" if epic_done else rng.choices(
            ["todo", "in_progress", "done", "blocked", "review"],
            weights=[50, 10, 30, 5, 5],
        )[0]
        add(task, rng.choice(["task", "bug", "feature"]), status, f"Task {n}", 1000 + n)
        issues[epic]["rel"]["children"].append(task)
    for n, task in enumerate(standalone):
        status = rng.choices(["todo", "in_progress", "done"], weights=[60, 10, 30])[0]
        add(task, "task", status, f"Standalone {n}", 3000 + n)
    for n in range(10):
        issues[missions[n]]["rel"]["advances"].append(standalone[n])

    # Blockers point from an earlier task to a later one, so the graph is
    # acyclic. Missions only advance epics that have no blockers.
    work = tasks + standalone
    for _ in range(150):
        blocker, blocked = sorted(rng.sample(range(len(work)), 2))
        edges = issues[work[blocker]]["rel"]["blocks"]
        if work[blocked] not in edges:
            edges.append(work[blocked])

    issue_dir = repo / ".atelier" / "issues"
    for issue in issues.values():
        (issue_dir / f"{issue['id']}.md").write_text(issue_markdown(issue))
    subprocess.run(["git", "add", "-A"], cwd=repo, check=True)
    subprocess.run(["git", "-c", "user.email=b@b", "-c", "user.name=b", "commit", "-qm", "fixture acceptance"], cwd=repo, check=True)
    commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip()
    evidence_dir = repo / ".atelier" / "evidence"
    done = [i for i in work if issues[i]["status"] == "done"]
    for n, target in enumerate(rng.sample(done, min(200, len(done)))):
        evidence_id = f"atelier-v{n:03d}"
        (evidence_dir / f"{evidence_id}.md").write_text(
            evidence_markdown(evidence_id, target, ts(base, 6000 + n), commit)
        )
    epic_with_children = max(epics, key=lambda e: len(issues[e]["rel"]["children"]))
    return len(issues), missions[0], epic_with_children


def run(binary, repo, args, expected=0):
    started = time.perf_counter()
    result = subprocess.run(
        [str(binary), *args], cwd=repo, capture_output=True, text=True
    )
    elapsed = (time.perf_counter() - started) * 1000
    if result.returncode != expected:
        sys.exit(f"atelier {' '.join(args)} exited {result.returncode}, expected {expected}:\n"
                 f"{result.stdout}\n{result.stderr}")
    return elapsed


def guard_fixture(binary, repo):
    """Give the same sized repository one honestly approved, completed owner."""
    def command(args):
        result = subprocess.run(args, cwd=repo, capture_output=True, text=True)
        if result.returncode:
            sys.exit(f"Fixture command {args} failed:\n{result.stdout}\n{result.stderr}")
        return result.stdout.strip()

    def commit(message):
        command(["git", "add", "-A"])
        command(["git", "commit", "-qm", message])

    for path in (repo / ".atelier/issues").glob("*.md"):
        record = path.read_text()
        record = re.sub(r"^closed_at:.*\n", "", record, flags=re.M)
        record = re.sub(r'^status: ".*"$', 'status: "todo"', record, flags=re.M)
        record = re.sub(r"  blocks:\n(?:  -.*\n(?:    .*\n)*)*", "  blocks: []\n", record)
        path.write_text(record)
    policy = repo / ".atelier/workflow.yaml"
    policy.write_text(policy.read_text().replace("base_branch: main", "base_branch: master"))
    commit("Normalize guard fixture backlog")
    owner = "atelier-s000"
    command([str(binary), "start", owner])
    (repo / "code.rs").write_text("verified fixture code\n")
    commit("Implement fixture owner")
    command([str(binary), "evidence", "record", "--kind", "validation", "--target",
             "issue/" + owner, "--result", "pass", "Observed fixture result"])
    command([str(binary), "issue", "transition", owner, "close", "--reason",
             "Verified fixture owner"])
    commit("Record passing evidence and done claim")
    review = command([str(binary), "-q", "review", "approve", owner, "--identity",
                      "Independent Benchmark Reviewer", "Observed fixture changes"])
    commit("Independent fixture approval")
    run(binary, repo, ["check"])
    return owner, review


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--binary", default=str(ROOT / "target/release/atelier"))
    parser.add_argument("--issues", type=int, default=1000)
    parser.add_argument("--runs", type=int, default=10)
    parser.add_argument("--keep", action="store_true", help="keep the fixture repo")
    options = parser.parse_args()
    if options.runs < 1 or options.issues < 220 or (options.issues - 160) % 60:
        parser.error("runs must be positive; issues must be 160 + a positive multiple of 60")
    binary = Path(options.binary).resolve()
    if not binary.exists():
        sys.exit(f"{binary} not found; run `cargo build --release` first")

    repo = Path(tempfile.mkdtemp(prefix="atelier-bench-"))
    guard_repo = None
    try:
        subprocess.run(["git", "init", "-q", "-b", "master"], cwd=repo, check=True)
        subprocess.run(["git", "config", "user.name", "Benchmark Author"], cwd=repo, check=True)
        subprocess.run(["git", "config", "user.email", "benchmark@example.test"], cwd=repo, check=True)
        run(binary, repo, ["init"])
        count, mission, epic = generate(repo, options.issues)
        for args in (["add", "-A"], ["-c", "user.email=b@b", "-c", "user.name=b",
                                      "commit", "-qm", "fixture"]):
            subprocess.run(["git", *args], cwd=repo, check=True)
        evidence = len(list((repo / ".atelier/evidence").glob("*.md")))
        print(f"Fixture: {count} issues, {evidence} evidence records in {repo}")
        run(binary, repo, ["check"])
        todo = next(path.stem for path in sorted((repo / ".atelier/issues").glob("*.md"))
                    if 'status: "todo"' in path.read_text())

        commands = [
            ["work"],
            ["work", "ready"],
            ["work", "blocked"],
            ["work", "active"],
            ["work", "all"],
            ["work", "missions"],
            ["work", "missions", "--all"],
            ["work", "epic", epic],
            ["status"],
            ["issue", "show", epic],
            ["issue", "show", mission],
            ["issue", "show", "atelier-t000"],
            ["work", "mission", mission],
            *[["work", "mission", mission, "--" + bucket]
              for bucket in ("ready", "blocked", "active", "done", "all")],
            ["issue", "list"],
            ["issue", "list", "--limit", str(count)],
            ["issue", "list", "--status", "done"],
            ["issue", "list", "--label", "cli", "--issue-type", "task"],
            ["issue", "transition", todo],
            ["issue", "transition", todo, "--verbose"],
            ["evidence", "list"],
            ["evidence", "list", "--result", "pass"],
            ["evidence", "show", "atelier-v000"],
            ["review", "list"],
            ["review", "list", "--json"],
            ["check"],
            ["check", epic],
            ["prime"],
            ["prune"],
            ["prune", "--quiet"],
            ["bundle", "preview", str(ROOT / "docs/spec/bundle/fixtures/valid-bundle.json")],
        ]
        guard_repo = Path(tempfile.mkdtemp(prefix="atelier-bench-guards-"))
        shutil.copytree(repo, guard_repo, dirs_exist_ok=True)
        owner, review = guard_fixture(binary, guard_repo)
        guard_commands = [
            ["prime"], ["land"], ["land", owner],
            ["merge", owner, "--check"], ["merge", owner, "--check", "--json"],
            ["review", "list", owner], ["review", "list", owner, "--json"],
            ["review", "show", review],
        ]
        print(f"Guard fixture: {count} issues, {evidence + 1} evidence, one approval in {guard_repo}")
        print(f"Binary: {binary}; {options.runs} runs after one warm-up")
        available = (
            len(os.sched_getaffinity(0))
            if hasattr(os, "sched_getaffinity")
            else os.cpu_count()
        )
        print(f"CPU cores available: {available} of {os.cpu_count()}\n")
        print(f"| Command | median ms | max ms |")
        print(f"| --- | ---: | ---: |")
        over = []
        for fixture, group, args, expected in (
            [(repo, "query", args, 0) for args in commands]
            + [(guard_repo, "guard", args, 0) for args in guard_commands]
            + [(repo, "refusal", ["land"], 1)]
        ):
            run(binary, fixture, args, expected)
            samples = [run(binary, fixture, args, expected) for _ in range(options.runs)]
            median = statistics.median(samples)
            label = " ".join("<epic>" if a == epic else "<mission>" if a == mission else a
                             for a in args)
            print(f"| {group}: `atelier {label}` | {median:.1f} | {max(samples):.1f} |")
            if max(samples) >= BUDGET_MS:
                over.append(f"{group}: {label}")
        if over:
            sys.exit(f"\nMedian or max over the {BUDGET_MS:.0f} ms budget: {', '.join(over)}")
    finally:
        if options.keep:
            print(f"\nKept fixture at {repo}")
            if guard_repo:
                print(f"Kept guard fixture at {guard_repo}")
        else:
            shutil.rmtree(repo)
            if guard_repo:
                shutil.rmtree(guard_repo)


if __name__ == "__main__":
    main()
