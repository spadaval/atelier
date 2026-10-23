---
created_at: "2026-10-23T16:29:07.970402227+00:00"
id: "atelier-lr9v"
evidence_type: "test"
commit: "7582a2221784baf5591841f72de9706bfa66ecec"
result: "pass"
captured_at: "2026-10-23T16:29:05.344135086+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(printed_fixes_run_in_order_recover_a_committed_review_deletion) | test(an_operation_left_in_progress_is_reported_before_any_gate) | test(a_changed_review_record_is_restored_and_a_committed_rewrite_is_dropped)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
target:
  kind: "issue"
  id: "atelier-0uaj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0uaj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Findings atelier-61il and atelier-vlhn: printed fixes run in order recover a committed review deletion and land passes; the commit-addressed rebase works at and below the tip; a rebase or merge in progress is reported first with its abort, run verbatim"
updated_at: "2026-10-23T16:29:07.970687339+00:00"
---

## Summary

Findings atelier-61il and atelier-vlhn: printed fixes run in order recover a committed review deletion and land passes; the commit-addressed rebase works at and below the tip; a rebase or merge in progress is reported first with its abort, run verbatim

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(printed_fixes_run_in_order_recover_a_committed_review_deletion) | test(an_operation_left_in_progress_is_reported_before_any_gate) | test(a_changed_review_record_is_restored_and_a_committed_rewrite_is_dropped)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 798
SHA-256: d4748da3ffe4759184d68cce890ee29019205e4eda0ae1d99f909de9c718e1c5
Test summary: Summary [   0.652s] 3 tests run: 3 passed, 727 skipped
