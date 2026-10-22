---
created_at: "2026-10-22T22:37:18.239241179+00:00"
id: "atelier-hxgo"
evidence_type: "test"
commit: "13a1724be5ffce31d76cc6b8aa4d1a7968ef28ef"
result: "pass"
captured_at: "2026-10-22T22:37:01.954626427+00:00"
command: "cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(sync_) | test(a_recorded_command_splits)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
target:
  kind: "issue"
  id: "atelier-s2ei"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-s2ei"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "## Summary\n\nmerge --sync scenarios pass after the review fixes (6 sync_ tests plus split_command round trip)\n\n## Command\n\n```console\ncargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(sync_) | test(a_recorded_command_splits)'\n```\n\nExit status: 0\n\n## Stdout\n\nBytes: 0\nTruncated: no\n\n```text\n```\n\n## Stderr\n\nBytes: 1450\nTruncated: no\n\n```text\n   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-s2ei/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.15s\n────────────\n Nextest run ID b1e725d7-c036-4a12-af7b-4680164c993c with nextest profile: default\n    Starting 8 tests across 6 binaries (671 tests skipped)\n        PASS [   0.007s] (1/8) atelier-cli commands::evidence::tests::a_recorded_command_splits_back_into_the_arguments_it_was_written_from\n        PASS [   0.426s] (2/8) atelier-cli::merge_gates sync_lists_only_the_in_scope_record_a_clean_base_merge_changed\n        PASS [   0.429s] (3/8) atelier-cli::merge_gates sync_names_the_commit_that_staled_the_approval_before_the_sync_and_every_path\n        PASS [   0.299s] (4/8) atelier-cli::merge_gates sync_refuses_a_dirty_checkout_and_conflicts_with_check\n        PASS [   0.440s] (5/8) atelier-cli::merge_gates sync_reports_a_rerun_that_errors_with_a_code_and_still_reports_the_approval\n        PASS [   1.227s] (6/8) atelier-cli::merge_gates sync_records_a_failing_rerun_and_refuses\n        PASS [   1.724s] (7/8) atelier-cli::merge_gates sync_merges_the_base_reruns_current_receipts_and_reports_the_approval_kept\n        PASS [   1.167s] (8/8) atelier-cli::merge_gates sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution\n────────────\n     Summary [   1.894s] 8 tests run: 8 passed, 671 skipped\n```\n (rerun after merging master)"
updated_at: "2026-10-22T22:37:18.240431577+00:00"
---

## Summary

## Summary

merge --sync scenarios pass after the review fixes (6 sync_ tests plus split_command round trip)

## Command

```console
cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(sync_) | test(a_recorded_command_splits)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1450
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-s2ei/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.15s
────────────
 Nextest run ID b1e725d7-c036-4a12-af7b-4680164c993c with nextest profile: default
    Starting 8 tests across 6 binaries (671 tests skipped)
        PASS [   0.007s] (1/8) atelier-cli commands::evidence::tests::a_recorded_command_splits_back_into_the_arguments_it_was_written_from
        PASS [   0.426s] (2/8) atelier-cli::merge_gates sync_lists_only_the_in_scope_record_a_clean_base_merge_changed
        PASS [   0.429s] (3/8) atelier-cli::merge_gates sync_names_the_commit_that_staled_the_approval_before_the_sync_and_every_path
        PASS [   0.299s] (4/8) atelier-cli::merge_gates sync_refuses_a_dirty_checkout_and_conflicts_with_check
        PASS [   0.440s] (5/8) atelier-cli::merge_gates sync_reports_a_rerun_that_errors_with_a_code_and_still_reports_the_approval
        PASS [   1.227s] (6/8) atelier-cli::merge_gates sync_records_a_failing_rerun_and_refuses
        PASS [   1.724s] (7/8) atelier-cli::merge_gates sync_merges_the_base_reruns_current_receipts_and_reports_the_approval_kept
        PASS [   1.167s] (8/8) atelier-cli::merge_gates sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution
────────────
     Summary [   1.894s] 8 tests run: 8 passed, 671 skipped
```
 (rerun after merging master)

## Command

```console
cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(sync_) | test(a_recorded_command_splits)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1612
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/atelier-s2ei/crates/atelier-app)
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-s2ei/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 11.41s
────────────
 Nextest run ID 775a4129-4545-474a-a619-3b3c51009682 with nextest profile: default
    Starting 8 tests across 6 binaries (673 tests skipped)
        PASS [   0.007s] (1/8) atelier-cli commands::evidence::tests::a_recorded_command_splits_back_into_the_arguments_it_was_written_from
        PASS [   0.434s] (2/8) atelier-cli::merge_gates sync_names_the_commit_that_staled_the_approval_before_the_sync_and_every_path
        PASS [   0.445s] (3/8) atelier-cli::merge_gates sync_lists_only_the_in_scope_record_a_clean_base_merge_changed
        PASS [   0.235s] (4/8) atelier-cli::merge_gates sync_refuses_a_dirty_checkout_and_conflicts_with_check
        PASS [   0.378s] (5/8) atelier-cli::merge_gates sync_reports_a_rerun_that_errors_with_a_code_and_still_reports_the_approval
        PASS [   1.255s] (6/8) atelier-cli::merge_gates sync_records_a_failing_rerun_and_refuses
        PASS [   1.791s] (7/8) atelier-cli::merge_gates sync_merges_the_base_reruns_current_receipts_and_reports_the_approval_kept
        PASS [   1.270s] (8/8) atelier-cli::merge_gates sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution
────────────
     Summary [   1.940s] 8 tests run: 8 passed, 673 skipped
```

