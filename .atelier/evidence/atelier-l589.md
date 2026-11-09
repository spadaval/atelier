---
created_at: "2026-11-09T18:40:31.115403546+00:00"
id: "atelier-l589"
evidence_type: "test"
commit: "089ddbdf5602fecd693f8291113687ddffb2d84b"
result: "pass"
captured_at: "2026-11-09T18:40:27.215446215+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session -E 'test(base_done_claims_keep_planning_approvals_through_other_missions_strategy_edits) | test(base_done_child_keeps_its_mission_strategy_edit_stale_under_a_parent_approval) | test(retained_stale_approvals_never_age_into_acceptance)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ygul"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ygul"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Regressions for both real cases, unlink, prune, a parent approval over a mission-linked child, and 249ce829's stale-approval test"
updated_at: "2026-11-09T18:40:31.250121744+00:00"
---

## Summary

Regressions for both real cases, unlink, prune, a parent approval over a mission-linked child, and 249ce829's stale-approval test

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test session -E 'test(base_done_claims_keep_planning_approvals_through_other_missions_strategy_edits) | test(base_done_child_keeps_its_mission_strategy_edit_stale_under_a_parent_approval) | test(retained_stale_approvals_never_age_into_acceptance)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 806
SHA-256: 4fe1b4c0692eedbb5e96e0353adb2395cb4412b616c22e1996a38b50cf5310de
Test summary: Summary [   1.731s] 3 tests run: 3 passed, 140 skipped
