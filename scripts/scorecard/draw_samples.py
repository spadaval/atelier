#!/usr/bin/env python3
"""Draw the seeded samples for the scorecard's three classified measures.

  draw_samples.py [--out DIR]

Reads only the frozen baseline: T3's database in the snapshot, opened
read-only and immutable, and each repository's `.atelier/` records at its
manifest commit. Writes the unlabelled samples (interventions.json,
refusals.json, findings.json) to DIR, by default the `samples/` directory
beside the manifest, and prints each population's size. The same snapshot,
manifest and seed give the same entries.

The populations, and how to judge them, are defined in
docs/architecture/quality/scorecard-baseline/rubric.md.
"""
import argparse
import json
import random
import re
import sqlite3
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

SEED = 20261110
WINDOW_START = "2026-10-14T00:00:00.000Z"
FINDINGS_PER_REPO = 20
REFUSALS_DRAWN = 130  # 100 judged, plus reserve for entries excluded by the rubric

ROOT = Path(__file__).resolve().parents[2]
BASELINE = ROOT / "docs/architecture/quality/scorecard-baseline"
REPOS = Path.home() / "projects"
# T3 project IDs of the five repositories in the snapshot.
PROJECTS = {
    "mcp:6767fcba-53d7-4aaa-985c-3210ab28cece": "atelier",
    "cb783c06-0fbd-4e0e-a3d3-15b1db6a165d": "astromech",
    "8039e9bf-63a1-4a8a-abea-00bce74f40f1": "demiurge",
    "fa4ab682-72b9-4ef5-9717-281dd5765548": "life-on-rails",
    "820f2032-cbfa-4844-a1dd-5f21822c81df": "cosplay-studio",
}

# A refusal: atelier's own "<command>: refused" or "Error: " line. The
# harness's "Error: Exit code N" and the argument parser's lowercase
# "error:" are not refusals.
ATELIER_CALL = re.compile(r"(?:(?<![\w/.\-])|target/(?:debug|release)/|\.cargo/bin/)atelier\s+[a-z]")
REFUSAL = re.compile(r"^(?:[a-z][\w-]*: refused\b|Error: (?!Exit code \d))", re.M)
# Calls that are not real use: a scratch or fixture repository, a dry run or
# diagnosis, a help lookup, or a test run whose output holds test errors.
SCRATCH = re.compile(
    r"(cd|-C|pushd|--repo)\s+[^&;|\n]*(\.cache/|/tmp|fixture|scratch|sandbox|tmp\.|"
    r"\$T\b|\$D\b|\$R\b|\$TMP|probe|demo)|mktemp|git init")
DIAGNOSIS = re.compile(r"atelier\s+(merge|land)\b[^\n;&|]*--check|atelier\s+(check|doctor)\b|--help|atelier\s+help\b")
TEST_RUN = re.compile(r"cargo\s+(nextest|test)|pnpm\s+(test|e2e)|pytest")


def utc(stamp):
    """Any ISO time as T3's form, 2026-11-10T05:14:58.000Z, so strings compare."""
    t = datetime.fromisoformat(stamp.replace("Z", "+00:00")).astimezone(timezone.utc)
    return t.strftime("%Y-%m-%dT%H:%M:%S.") + f"{t.microsecond // 1000:03d}Z"


def git(repo, *args):
    return subprocess.run(["git", "-C", str(REPOS / repo), *args],
                          capture_output=True, text=True, check=True).stdout


def records(repo, commit, directory):
    """Every Markdown record in a directory at a commit: (path, header, body)."""
    paths = [p for p in git(repo, "ls-tree", "-r", "--name-only", commit, directory).split() if p.endswith(".md")]
    raw = subprocess.run(["git", "-C", str(REPOS / repo), "cat-file", "--batch"],
                         input="".join(f"{commit}:{p}\n" for p in paths).encode(),
                         capture_output=True, check=True).stdout
    out, i = [], 0
    for path in paths:
        end = raw.index(b"\n", i)
        size = int(raw[i:end].split()[2])
        text = raw[end + 1:end + 1 + size].decode()
        i = end + 1 + size + 1
        m = re.match(r"---\n(.*?)\n---\n?(.*)", text, re.S)
        if m:
            out.append((path, m.group(1), m.group(2)))
    return out


