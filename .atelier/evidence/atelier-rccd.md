---
created_at: "2026-10-23T00:02:43.329914318+00:00"
id: "atelier-rccd"
evidence_type: "validation"
commit: "f3ee2581393146618dcb92e6bd5d971bdfb6110b"
result: "pass"
captured_at: "2026-10-23T00:02:42.097235939+00:00"
command: "bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s1.sh"
exit_status: "0"
path: "/home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s1.sh"
agent_identity: "claude-validator"
target:
  kind: "issue"
  id: "atelier-k8h8"
  role: "validates"
criteria:
- slug: "S1"
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
title: "S1 claim: status shows each waiting branch's readiness against the current base (behind count, clean merge or conflicting files, approval state, proof at head) and which waiting branches share files. Method: s1.sh builds a scratch repo under TMPDIR with three done-not-merged branches (approved with proof at head; approval stale with later code; unapproved with proof older than head and a conflicting edit) while the base moves 2 commits, then asserts atelier status lines. Observed: 'approved, behind 2, proof at head, merges cleanly', 'approval stale, behind 2, no proof at head, merges cleanly', 'not approved, behind 2, no proof at head, conflicts: c.txt', and 'Shared files: task/<c> and task/<b> both touch shared.txt'. Test waiting_branches_show_merge_readiness_against_the_base also passes in the full suite (894/894). Limits: readiness is shown only for done-not-merged branches; in-progress branches get a plain row, which matches 'waiting'."
updated_at: "2026-10-23T00:02:43.331438869+00:00"
---

## Summary

S1 claim: status shows each waiting branch's readiness against the current base (behind count, clean merge or conflicting files, approval state, proof at head) and which waiting branches share files. Method: s1.sh builds a scratch repo under TMPDIR with three done-not-merged branches (approved with proof at head; approval stale with later code; unapproved with proof older than head and a conflicting edit) while the base moves 2 commits, then asserts atelier status lines. Observed: 'approved, behind 2, proof at head, merges cleanly', 'approval stale, behind 2, no proof at head, merges cleanly', 'not approved, behind 2, no proof at head, conflicts: c.txt', and 'Shared files: task/<c> and task/<b> both touch shared.txt'. Test waiting_branches_show_merge_readiness_against_the_base also passes in the full suite (894/894). Limits: readiness is shown only for done-not-merged branches; in-progress branches get a plain row, which matches 'waiting'.

## Command

```console
bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s1.sh
```

Exit status: 0

## Stdout

Bytes: 819
Truncated: no

```text
Branches
--------
task/s1-75uq  s1-75uq ✓ done, not merged, ⚠ approval stale, behind 2, no proof at head, merges cleanly  charlie
task/s1-7otn  s1-7otn ✓ done, not merged, ⚠ not approved, behind 2, no proof at head, conflicts: c.txt  bravo
task/s1-ktea  s1-ktea ✓ done, not merged, ✓ approved, behind 2, proof at head, merges cleanly  alpha
Shared files: task/s1-75uq and task/s1-7otn both touch shared.txt

Next
ok: task/s1-ktea  s1-ktea ✓ done, not merged, ✓ approved, behind 2, proof at head, merges cleanly
ok: task/s1-75uq  s1-75uq ✓ done, not merged, ⚠ approval stale, behind 2, no proof at head, merges cleanly
ok: task/s1-7otn  s1-7otn ✓ done, not merged, ⚠ not approved, behind 2, no proof at head, conflicts: c.txt
ok: Shared files: task/s1-75uq and task/s1-7otn both touch shared.txt
```

## Stderr

Bytes: 0
Truncated: no

```text
```

