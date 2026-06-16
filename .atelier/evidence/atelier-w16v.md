---
created_at: "2026-06-16T19:01:18.602129782+00:00"
id: "atelier-w16v"
evidence_type: "validation"
captured_at: "2026-06-16T19:01:01.409113761+00:00"
command: "bash -lc 'cargo test -p atelier-cli work_order --lib -- --nocapture && cargo test -p atelier-cli test_issue_list_ -- --nocapture && cargo test -p atelier-cli test_mission_list_human_overview_orders_and_summarizes -- --nocapture && cargo test -p atelier-cli test_graph_tree_orders_children_by_visible_blockers -- --nocapture && cargo build -p atelier-cli && target/debug/atelier lint atelier-em15 && target/debug/atelier lint atelier-k1ga && target/debug/atelier doctor && cargo fmt -- --check && git diff --check'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-em15"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 3436
    summary: "\nrunning 7 tests\ntest commands::work_order::tests::cycles_are_deterministic_and_preserve_all_rows ... ok\ntest commands::work_order::tests::hidden_blockers_do_not_insert_phantom_rows ... ok\ntest commands::work_order::tests::done_blockers_are_ignored_by_open_blocker_input ... ok\ntest commands::work_order::tests::orders_diamond_dependencies_before_join ... ok\ntest commands::work_order::tests::row_state_labels_cover_work_view_vocabulary ... ok\ntest commands::work_order::tests::ties_use_priority_update_time_and_id ... ok\ntest commands::work_order::tests::orders_simple_visible_blocker_chain ... ok\n\ntest result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 163 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n\nrunning 7 tests\ntest issues::test_issue_list_ready_rejects_closed_status ... ok\ntest issues::test_issue_list_orders_visible_blockers_before_blocked_rows ... ok\ntest issues::test_issue_list_blocked_replaces_blocked_helper ... ok\ntest issues::test_issue_list_marks_external_epic_blockers_by_id ... ok\ntest issues::test_issue_list_ready_marks_blocked_parent_headers_as_context ... ok\ntest issues::test_issue_list_ready_treats_internal_epic_blockers_as_ready ... ok\ntest issues::test_issue_list_ready_excludes_blocked_and_quiet_matches_human_order ... ok\n\ntest result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 330 filtered out; finished in 0.92s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n\nrunning 1 test\ntest mission_projection_worktree::test_mission_list_human_overview_orders_and_summarizes ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 336 filtered out; finished in 4.01s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n\nrunning 1 test\ntest setup_guidance::test_graph_tree_orders_children_by_visible_blockers ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 336 filtered out; finished in 0.29s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s\n\nLint passed.\nLint passed.\nDatabase: /root/atelier/.atelier/runtime/state.db\nState: /root/atelier/.atelier\nInstall health:\n  config: ok\n  ignored_runtime_paths: ok\nProjection rebuild:\n  state_dir: ok\n  rebuild_ready: ok\n  projection_fresh: ok\n  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources\nCache health:\n  cache_dir: missing (optional)\n  projection_metadata: ok\nRuntime state:\n  directory: ok\n  database: ok\n  local_tables: ok\n  diagnostics: enabled\nCompatibility:\n  tables: \nLegacy health:\nconfig: ok\ndatabase: ok\nignore_rules: ok\nprojection_fresh: ok\nrebuild_ready: ok\nruntime_state: ok\nruntime_tables: ok\n"
    truncated: false
  stderr:
    bytes: 1777
    summary: "   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.20s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.75s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.81s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.72s\n     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)\n     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.58s\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-em15"
    role: "validates"
  - kind: "issue"
    id: "atelier-k1ga"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Focused regressions and local hygiene checks pass for blocker-aware ordering epic"
updated_at: "2026-06-16T19:02:45.204399204+00:00"
---

Focused regressions and local hygiene checks pass for blocker-aware ordering epic

Command: bash -lc 'cargo test -p atelier-cli work_order --lib -- --nocapture && cargo test -p atelier-cli test_issue_list_ -- --nocapture && cargo test -p atelier-cli test_mission_list_human_overview_orders_and_summarizes -- --nocapture && cargo test -p atelier-cli test_graph_tree_orders_children_by_visible_blockers -- --nocapture && cargo build -p atelier-cli && target/debug/atelier lint atelier-em15 && target/debug/atelier lint atelier-k1ga && target/debug/atelier doctor && cargo fmt -- --check && git diff --check'
Exit status: 0

Stdout summary:

running 7 tests
test commands::work_order::tests::cycles_are_deterministic_and_preserve_all_rows ... ok
test commands::work_order::tests::hidden_blockers_do_not_insert_phantom_rows ... ok
test commands::work_order::tests::done_blockers_are_ignored_by_open_blocker_input ... ok
test commands::work_order::tests::orders_diamond_dependencies_before_join ... ok
test commands::work_order::tests::row_state_labels_cover_work_view_vocabulary ... ok
test commands::work_order::tests::ties_use_priority_update_time_and_id ... ok
test commands::work_order::tests::orders_simple_visible_blocker_chain ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 163 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


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

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 330 filtered out; finished in 0.92s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mission_projection_worktree::test_mission_list_human_overview_orders_and_summarizes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 336 filtered out; finished in 4.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_graph_tree_orders_children_by_visible_blockers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 336 filtered out; finished in 0.29s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s

Lint passed.
Lint passed.
Database: /root/atelier/.atelier/runtime/state.db
State: /root/atelier/.atelier
Install health:
  config: ok
  ignored_runtime_paths: ok
Projection rebuild:
  state_dir: ok
  rebuild_ready: ok
  projection_fresh: ok
  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources
Cache health:
  cache_dir: missing (optional)
  projection_metadata: ok
Runtime state:
  directory: ok
  database: ok
  local_tables: ok
  diagnostics: enabled
Compatibility:
  tables: 
Legacy health:
config: ok
database: ok
ignore_rules: ok
projection_fresh: ok
rebuild_ready: ok
runtime_state: ok
runtime_tables: ok

Stderr summary:
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.20s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.75s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.81s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.72s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.58s

