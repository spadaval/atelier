---
created_at: "2026-11-09T18:09:52.388748092+00:00"
id: "atelier-pxul"
evidence_type: "test"
commit: "cd8fde710f211ac60dbd2f71fdac679ee21e6d2d"
result: "pass"
captured_at: "2026-11-09T18:09:47.075763266+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session -E 'test(base_done_claims_keep_planning_approvals_through_other_missions_strategy_edits) | test(retained_stale_approvals_never_age_into_acceptance)'"
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
title: "Regression test for both real cases plus 249ce829's stale-approval test"
updated_at: "2026-11-09T18:09:52.522345316+00:00"
---

## Summary

Regression test for both real cases plus 249ce829's stale-approval test

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test session -E 'test(base_done_claims_keep_planning_approvals_through_other_missions_strategy_edits) | test(retained_stale_approvals_never_age_into_acceptance)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 771
SHA-256: fb2122b28da08307d0f855bc298cbaeae2d5e7b89aa5a94213db9debe2c58391
Test summary: Summary [   1.456s] 2 tests run: 2 passed, 140 skipped
