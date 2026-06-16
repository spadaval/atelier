---
created_at: "2026-06-16T17:47:28.048787418+00:00"
id: "atelier-lvf3"
evidence_type: "test"
captured_at: "2026-06-16T17:47:21.224644126+00:00"
command: "bash -lc 'cargo test -p atelier-cli --test cli_integration test_doctor_fix_repairs_missing_and_stale_local_projection_state -- --nocapture && cargo test -p atelier-cli --test cli_integration test_doctor_fix_refuses_to_modify_malformed_canonical_records -- --nocapture && cargo test -p atelier-cli --test cli_integration test_projection_index_rejects_invalid_markdown_without_rebuild -- --nocapture'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-a7gd"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 629
    summary: "\nrunning 1 test\ntest setup_guidance::test_doctor_fix_repairs_missing_and_stale_local_projection_state ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.36s\n\n\nrunning 1 test\ntest setup_guidance::test_doctor_fix_refuses_to_modify_malformed_canonical_records ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.13s\n\n\nrunning 1 test\ntest mission_projection_worktree::test_projection_index_rejects_invalid_markdown_without_rebuild ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.23s\n\n"
    truncated: false
  stderr:
    bytes: 693
    summary: "   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.08s\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.70s\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.80s\n     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-a7gd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "doctor fix repairs local projection and refuses invalid canonical records"
updated_at: "2026-06-16T17:47:33.844664875+00:00"
---

doctor fix repairs local projection and refuses invalid canonical records

Command: bash -lc 'cargo test -p atelier-cli --test cli_integration test_doctor_fix_repairs_missing_and_stale_local_projection_state -- --nocapture && cargo test -p atelier-cli --test cli_integration test_doctor_fix_refuses_to_modify_malformed_canonical_records -- --nocapture && cargo test -p atelier-cli --test cli_integration test_projection_index_rejects_invalid_markdown_without_rebuild -- --nocapture'
Exit status: 0

Stdout summary:

running 1 test
test setup_guidance::test_doctor_fix_repairs_missing_and_stale_local_projection_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.36s


running 1 test
test setup_guidance::test_doctor_fix_refuses_to_modify_malformed_canonical_records ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.13s


running 1 test
test mission_projection_worktree::test_projection_index_rejects_invalid_markdown_without_rebuild ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 332 filtered out; finished in 0.23s

Stderr summary:
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.08s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.70s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.80s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)

