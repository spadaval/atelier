---
created_at: "2026-06-16T18:38:09.009386900+00:00"
id: "atelier-9fov"
evidence_type: "test"
captured_at: "2026-06-16T18:38:07.593818178+00:00"
command: "cargo test -p atelier-cli tree --lib -- --nocapture"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-qh52"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 1971
    summary: "\nrunning 12 tests\ntest commands::tree::tests::test_blocker_suffix_empty ... ok\ntest commands::tree::tests::test_blocker_suffix_names_drill_down ... ok\nNo issues found.\ntest commands::tree::tests::test_run_empty ... ok\nNo issues found.\n[ready] #atelier-72mp medium - Test issue\n[ready] #atelier-d6nm medium - Todo issue\n\nLegend: ready, blocked, active, review, validation, done, not-ready\n\nLegend: ready, blocked, active, review, validation, done, not-ready\ntest commands::tree::tests::test_run_single_issue ... ok\ntest commands::tree::tests::test_run_with_status_filter ... ok\n[ready] #atelier-7hvr high - Grandparent\n[ready] #atelier-atfp medium - Todo issue\n[done] #atelier-hb6b medium - Issue\n  [ready] #atelier-011g medium - Parent\n\nLegend: ready, blocked, active, review, validation, done, not-ready\n[done] #atelier-lnq2 medium - Done issue\n[ready] #atelier-8353 high - Parent\ntest commands::tree::tests::test_run_done_filter ... ok\n[ready] #atelier-xb1n high - Root\n    [ready] #atelier-8lcx low - Child\n\nLegend: ready, blocked, active, review, validation, done, not-ready\ntest commands::tree::tests::test_run_all_filter ... ok\n  [ready] #atelier-0d2s medium - Child\n  [ready] #atelier-s40i medium - Child 1\n\nLegend: ready, blocked, active, review, validation, done, not-ready\ntest commands::tree::tests::test_run_nested_hierarchy ... ok\n  [ready] #atelier-bgvw low - Child 2\n    [ready] #atelier-7i2t medium - Grandchild\ntest commands::tree::tests::test_filtered_subissues_excludes_done_children ... ok\n\nLegend: ready, blocked, active, review, validation, done, not-ready\n\nLegend: ready, blocked, active, review, validation, done, not-ready\ntest commands::tree::tests::test_run_accepts_empty_flat_and_hierarchical_trees ... ok\ntest commands::tree::tests::test_run_with_hierarchy ... ok\ntest commands::tree::tests::test_progress_summary_counts_mixed_statuses ... ok\n\ntest result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 158 filtered out; finished in 0.11s\n\n"
    truncated: false
  stderr:
    bytes: 219
    summary: "   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.23s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qh52"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Tree unit tests cover state labels, hierarchy recursion, filters, and blocker drill-down suffixes"
updated_at: "2026-06-16T18:38:12.726905550+00:00"
---

Tree unit tests cover state labels, hierarchy recursion, filters, and blocker drill-down suffixes

Command: cargo test -p atelier-cli tree --lib -- --nocapture
Exit status: 0

Stdout summary:

running 12 tests
test commands::tree::tests::test_blocker_suffix_empty ... ok
test commands::tree::tests::test_blocker_suffix_names_drill_down ... ok
No issues found.
test commands::tree::tests::test_run_empty ... ok
No issues found.
[ready] #atelier-72mp medium - Test issue
[ready] #atelier-d6nm medium - Todo issue

Legend: ready, blocked, active, review, validation, done, not-ready

Legend: ready, blocked, active, review, validation, done, not-ready
test commands::tree::tests::test_run_single_issue ... ok
test commands::tree::tests::test_run_with_status_filter ... ok
[ready] #atelier-7hvr high - Grandparent
[ready] #atelier-atfp medium - Todo issue
[done] #atelier-hb6b medium - Issue
  [ready] #atelier-011g medium - Parent

Legend: ready, blocked, active, review, validation, done, not-ready
[done] #atelier-lnq2 medium - Done issue
[ready] #atelier-8353 high - Parent
test commands::tree::tests::test_run_done_filter ... ok
[ready] #atelier-xb1n high - Root
    [ready] #atelier-8lcx low - Child

Legend: ready, blocked, active, review, validation, done, not-ready
test commands::tree::tests::test_run_all_filter ... ok
  [ready] #atelier-0d2s medium - Child
  [ready] #atelier-s40i medium - Child 1

Legend: ready, blocked, active, review, validation, done, not-ready
test commands::tree::tests::test_run_nested_hierarchy ... ok
  [ready] #atelier-bgvw low - Child 2
    [ready] #atelier-7i2t medium - Grandchild
test commands::tree::tests::test_filtered_subissues_excludes_done_children ... ok

Legend: ready, blocked, active, review, validation, done, not-ready

Legend: ready, blocked, active, review, validation, done, not-ready
test commands::tree::tests::test_run_accepts_empty_flat_and_hierarchical_trees ... ok
test commands::tree::tests::test_run_with_hierarchy ... ok
test commands::tree::tests::test_progress_summary_counts_mixed_statuses ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 158 filtered out; finished in 0.11s

Stderr summary:
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.23s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)

