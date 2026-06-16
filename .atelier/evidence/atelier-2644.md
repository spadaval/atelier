---
created_at: "2026-06-16T18:28:02.999953585+00:00"
id: "atelier-2644"
evidence_type: "test"
captured_at: "2026-06-16T18:27:58.274472887+00:00"
command: "bash -lc 'cargo test -p atelier-cli work_order -- --nocapture && cargo test -p atelier-cli test_issue_list_ -- --nocapture'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-nqjc"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 2031
    summary: "\nrunning 7 tests\ntest commands::work_order::tests::done_blockers_are_ignored_by_open_blocker_input ... ok\ntest commands::work_order::tests::hidden_blockers_do_not_insert_phantom_rows ... ok\ntest commands::work_order::tests::cycles_are_deterministic_and_preserve_all_rows ... ok\ntest commands::work_order::tests::orders_diamond_dependencies_before_join ... ok\ntest commands::work_order::tests::row_state_labels_cover_work_view_vocabulary ... ok\ntest commands::work_order::tests::orders_simple_visible_blocker_chain ... ok\ntest commands::work_order::tests::ties_use_priority_update_time_and_id ... ok\n\ntest result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 164 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 336 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 171 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n\nrunning 7 tests\ntest issues::test_issue_list_ready_rejects_closed_status ... ok\ntest issues::test_issue_list_orders_visible_blockers_before_blocked_rows ... ok\ntest issues::test_issue_list_blocked_replaces_blocked_helper ... ok\ntest issues::test_issue_list_marks_external_epic_blockers_by_id ... ok\ntest issues::test_issue_list_ready_marks_blocked_parent_headers_as_context ... ok\ntest issues::test_issue_list_ready_treats_internal_epic_blockers_as_ready ... ok\ntest issues::test_issue_list_ready_excludes_blocked_and_quiet_matches_human_order ... ok\n\ntest result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 329 filtered out; finished in 0.51s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s\n\n"
    truncated: false
  stderr:
    bytes: 946
    summary: "   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.90s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.76s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nqjc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Shared work_order and issue-list regression tests pass after subissue ordering changes."
updated_at: "2026-06-16T18:28:06.863910137+00:00"
---

Shared work_order and issue-list regression tests pass after subissue ordering changes.

Command: bash -lc 'cargo test -p atelier-cli work_order -- --nocapture && cargo test -p atelier-cli test_issue_list_ -- --nocapture'
Exit status: 0

Stdout summary:

running 7 tests
test commands::work_order::tests::done_blockers_are_ignored_by_open_blocker_input ... ok
test commands::work_order::tests::hidden_blockers_do_not_insert_phantom_rows ... ok
test commands::work_order::tests::cycles_are_deterministic_and_preserve_all_rows ... ok
test commands::work_order::tests::orders_diamond_dependencies_before_join ... ok
test commands::work_order::tests::row_state_labels_cover_work_view_vocabulary ... ok
test commands::work_order::tests::orders_simple_visible_blocker_chain ... ok
test commands::work_order::tests::ties_use_priority_update_time_and_id ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 164 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 336 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 171 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
test issues::test_issue_list_ready_rejects_closed_status ... ok
test issues::test_issue_list_orders_visible_blockers_before_blocked_rows ... ok
test issues::test_issue_list_blocked_replaces_blocked_helper ... ok
test issues::test_issue_list_marks_external_epic_blockers_by_id ... ok
test issues::test_issue_list_ready_marks_blocked_parent_headers_as_context ... ok
test issues::test_issue_list_ready_treats_internal_epic_blockers_as_ready ... ok
test issues::test_issue_list_ready_excludes_blocked_and_quiet_matches_human_order ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 329 filtered out; finished in 0.51s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s

Stderr summary:
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.90s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.76s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)

