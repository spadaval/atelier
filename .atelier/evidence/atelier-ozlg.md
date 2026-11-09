---
created_at: "2026-11-09T18:29:04.582188905+00:00"
id: "atelier-ozlg"
evidence_type: "test"
commit: "d131401b117aa7a378d6f2922cc6605bbb9c3859"
result: "pass"
captured_at: "2026-11-09T18:29:00.216608309+00:00"
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
title: "Regression test for both real cases, the unlink and prune transitions, plus 249ce829's stale-approval test"
updated_at: "2026-11-09T18:29:04.718703080+00:00"
---

## Summary

Regression test for both real cases, the unlink and prune transitions, plus 249ce829's stale-approval test

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test session -E 'test(base_done_claims_keep_planning_approvals_through_other_missions_strategy_edits) | test(retained_stale_approvals_never_age_into_acceptance)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 676
SHA-256: 0cafd15b7c310edc73983b52cbadb599865419c684755e4e99a2fb5411754c02
Test summary: Summary [   1.805s] 2 tests run: 2 passed, 140 skipped