def field(header, key):
    m = re.search(rf"^{key}: *\"?([^\"\n]*)\"?$", header, re.M)
    return m.group(1).strip("'") if m else None


def in_window(stamp, cutoff):
    return stamp is not None and WINDOW_START <= utc(stamp) <= cutoff


def draw_findings(manifest, cutoff):
    """20 fix-now findings from each repository, then the 100 in seeded order."""
    rng, drawn, sizes = random.Random(SEED), [], {}
    for repo, info in manifest["repositories"].items():
        found = []
        for path, header, _ in records(repo, info["commit"], ".atelier/reviews"):
            if (re.search(r"^  event: finding$", header, re.M)
                    and re.search(r"^  recommendation: fix_now$", header, re.M)
                    and in_window(field(header, "created_at"), cutoff)):
                found.append(dict(repository=repo, commit=info["commit"], record=path,
                                  finding=field(header, "id"),
                                  target=field(header, "target") or "branch:" + str(field(header, "branch")),
                                  reviewed_commit=field(header, "commit")))
        found.sort(key=lambda f: f["finding"])
        sizes[repo] = len(found)
        drawn += rng.sample(found, min(FINDINGS_PER_REPO, len(found)))
    rng.shuffle(drawn)
    return drawn, sizes


def project_threads(db):
    marks = ",".join("?" * len(PROJECTS))
    return {tid: (PROJECTS[pid], json.loads(payload)) for tid, pid, payload in db.execute(
        f"select thread_id, project_id, payload_json from orchestration_v2_projection_threads "
        f"where project_id in ({marks})", list(PROJECTS))}


def draw_refusals(db, cutoff):
    population = []
    for tid, (repo, _) in project_threads(db).items():
        for item, ordinal, payload in db.execute(
                "select turn_item_id, ordinal, payload_json from orchestration_v2_projection_turn_items "
                "where thread_id = ? and type = 'command_execution'", (tid,)):
            p = json.loads(payload)
            cmd, out, stamp = p.get("input") or "", p.get("output") or "", p.get("startedAt")
            if not (ATELIER_CALL.search(cmd) and REFUSAL.search(out) and in_window(stamp, cutoff)):
                continue
            if SCRATCH.search(cmd) or DIAGNOSIS.search(cmd) or TEST_RUN.search(cmd):
                continue
            population.append(dict(repository=repo, thread=tid, item=item, ordinal=ordinal, timestamp=stamp))
    population.sort(key=lambda r: (r["timestamp"], r["item"]))
    return random.Random(SEED).sample(population, min(REFUSALS_DRAWN, len(population))), len(population)


def closed_missions(manifest, cutoff):
    """Missions closed in the window, with their start and the items that advance them."""
    missions = {}
    for repo, info in manifest["repositories"].items():
        issues = records(repo, info["commit"], ".atelier/issues")
        for path, header, _ in issues:
            if field(header, "issue_type") != "mission" or not in_window(field(header, "closed_at"), cutoff):
                continue
            mid = field(header, "id")
            items = [field(h, "id") for _, h, _ in issues if any(
                re.search(rf'id: "?{re.escape(mid)}"?\n', b) and re.search(r'type: "?advances', b)
                for b in h.split("- kind:")[1:])]
            # Start: the first commit at which the record left draft or backlog.
            start = None
            for line in git(repo, "log", "--reverse", "--format=%H %cI", info["commit"], "--", path).splitlines():
                commit, stamp = line.split(" ", 1)
                status = field(git(repo, "show", f"{commit}:{path}"), "status")
                if status in ("todo", "in_progress", "done"):
                    start = utc(stamp)
                    break
            missions[mid] = dict(repository=repo, status=field(header, "status"), start=start,
                                 close=utc(field(header, "closed_at")), items=items)
    return missions


