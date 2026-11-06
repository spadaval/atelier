---
created_at: "2026-11-06T23:50:29.574431306+00:00"
id: "atelier-8oxm"
evidence_type: "check"
commit: "7da9150b8adc100a2aee22361c4ef6ed09dad5a6"
result: "pass"
captured_at: "2026-11-06T23:50:29.574319517+00:00"
target:
  kind: "issue"
  id: "atelier-rkxm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rkxm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Counted each workflow status and issue type in atelier, astromech, demiurge and cosplay-studio; the script and rerun command are in the body"
updated_at: "2026-11-06T23:50:29.575212161+00:00"
---

Counted each workflow status and issue type in atelier, astromech, demiurge and cosplay-studio; the counts are the tables in the 2026-11-06 result note on atelier-rkxm, and this run reproduced them exactly (output sha256 prefix bf51f9253bd8525d at atelier 05f01d83, astromech 5d6acaae, demiurge b9c8cbc, cosplay-studio 03057e8).

Rerun: save the script below as count-use.py and run `python3 -I count-use.py ~/projects/atelier ~/projects/astromech ~/projects/demiurge ~/projects/cosplay-studio` (read-only: git log, ls-tree, reflog and cat-file). Counts move as refs move; the base tip and branch columns reflect the refs on the run date.

```python
#!/usr/bin/env python3
"""Count how Atelier repositories use each workflow status and issue type.

Usage: count-use.py REPO...   (read-only: runs git log/ls-tree/cat-file only)

Per repository, for each status and issue type in the base branch's
.atelier/workflow.yaml, prints the number of distinct issue records:
  base     at rest on the base branch tip
  tips     at any tip the local base ref has had, from its reflog (the
           reflog's start date is printed; it covers only this clone)
  base-fp  at any commit on the base's first-parent line (with fast_forward_only
           merges this includes work-branch commits)
  branches in that state at any commit reachable from refs/heads, refs/remotes, refs/tags
  all      same over `git log --all` (adds T3 Code checkpoint refs, refs/t3/*)
Records are .atelier/issues/<id>.md with schema atelier.issue; every blob any
commit held at such a path is read (`git log -m --root --raw` lists merge
results too). A record file git sees as renamed (-M, default 50% similarity)
counts as one record under its newest id. Values in records but not in today's policy are listed after.
"""
import collections, re, subprocess, sys

def git(repo, *args, inp=None):
    return subprocess.run(["git", "-C", repo, *args], input=inp,
                          capture_output=True, check=True).stdout

def fields(body):
    fm = body.split("\n---", 1)[0]
    get = lambda k: (re.search(rf'^{k}:\s*"?([^"\n]*)"?\s*$', fm, re.M) or [None, None])[1]
    return get("schema"), get("id"), get("status"), get("issue_type")

def read_blobs(repo, blobs):
    out, i, res = git(repo, "cat-file", "--batch", inp=("\n".join(blobs) + "\n").encode()), 0, {}
    while i < len(out):
        nl = out.index(b"\n", i); sha, _, size = out[i:nl].split(); size = int(size)
        res[sha.decode()] = fields(out[nl + 1:nl + 1 + size].decode("utf8", "replace"))
        i = nl + 2 + size
    return res

RECORD = re.compile(r"^\.atelier/issues/[^/]+\.md$")

def history_blobs(repo, refs):
    raw = git(repo, "log", *refs, "-m", "--root", "--raw", "--no-abbrev", "--format=",
              "--", ".atelier/issues").decode()
    return {l.split()[3] for l in raw.splitlines()
            if l.startswith(":") and RECORD.match(l.split("\t")[-1]) and l.split()[3] != "0" * 40}

def tip_blobs(repo, ref):
    raw = git(repo, "ls-tree", "-r", ref, "--", ".atelier/issues").decode()
    return {l.split()[2] for l in raw.splitlines() if RECORD.match(l.split("\t")[-1])}

def renames(repo):
    raw = git(repo, "log", "--all", "-m", "--root", "-M", "--diff-filter=R", "--name-status",
              "--format=", "--", ".atelier/issues").decode()
    stem = lambda p: p.rsplit("/", 1)[-1][:-3]
    return {stem(o): stem(n) for _, o, n in (l.split("\t") for l in raw.splitlines() if l.startswith("R"))
            if RECORD.match(o) and RECORD.match(n)}

def tally(meta, blobs, alias):
    st, ty = collections.defaultdict(set), collections.defaultdict(set)
    for b in blobs:
        schema, rid, status, itype = meta[b]
        seen = set()
        while rid in alias and rid not in seen:
            seen.add(rid); rid = alias[rid]
        if schema == "atelier.issue":
            st[status].add(rid); ty[itype].add(rid)
    return st, ty

def policy(text):
    # Top-level `issue_types:` and `statuses:` maps: their two-space keys.
    out, cur = {"issue_types": [], "statuses": []}, None
    for line in text.splitlines():
        if re.match(r"^\S", line):
            cur = line.rstrip(":") if line.rstrip(":") in out else None
        elif cur and (m := re.match(r"^  ([a-z_]+):", line)):
            out[cur].append(m.group(1))
    return out

def main(repos):
    for repo in repos:
        wf = git(repo, "show", "HEAD:.atelier/workflow.yaml").decode()
        base = re.search(r"base_branch:\s*(\S+)", wf).group(1)
        wf = git(repo, "show", f"{base}:.atelier/workflow.yaml").decode()
        pol = policy(wf)
        tips = git(repo, "reflog", "show", "--format=%H", base).decode().split()
        since = git(repo, "reflog", "show", "-1", "--skip", str(len(tips) - 1),
                    "--date=short", "--format=%gd", base).decode().strip()
        sets = {"base": tip_blobs(repo, base),
                "tips": set().union(*(tip_blobs(repo, t) for t in set(tips))),
                "base-fp": history_blobs(repo, ["--first-parent", base]),
                "branches": history_blobs(repo, ["--branches", "--remotes", "--tags"]),
                "all": history_blobs(repo, ["--all"])}
        meta = read_blobs(repo, set().union(*sets.values()))
        alias = renames(repo)
        tallies = {k: tally(meta, v, alias) for k, v in sets.items()}
        print(f"## {repo} (base {base} @ {git(repo, 'rev-parse', '--short', base).decode().strip()}; reflog from {since}; {len(set(tips))} tips; {len(alias)} renamed records merged)")
        for idx, key in ((0, "statuses"), (1, "issue_types")):
            print(f"\n{key:<14}" + "".join(f"{c:>9}" for c in sets))
            for name in pol[key]:
                print(f"{name:<14}" + "".join(f"{len(tallies[c][idx].get(name, ())):>9}" for c in sets))
            extra = sorted(set().union(*(t[idx].keys() for t in tallies.values())) - set(pol[key]), key=str)
            for name in extra:
                print(f"{str(name) + '*':<14}" + "".join(f"{len(tallies[c][idx].get(name, ())):>9}" for c in sets))
        print("\n(* = in records, not in this policy)\n")

main(sys.argv[1:])
```
