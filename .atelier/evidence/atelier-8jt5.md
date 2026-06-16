---
created_at: "2026-06-16T17:41:43.177338828+00:00"
id: "atelier-8jt5"
evidence_type: "test"
captured_at: "2026-06-16T17:41:37.148760681+00:00"
command: "bash -lc 'cargo test -p atelier-cli --test smoke_tests test_canonical_export_check_cli -- --nocapture && cargo test -p atelier-cli --test cli_integration test_top_level_help_only_shows_core_commands -- --nocapture && cargo test -p atelier-cli --test cli_integration test_root_status_summarizes_checkout_orientation -- --nocapture && cargo test -p atelier-app export:: -- --nocapture'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-vuqb"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 1447
    summary: "\nrunning 1 test\ntest smoke::cli_data::test_canonical_export_check_cli ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 51 filtered out; finished in 0.20s\n\n\nrunning 1 test\ntest setup_guidance::test_top_level_help_only_shows_core_commands ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.01s\n\n\nrunning 1 test\ntest setup_guidance::test_root_status_summarizes_checkout_orientation ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.24s\n\n\nrunning 10 tests\ntest export::tests::test_canonical_noop_export_is_deterministic ... ok\ntest export::tests::test_canonical_check_reports_invalid_duplicate_id ... ok\ntest export::tests::test_canonical_issue_type_is_explicit_not_label_derived ... ok\ntest export::tests::test_canonical_check_reports_stale_projection_metadata ... ok\ntest export::tests::test_canonical_export_removes_stale_record_file ... ok\ntest export::tests::test_canonical_changed_record_export_rewrites_issue ... ok\ntest export::tests::test_canonical_check_ignores_sqlite_only_canonical_drift ... ok\ntest export::tests::test_canonical_markdown_serialization_stability ... ok\ntest export::tests::test_canonical_check_reports_dangling_link ... ok\ntest export::tests::test_canonical_export_preserves_issue_activity_sidecars ... ok\n\ntest result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 34 filtered out; finished in 0.13s\n\n"
    truncated: false
  stderr:
    bytes: 841
    summary: "   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.64s\n     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.65s\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.67s\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s\n     Running unittests src/lib.rs (target/debug/deps/atelier_app-b996d9a0f399a6f5)\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vuqb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "focused command-surface and export tests pass"
updated_at: "2026-06-16T17:41:48.046189474+00:00"
---

focused command-surface and export tests pass

Command: bash -lc 'cargo test -p atelier-cli --test smoke_tests test_canonical_export_check_cli -- --nocapture && cargo test -p atelier-cli --test cli_integration test_top_level_help_only_shows_core_commands -- --nocapture && cargo test -p atelier-cli --test cli_integration test_root_status_summarizes_checkout_orientation -- --nocapture && cargo test -p atelier-app export:: -- --nocapture'
Exit status: 0

Stdout summary:

running 1 test
test smoke::cli_data::test_canonical_export_check_cli ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 51 filtered out; finished in 0.20s


running 1 test
test setup_guidance::test_top_level_help_only_shows_core_commands ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.01s


running 1 test
test setup_guidance::test_root_status_summarizes_checkout_orientation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.24s


running 10 tests
test export::tests::test_canonical_noop_export_is_deterministic ... ok
test export::tests::test_canonical_check_reports_invalid_duplicate_id ... ok
test export::tests::test_canonical_issue_type_is_explicit_not_label_derived ... ok
test export::tests::test_canonical_check_reports_stale_projection_metadata ... ok
test export::tests::test_canonical_export_removes_stale_record_file ... ok
test export::tests::test_canonical_changed_record_export_rewrites_issue ... ok
test export::tests::test_canonical_check_ignores_sqlite_only_canonical_drift ... ok
test export::tests::test_canonical_markdown_serialization_stability ... ok
test export::tests::test_canonical_check_reports_dangling_link ... ok
test export::tests::test_canonical_export_preserves_issue_activity_sidecars ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 34 filtered out; finished in 0.13s

Stderr summary:
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.64s
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.65s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.67s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-b996d9a0f399a6f5)

