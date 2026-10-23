---
created_at: "2026-10-23T00:03:16.826577959+00:00"
id: "atelier-7pql"
evidence_type: "validation"
commit: "f3ee2581393146618dcb92e6bd5d971bdfb6110b"
result: "pass"
captured_at: "2026-10-23T00:03:16.023752278+00:00"
command: "bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s6.sh"
exit_status: "0"
path: "/home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s6.sh"
agent_identity: "claude-validator"
target:
  kind: "issue"
  id: "atelier-k8h8"
  role: "validates"
criteria:
- slug: "S6"
  result: "pass"
criteria_mission: "atelier-k8h8"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-k8h8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S6 claim: in an orchestrated run with >=5 parallel branches, catch-up merges average <=1 per landed branch (baseline astromech-cmhv: 16 catch-ups, half re-reviewed). Method: s6.sh counts, from this repo's history 22306572 (Start mission atelier-k8h8, 2026-10-22 16:32)..master, each first-parent 'Merge <branch> into master' and the merges of master into that branch (merge^1..merge^2), checks each catch-up for conflicts with git merge-tree, and computes peak overlap from first branch commit to landing. Observed: 11 landed branches: c3kh 0, 9g48 0, yim7 0, zuow 0, 1ztv 2 (021e4dac records conflict in 3 issue files; 699b0732 merge.rs and work-model.md), tciw 0, s2ei 1 (13a1724b review_gate.rs), r28p 1 (75d2dc13 review_gate.rs), 7b6e 0, j8kg 0, fngz 1 (9d682eba tests/session.rs). Total 5, average 0.45 (first wave of five: 2/5 = 0.40). All 5 catch-ups were textual conflicts that needed resolution; no clean catch-up was made. Peak parallel: 5 (c3kh, 9g48, yim7, zuow, 1ztv all open 16:32-17:01). All landings were merge_commit merges by atelier merge. Compared: baseline 16 catch-ups on one branch; 2026-10-17 run 9 for 4 landings (2.25). Limits: branch open time is taken from its first commit; no unmerged branch from this run exists; counts are from the commit graph, not session transcripts."
updated_at: "2026-10-23T00:03:16.827589223+00:00"
---

## Summary

S6 claim: in an orchestrated run with >=5 parallel branches, catch-up merges average <=1 per landed branch (baseline astromech-cmhv: 16 catch-ups, half re-reviewed). Method: s6.sh counts, from this repo's history 22306572 (Start mission atelier-k8h8, 2026-10-22 16:32)..master, each first-parent 'Merge <branch> into master' and the merges of master into that branch (merge^1..merge^2), checks each catch-up for conflicts with git merge-tree, and computes peak overlap from first branch commit to landing. Observed: 11 landed branches: c3kh 0, 9g48 0, yim7 0, zuow 0, 1ztv 2 (021e4dac records conflict in 3 issue files; 699b0732 merge.rs and work-model.md), tciw 0, s2ei 1 (13a1724b review_gate.rs), r28p 1 (75d2dc13 review_gate.rs), 7b6e 0, j8kg 0, fngz 1 (9d682eba tests/session.rs). Total 5, average 0.45 (first wave of five: 2/5 = 0.40). All 5 catch-ups were textual conflicts that needed resolution; no clean catch-up was made. Peak parallel: 5 (c3kh, 9g48, yim7, zuow, 1ztv all open 16:32-17:01). All landings were merge_commit merges by atelier merge. Compared: baseline 16 catch-ups on one branch; 2026-10-17 run 9 for 4 landings (2.25). Limits: branch open time is taken from its first commit; no unmerged branch from this run exists; counts are from the commit graph, not session transcripts.

## Command

```console
bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s6.sh
```

Exit status: 0

## Stdout

Bytes: 1148
Truncated: no

```text
feature/atelier-c3kh       catch-ups=0 (conflicted=0) open 16:32..17:01 ;
feature/atelier-9g48       catch-ups=0 (conflicted=0) open 16:32..17:06 ;
feature/atelier-yim7       catch-ups=0 (conflicted=0) open 16:32..17:18 ;
feature/atelier-zuow       catch-ups=0 (conflicted=0) open 16:32..17:51 ;
feature/atelier-1ztv       catch-ups=2 (conflicted=2) open 16:32..18:09 021e4dac Merge branch 'master' into feature/atelier-1ztv;699b0732 Merge master into feature/atelier-1ztv;
feature/atelier-tciw       catch-ups=0 (conflicted=0) open 18:12..18:30 ;
feature/atelier-s2ei       catch-ups=1 (conflicted=1) open 18:12..18:45 13a1724b Merge branch 'master' into feature/atelier-s2ei;
feature/atelier-r28p       catch-ups=1 (conflicted=1) open 18:12..19:00 75d2dc13 Merge branch 'master' into feature/atelier-r28p;
task/atelier-7b6e          catch-ups=0 (conflicted=0) open 18:34..19:23 ;
task/atelier-j8kg          catch-ups=0 (conflicted=0) open 19:03..19:30 ;
task/atelier-fngz          catch-ups=1 (conflicted=1) open 18:48..19:50 9d682eba Merge master into task/atelier-fngz;
landed branches=11 catch-up merges=5 average=0.45 peak open in parallel=5
```

## Stderr

Bytes: 0
Truncated: no

```text
```

