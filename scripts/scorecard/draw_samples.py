#!/usr/bin/env python3
"""Draw the seeded samples for the scorecard's three classified measures.

  draw_samples.py [--out DIR]

Reads only frozen inputs: T3's database in the snapshot, opened read-only
and immutable, each repository's `atelier log` in the snapshot, and its
`.atelier/` records at its manifest commit. Writes the unlabelled samples (interventions.json,
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
import sys
from datetime import datetime, timezone
from pathlib import Path

import atelier_log
from snapshot import git

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
FENCE = re.compile(r" {0,3}(`{3,}|~{3,})(.*)")


def utc(stamp):
    """Any ISO time as T3's form, 2026-11-10T05:14:58.000Z, so strings compare."""
    t = datetime.fromisoformat(stamp.replace("Z", "+00:00")).astimezone(timezone.utc)
    return t.strftime("%Y-%m-%dT%H:%M:%S.") + f"{t.microsecond // 1000:03d}Z"


def reviews(repo, commit):
    """Every review file at a commit: (path, header, body)."""
    paths = [p for p in git(REPOS / repo, "ls-tree", "-r", "--name-only", commit, ".atelier").split()
             if "/reviews/" in p and p.endswith(".md")]
    out = []
    for path in paths:
        m = re.match(r"---\n(.*?)\n---\n?(.*)", git(REPOS / repo, "show", f"{commit}:{path}"), re.S)
        if m:
            out.append((path, m.group(1), m.group(2)))
    return out


def unfenced(text):
    """The text's lines outside fenced code blocks, fence lines left out, as the records read them
    (`unfenced_lines` in crates/atelier-records/src/document.rs): an example is never a heading or a field."""
    lines, open_ = [], None  # the open fence's marks
    for line in text.split("\n"):
        fence = FENCE.fullmatch(line)
        if fence and not (fence[1][0] == "`" and "`" in fence[2]) and (not open_ or (
                fence[1][0] == open_[0] and len(fence[1]) >= len(open_) and not fence[2].strip())):
            open_ = None if open_ else fence[1]
        elif not open_:
            lines.append(line)
    return "\n".join(lines)


def field(header, key):
    m = re.search(rf"^{key}: *\"?([^\"\n]*)\"?$", header, re.M)
    return m.group(1).strip("'") if m else None


def in_window(stamp, cutoff):
    return stamp is not None and WINDOW_START <= utc(stamp) <= cutoff


def draw_findings(manifest, cutoff):
    """20 fix-now findings raised in each repository, then the 100 in seeded order.

    A finding a later round carries (its `Was:` line) was raised in the round it names, so it is left out.
    """
    rng, drawn, sizes = random.Random(SEED), [], {}
    for repo, info in manifest["repositories"].items():
        found = []
        for path, header, body in reviews(repo, info["commit"]):
            if not in_window(field(header, "created_at"), cutoff):
                continue
            target = path.split("/")[2]
            for section in re.split(r"^## ", unfenced(body), flags=re.M):
                number = re.match(r"Finding (\d+): ", section)
                if number and re.search(r"^Recommendation: fix now$", section, re.M) and not re.search(r"^Was: ", section, re.M):
                    found.append(dict(repository=repo, commit=info["commit"], record=path,
                                      finding=f"{target}/{Path(path).stem}.{number.group(1)}", target=target,
                                      reviewed_commit=field(header, "commit")))
        found.sort(key=lambda f: f["finding"])
        sizes[repo] = len(found)
        drawn += rng.sample(found, min(FINDINGS_PER_REPO, len(found)))
    rng.shuffle(drawn)
    return drawn, sizes


def project_threads(db):
    marks = ",".join("?" * len(PROJECTS))
    return {tid: PROJECTS[pid] for tid, pid in db.execute(
        f"select thread_id, project_id from orchestration_v2_projection_threads "
        f"where project_id in ({marks})", list(PROJECTS))}


def draw_refusals(db, cutoff):
    population = []
    for tid, repo in project_threads(db).items():
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


def closed_missions(manifest, snap, cutoff):
    """Missions closed in the window: start and close from their `ready` and `close` lines, their items
    from the item records that name them, and the T3 threads on the mission's and its items' lines."""
    missions = {}
    for repo, info in manifest["repositories"].items():
        lines = atelier_log.read(snap, repo, lambda stamp: utc(stamp) <= cutoff)
        ids, items = atelier_log.missions(REPOS / repo, info["commit"]), mission_items(repo, info["commit"])
        for close in lines:
            if close.act != "close" or close.record not in ids or not in_window(close.time, cutoff):
                continue
            mid, mine = close.record, items.get(close.record, [])
            missions[mid] = dict(repository=repo, items=mine, close=utc(close.time),
                                 start=next((utc(l.time) for l in lines if l.record == mid and l.act == "ready"), None),
                                 threads={atelier_log.t3_thread(l.thread) for l in lines if l.thread and l.record in {mid, *mine}})
    return missions


def mission_items(repo, commit):
    """Item IDs by the mission their record's front matter names at a commit."""
    found, dashes = {}, {}
    for line in git(REPOS / repo, "grep", "-e", "^---$", "-e", "^mission: ", commit, "--", ".atelier/items/*/item.md").splitlines():
        _, path, text = line.split(":", 2)
        if text == "---":
            dashes[path] = dashes.get(path, 0) + 1
        elif dashes.get(path) == 1 and text != "mission: null":  # between the front matter's two `---` lines
            found.setdefault(text.split(": ", 1)[1], []).append(path.split("/")[2])
    return found


def draw_interventions(db, missions):
    """Every typed message of Sumeet's in a closed mission's threads, in seeded order."""
    threads = project_threads(db)
    parent = {tid: (json.loads(p).get("lineage") or {}).get("parentThreadId")
              for tid, p in db.execute("select thread_id, payload_json from orchestration_v2_projection_threads")}
    direct, owners = {}, {}
    for mid, m in missions.items():
        for tid in m["threads"] if m["start"] else ():
            if threads.get(tid) == m["repository"]:
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
    snap = Path(manifest["snapshot"]).expanduser()
    db = sqlite3.connect(f"file:{snap / manifest['parts']['t3_database']['path']}?mode=ro&immutable=1", uri=True)
    args.out.mkdir(parents=True, exist_ok=True)

    findings, sizes = draw_findings(manifest, cutoff)
    write(args.out, "findings", findings, sizes,
          "20 fix-now findings drawn from each repository, then all 100 shuffled.")
    refusals, size = draw_refusals(db, cutoff)
    write(args.out, "refusals", refusals, size,
          "Seeded order. Judge in order; gate precision counts the first 100 entries not labelled excluded.")
    missions = closed_missions(manifest, snap, cutoff)
    interventions, stats = draw_interventions(db, missions)
    write(args.out, "interventions", interventions, len(interventions),
          "Every typed message in a closed mission's threads, in seeded order.")

    print(f"fix-now findings in the window: {sizes}")
    print(f"refusals in real use: {size}")
    print("missions closed in the window:")
    for mid, m in sorted(missions.items(), key=lambda kv: (kv[1]["repository"], kv[0])):
        count = sum(mid in e["missions"] for e in interventions)
        print(f"  {m['repository']} {mid} start={m['start']} close={m['close']} messages={count}")
    print(f"intervention messages: {len(interventions)}; {stats}")


if __name__ == "__main__":
    sys.exit(main())
