---
created_at: "2026-10-22T23:28:11.776841208+00:00"
id: "atelier-sryw"
evidence_type: "test"
commit: "9d682eba58800134ce73ba81f6851b4886edbf96"
result: "pass"
captured_at: "2026-10-22T23:28:03.730208779+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks --test merge_gates sync"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "## Summary\n\nmerge --sync's approval report agrees with merge --check when the base already holds work after the approval, under both strategies (atelier-ns03)\n\n## Command\n\n```console\ncargo nextest run -j 4 -p atelier-cli --features test-hooks --test merge_gates sync\n```\n\nExit status: 0\n\n## Stdout\n\nBytes: 0\nTruncated: no\n\n```text\n```\n\n## Stderr\n\nBytes: 1497\nTruncated: no\n\n```text\ncargo-gate: 2 builds already running, waiting for a slot...\n   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.00s\n────────────\n Nextest run ID c1d8e449-3661-4b8f-b753-d19e6e4ff2fd with nextest profile: default\n    Starting 8 tests across 1 binary (65 tests skipped)\n        PASS [   0.430s] (1/8) atelier-cli::merge_gates sync_lists_only_the_in_scope_record_a_clean_base_merge_changed\n        PASS [   0.447s] (2/8) atelier-cli::merge_gates sync_names_the_commit_that_staled_the_approval_before_the_sync_and_every_path\n        PASS [   0.259s] (3/8) atelier-cli::merge_gates sync_refuses_a_dirty_checkout_and_conflicts_with_check\n        PASS [   0.388s] (4/8) atelier-cli::merge_gates sync_reports_a_rerun_that_errors_with_a_code_and_still_reports_the_approval\n        PASS [   1.266s] (5/8) atelier-cli::merge_gates sync_agrees_with_check_when_the_base_already_holds_work_after_the_approval\n        PASS [   1.141s] (6/8) atelier-cli::merge_gates sync_records_a_failing_rerun_and_refuses\n        PASS [   1.728s] (7/8) atelier-cli::merge_gates sync_merges_the_base_reruns_current_receipts_and_reports_the_approval_kept\n        PASS [   1.575s] (8/8) atelier-cli::merge_gates sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution\n────────────\n     Summary [   2.670s] 8 tests run: 8 passed, 65 skipped\n```\n (rerun after merging master)"
updated_at: "2026-10-22T23:28:11.777182123+00:00"
---

## Summary

## Summary

merge --sync's approval report agrees with merge --check when the base already holds work after the approval, under both strategies (atelier-ns03)

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

Bytes: 1497
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.00s
────────────
 Nextest run ID c1d8e449-3661-4b8f-b753-d19e6e4ff2fd with nextest profile: default
    Starting 8 tests across 1 binary (65 tests skipped)
        PASS [   0.430s] (1/8) atelier-cli::merge_gates sync_lists_only_the_in_scope_record_a_clean_base_merge_changed
        PASS [   0.447s] (2/8) atelier-cli::merge_gates sync_names_the_commit_that_staled_the_approval_before_the_sync_and_every_path
        PASS [   0.259s] (3/8) atelier-cli::merge_gates sync_refuses_a_dirty_checkout_and_conflicts_with_check
        PASS [   0.388s] (4/8) atelier-cli::merge_gates sync_reports_a_rerun_that_errors_with_a_code_and_still_reports_the_approval
        PASS [   1.266s] (5/8) atelier-cli::merge_gates sync_agrees_with_check_when_the_base_already_holds_work_after_the_approval
        PASS [   1.141s] (6/8) atelier-cli::merge_gates sync_records_a_failing_rerun_and_refuses
        PASS [   1.728s] (7/8) atelier-cli::merge_gates sync_merges_the_base_reruns_current_receipts_and_reports_the_approval_kept
        PASS [   1.575s] (8/8) atelier-cli::merge_gates sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution
────────────
     Summary [   2.670s] 8 tests run: 8 passed, 65 skipped
```
 (rerun after merging master)

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

Bytes: 1552
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.87s
────────────
 Nextest run ID a982a219-8c9d-4da9-ac34-b0c74b1ab44a with nextest profile: default
    Starting 9 tests across 1 binary (65 tests skipped)
        PASS [   0.450s] (1/9) atelier-cli::merge_gates sync_lists_only_the_in_scope_record_a_clean_base_merge_changed
        PASS [   0.373s] (2/9) atelier-cli::merge_gates sync_names_the_commit_that_staled_the_approval_before_the_sync_and_every_path
        PASS [   1.192s] (3/9) atelier-cli::merge_gates sync_agrees_with_check_when_the_base_already_holds_work_after_the_approval
        PASS [   1.252s] (4/9) atelier-cli::merge_gates sync_keeps_the_approval_across_a_follow_up_linked_after_it
        PASS [   0.221s] (5/9) atelier-cli::merge_gates sync_refuses_a_dirty_checkout_and_conflicts_with_check
        PASS [   0.320s] (6/9) atelier-cli::merge_gates sync_reports_a_rerun_that_errors_with_a_code_and_still_reports_the_approval
        PASS [   1.609s] (7/9) atelier-cli::merge_gates sync_merges_the_base_reruns_current_receipts_and_reports_the_approval_kept
        PASS [   0.956s] (8/9) atelier-cli::merge_gates sync_records_a_failing_rerun_and_refuses
        PASS [   1.050s] (9/9) atelier-cli::merge_gates sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution
────────────
     Summary [   2.464s] 9 tests run: 9 passed, 65 skipped
```

