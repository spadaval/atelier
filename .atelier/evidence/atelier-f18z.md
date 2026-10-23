---
created_at: "2026-10-23T17:05:57.766531636+00:00"
id: "atelier-f18z"
evidence_type: "test"
commit: "be96c10dbb108dace195f37b2659dc1623f1e752"
result: "pass"
captured_at: "2026-10-23T17:05:55.077315461+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(printed_fixes_run_in_order_recover_a_committed_review_deletion) | test(an_operation_left_in_progress_is_reported_before_any_gate) | test(a_changed_review_record_is_restored_and_a_committed_rewrite_is_dropped)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "Findings atelier-61il and atelier-vlhn: printed fixes run in order recover a committed review deletion and land passes; the commit-addressed rebase works at and below the tip; a rebase or merge in progress is reported first with its abort, run verbatim (rerun after merging master)"
updated_at: "2026-10-23T17:05:57.767233808+00:00"
---

## Summary

Findings atelier-61il and atelier-vlhn: printed fixes run in order recover a committed review deletion and land passes; the commit-addressed rebase works at and below the tip; a rebase or merge in progress is reported first with its abort, run verbatim (rerun after merging master)

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
SHA-256: 39d06e61c9d5c777e3eeb56178869539199e4602ba76043abf2c7bc46548b0ab
Test summary: Summary [   0.619s] 3 tests run: 3 passed, 734 skipped
