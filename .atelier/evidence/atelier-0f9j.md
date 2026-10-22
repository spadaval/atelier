---
created_at: "2026-10-22T22:22:28.112307066+00:00"
id: "atelier-0f9j"
evidence_type: "test"
commit: "0e27144c12e18dc3481177f476bd83b8b74d3501"
result: "pass"
captured_at: "2026-10-22T22:22:25.010636005+00:00"
command: "cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(sync_) | test(a_recorded_command_splits)'"
exit_status: "0"
agent_identity: "claude-implementer-s2ei"
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
title: "merge --sync scenarios pass: clean merge keeps approval, conflict stops and resumes, failing rerun refuses, under both strategies"
updated_at: "2026-10-22T22:22:28.112580195+00:00"
---

## Summary

merge --sync scenarios pass: clean merge keeps approval, conflict stops and resumes, failing rerun refuses, under both strategies

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

Bytes: 1065
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-s2ei/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.65s
────────────
 Nextest run ID 5c607c8a-86f7-4120-b388-9cdf14530276 with nextest profile: default
    Starting 5 tests across 6 binaries (671 tests skipped)
        PASS [   0.005s] (1/5) atelier-cli commands::evidence::tests::a_recorded_command_splits_back_into_the_arguments_it_was_written_from
        PASS [   0.160s] (2/5) atelier-cli::merge_gates sync_refuses_a_dirty_checkout_and_conflicts_with_check
        PASS [   0.736s] (3/5) atelier-cli::merge_gates sync_records_a_failing_rerun_and_refuses
        PASS [   0.809s] (4/5) atelier-cli::merge_gates sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution
        PASS [   1.174s] (5/5) atelier-cli::merge_gates sync_merges_the_base_reruns_current_receipts_and_reports_the_approval_kept
────────────
     Summary [   1.175s] 5 tests run: 5 passed, 671 skipped
```

