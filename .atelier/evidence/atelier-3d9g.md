---
created_at: "2026-10-22T22:58:04.968689709+00:00"
id: "atelier-3d9g"
evidence_type: "test"
commit: "c724357e3009b0520d4335d4541700096ba23589"
result: "pass"
captured_at: "2026-10-22T22:58:02.199739462+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks --test merge_gates sync"
exit_status: "0"
agent_identity: "claude-implementer-fngz"
target:
  kind: "issue"
  id: "atelier-o2pm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-o2pm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "merge --sync names a hand-merge resume and its merge commit, prints each rerun command before it starts, and ends 'merge: synced — approval needs fresh review' when the approval is stale"
updated_at: "2026-10-22T22:58:04.969016758+00:00"
---

## Summary

merge --sync names a hand-merge resume and its merge commit, prints each rerun command before it starts, and ends 'merge: synced — approval needs fresh review' when the approval is stale

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks --test merge_gates sync
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1306
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.18s
────────────
 Nextest run ID d5f6c838-d667-4a7a-9646-8bb45d9bc4b4 with nextest profile: default
    Starting 7 tests across 1 binary (64 tests skipped)
        PASS [   0.269s] (1/7) atelier-cli::merge_gates sync_names_the_commit_that_staled_the_approval_before_the_sync_and_every_path
        PASS [   0.273s] (2/7) atelier-cli::merge_gates sync_lists_only_the_in_scope_record_a_clean_base_merge_changed
        PASS [   0.161s] (3/7) atelier-cli::merge_gates sync_refuses_a_dirty_checkout_and_conflicts_with_check
        PASS [   0.239s] (4/7) atelier-cli::merge_gates sync_reports_a_rerun_that_errors_with_a_code_and_still_reports_the_approval
        PASS [   0.760s] (5/7) atelier-cli::merge_gates sync_records_a_failing_rerun_and_refuses
        PASS [   1.201s] (6/7) atelier-cli::merge_gates sync_merges_the_base_reruns_current_receipts_and_reports_the_approval_kept
        PASS [   0.922s] (7/7) atelier-cli::merge_gates sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution
────────────
     Summary [   1.353s] 7 tests run: 7 passed, 64 skipped
```

