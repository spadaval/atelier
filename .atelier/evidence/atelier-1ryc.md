---
created_at: "2026-06-16T17:50:51.185729123+00:00"
id: "atelier-1ryc"
evidence_type: "test"
captured_at: "2026-06-16T17:50:40.060955210+00:00"
command: "bash -lc 'cargo test -p atelier-cli --test smoke_tests test_canonical_export_check_cli -- --nocapture && cargo test -p atelier-cli --test cli_integration test_top_level_help_only_shows_core_commands -- --nocapture && cargo test -p atelier-cli --test cli_integration test_doctor_fix_repairs_missing_and_stale_local_projection_state -- --nocapture && cargo test -p atelier-cli --test cli_integration test_doctor_fix_refuses_to_modify_malformed_canonical_records -- --nocapture && cargo test -p atelier-cli --test cli_integration test_projection_index_rejects_invalid_markdown_without_rebuild -- --nocapture'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-m1r7"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 990
    summary: "\nrunning 1 test\ntest smoke::cli_data::test_canonical_export_check_cli ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 51 filtered out; finished in 0.22s\n\n\nrunning 1 test\ntest setup_guidance::test_top_level_help_only_shows_core_commands ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.01s\n\n\nrunning 1 test\ntest setup_guidance::test_doctor_fix_repairs_missing_and_stale_local_projection_state ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.40s\n\n\nrunning 1 test\ntest setup_guidance::test_doctor_fix_refuses_to_modify_malformed_canonical_records ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.13s\n\n\nrunning 1 test\ntest mission_projection_worktree::test_projection_index_rejects_invalid_markdown_without_rebuild ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.28s\n\n"
    truncated: false
  stderr:
    bytes: 1147
    summary: "   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.04s\n     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.71s\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.31s\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.68s\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.70s\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-m1r7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "focused command-surface recovery tests pass"
updated_at: "2026-06-16T17:50:58.608576179+00:00"
---

focused command-surface recovery tests pass

Command: bash -lc 'cargo test -p atelier-cli --test smoke_tests test_canonical_export_check_cli -- --nocapture && cargo test -p atelier-cli --test cli_integration test_top_level_help_only_shows_core_commands -- --nocapture && cargo test -p atelier-cli --test cli_integration test_doctor_fix_repairs_missing_and_stale_local_projection_state -- --nocapture && cargo test -p atelier-cli --test cli_integration test_doctor_fix_refuses_to_modify_malformed_canonical_records -- --nocapture && cargo test -p atelier-cli --test cli_integration test_projection_index_rejects_invalid_markdown_without_rebuild -- --nocapture'
Exit status: 0

Stdout summary:

running 1 test
test smoke::cli_data::test_canonical_export_check_cli ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 51 filtered out; finished in 0.22s


running 1 test
test setup_guidance::test_top_level_help_only_shows_core_commands ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.01s


running 1 test
test setup_guidance::test_doctor_fix_repairs_missing_and_stale_local_projection_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.40s


running 1 test
test setup_guidance::test_doctor_fix_refuses_to_modify_malformed_canonical_records ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.13s


running 1 test
test mission_projection_worktree::test_projection_index_rejects_invalid_markdown_without_rebuild ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.28s

Stderr summary:
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.04s
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.71s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.31s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.68s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.70s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)

