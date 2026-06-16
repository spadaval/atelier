---
created_at: "2026-06-16T18:37:17.571349392+00:00"
id: "atelier-fq1t"
evidence_type: "test"
captured_at: "2026-06-16T18:37:14.542861744+00:00"
command: "cargo test -p atelier-cli work_order --lib -- --nocapture"
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
    bytes: 697
    summary: "\nrunning 7 tests\ntest commands::work_order::tests::done_blockers_are_ignored_by_open_blocker_input ... ok\ntest commands::work_order::tests::hidden_blockers_do_not_insert_phantom_rows ... ok\ntest commands::work_order::tests::orders_diamond_dependencies_before_join ... ok\ntest commands::work_order::tests::cycles_are_deterministic_and_preserve_all_rows ... ok\ntest commands::work_order::tests::row_state_labels_cover_work_view_vocabulary ... ok\ntest commands::work_order::tests::ties_use_priority_update_time_and_id ... ok\ntest commands::work_order::tests::orders_simple_visible_blocker_chain ... ok\n\ntest result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 163 filtered out; finished in 0.00s\n\n"
    truncated: false
  stderr:
    bytes: 432
    summary: "    Blocking waiting for file lock on package cache\n    Blocking waiting for file lock on package cache\n    Blocking waiting for file lock on package cache\n    Blocking waiting for file lock on artifact directory\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.97s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n"
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
title: "Shared work-order unit tests still pass"
updated_at: "2026-06-16T18:37:24.853838509+00:00"
---

Shared work-order unit tests still pass

Command: cargo test -p atelier-cli work_order --lib -- --nocapture
Exit status: 0

Stdout summary:

running 7 tests
test commands::work_order::tests::done_blockers_are_ignored_by_open_blocker_input ... ok
test commands::work_order::tests::hidden_blockers_do_not_insert_phantom_rows ... ok
test commands::work_order::tests::orders_diamond_dependencies_before_join ... ok
test commands::work_order::tests::cycles_are_deterministic_and_preserve_all_rows ... ok
test commands::work_order::tests::row_state_labels_cover_work_view_vocabulary ... ok
test commands::work_order::tests::ties_use_priority_update_time_and_id ... ok
test commands::work_order::tests::orders_simple_visible_blocker_chain ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 163 filtered out; finished in 0.00s

Stderr summary:
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on artifact directory
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.97s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)

