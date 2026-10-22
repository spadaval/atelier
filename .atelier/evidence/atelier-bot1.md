---
created_at: "2026-10-22T23:45:02.606386242+00:00"
id: "atelier-bot1"
evidence_type: "test"
commit: "5959474657df56a8fd49bab0b7215c7fb6a05268"
result: "pass"
captured_at: "2026-10-22T23:44:59.248003744+00:00"
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
title: "merge --sync reports agree with review.current per approval (ns03, 3nib), asserted on the review.current node"
updated_at: "2026-10-22T23:45:02.606766500+00:00"
---

## Summary

merge --sync reports agree with review.current per approval (ns03, 3nib), asserted on the review.current node

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

Bytes: 1687
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.19s
────────────
 Nextest run ID dc70b815-ab90-415b-b273-bb18ac09997b with nextest profile: default
    Starting 10 tests across 1 binary (65 tests skipped)
        PASS [   0.267s] ( 1/10) atelier-cli::merge_gates sync_lists_only_the_in_scope_record_a_clean_base_merge_changed
        PASS [   0.544s] ( 2/10) atelier-cli::merge_gates sync_keeps_only_approvals_that_are_current_on_their_own
        PASS [   0.263s] ( 3/10) atelier-cli::merge_gates sync_names_the_commit_that_staled_the_approval_before_the_sync_and_every_path
        PASS [   0.814s] ( 4/10) atelier-cli::merge_gates sync_agrees_with_check_when_the_base_already_holds_work_after_the_approval
        PASS [   0.849s] ( 5/10) atelier-cli::merge_gates sync_keeps_the_approval_across_a_follow_up_linked_after_it
        PASS [   0.165s] ( 6/10) atelier-cli::merge_gates sync_refuses_a_dirty_checkout_and_conflicts_with_check
        PASS [   0.245s] ( 7/10) atelier-cli::merge_gates sync_reports_a_rerun_that_errors_with_a_code_and_still_reports_the_approval
        PASS [   1.139s] ( 8/10) atelier-cli::merge_gates sync_merges_the_base_reruns_current_receipts_and_reports_the_approval_kept
        PASS [   0.786s] ( 9/10) atelier-cli::merge_gates sync_records_a_failing_rerun_and_refuses
        PASS [   0.921s] (10/10) atelier-cli::merge_gates sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution
────────────
     Summary [   1.901s] 10 tests run: 10 passed, 65 skipped
```