def draw_interventions(db, missions):
    """Every typed message of Sumeet's in a closed mission's threads, in seeded order."""
    threads = project_threads(db)
    parent = {tid: (json.loads(p).get("lineage") or {}).get("parentThreadId")
              for tid, p in db.execute("select thread_id, payload_json from orchestration_v2_projection_threads")}
    direct, owners = {}, {}
    for tid, (repo, p) in threads.items():
        first = db.execute("select payload_json from orchestration_v2_projection_messages where thread_id = ? "
                           "and role = 'user' order by created_at limit 1", (tid,)).fetchone()
        text = " ".join([p.get("branch") or "", p.get("title") or "",
                         json.loads(first[0]).get("text", "") if first else ""])
        for mid, m in missions.items():
            if m["start"] and m["repository"] == repo and any(
                    re.search(rf"(?<![\w-]){re.escape(i)}(?![\w-])", text) for i in [mid] + m["items"]):
                direct.setdefault(tid, set()).add(mid)
    for tid, mids in direct.items():
        up, seen = tid, set()
        while up and up not in seen:  # the thread and every thread above it
            seen.add(up)
            owners.setdefault(up, set()).update(mids)
            up = parent.get(up)
    entries, agent_written = [], 0
    for tid, mids in owners.items():
        for msg, created, payload in db.execute(
                "select message_id, created_at, payload_json from orchestration_v2_projection_messages "
                "where thread_id = ? and role = 'user'", (tid,)):
            live = sorted(m for m in mids if max(missions[m]["start"], WINDOW_START) <= created <= missions[m]["close"])
            if not live:
                continue
            if json.loads(payload).get("createdBy") != "user":
                agent_written += 1
                continue
            entries.append(dict(repository=missions[live[0]]["repository"], missions=live,
                                thread=tid, message=msg, timestamp=created))
    entries.sort(key=lambda e: (e["timestamp"], e["message"]))
    random.Random(SEED).shuffle(entries)
    stats = dict(threads_in_projects=len(threads), threads_tied_directly=len(direct),
                 threads_tied_through_lineage=len(set(owners) - set(direct)),
                 threads_tied_to_several=sum(len(v) > 1 for v in owners.values()),
                 agent_written_messages_left_out=agent_written)
    return entries, stats


def write(out, measure, entries, population, note):
    doc = dict(measure=measure, seed=SEED, population=population, drawn_by="scripts/scorecard/draw_samples.py",
               note=note, entries=[{"n": n, **e} for n, e in enumerate(entries, 1)])
    (out / f"{measure}.json").write_text(json.dumps(doc, indent=1) + "\n")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", type=Path, default=BASELINE / "samples")
    args = ap.parse_args()
    manifest = json.loads((BASELINE / "manifest.json").read_text())
    cutoff = utc(manifest["cutoff"])
    snapshot = Path(manifest["snapshot"]).expanduser() / manifest["parts"]["t3_database"]["path"]
    db = sqlite3.connect(f"file:{snapshot}?mode=ro&immutable=1", uri=True)
    args.out.mkdir(parents=True, exist_ok=True)

    findings, sizes = draw_findings(manifest, cutoff)
    write(args.out, "findings", findings, sizes,
          "20 fix-now findings drawn from each repository, then all 100 shuffled.")
    refusals, size = draw_refusals(db, cutoff)
    write(args.out, "refusals", refusals, size,
          "Seeded order. Judge in order; gate precision counts the first 100 entries not labelled excluded.")
    missions = closed_missions(manifest, cutoff)
    interventions, stats = draw_interventions(db, missions)
    write(args.out, "interventions", interventions, len(interventions),
          "Every typed message in a closed mission's threads, in seeded order.")

    print(f"fix-now findings in the window: {sizes}")
    print(f"refusals in real use: {size}")
    print("missions closed in the window:")
    for mid, m in sorted(missions.items(), key=lambda kv: (kv[1]["repository"], kv[0])):
        count = sum(mid in e["missions"] for e in interventions)
        print(f"  {m['repository']} {mid} {m['status']} start={m['start']} close={m['close']} messages={count}")
    print(f"intervention messages: {len(interventions)}; {stats}")


if __name__ == "__main__":
    sys.exit(main())
