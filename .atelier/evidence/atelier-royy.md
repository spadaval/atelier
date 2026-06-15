---
created_at: "2026-06-15T07:21:34.716566005+00:00"
id: "atelier-royy"
evidence_type: "test"
captured_at: "2026-06-15T07:21:34.506938914+00:00"
command: "cargo test -p atelier-core"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-nbni"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 577
    summary: "\nrunning 5 tests\ntest models::tests::evidence_target_defaults_to_validates_role ... ok\ntest models::tests::issue_serialization_preserves_domain_values ... ok\ntest record_id::tests::legacy_ids_are_project_scoped_base36 ... ok\ntest record_id::tests::validates_project_scoped_ids ... ok\ntest relationships::tests::sorting_relationships_deduplicates_each_value_set ... ok\n\ntest result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n"
    truncated: false
  stderr:
    bytes: 183
    summary: "    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s\n     Running unittests src/lib.rs (target/debug/deps/atelier_core-8aedec801e3d2ef0)\n   Doc-tests atelier_core\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nbni"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier-core owns domain records, record-id validation, and relationship values; focused core tests pass and cargo metadata shows no SQLite, Clap, filesystem-layout, or telemetry dependency."
updated_at: "2026-06-15T07:21:37.947609574+00:00"
---

atelier-core owns domain records, record-id validation, and relationship values; focused core tests pass and cargo metadata shows no SQLite, Clap, filesystem-layout, or telemetry dependency.

Command: cargo test -p atelier-core
Exit status: 0

Stdout summary:

running 5 tests
test models::tests::evidence_target_defaults_to_validates_role ... ok
test models::tests::issue_serialization_preserves_domain_values ... ok
test record_id::tests::legacy_ids_are_project_scoped_base36 ... ok
test record_id::tests::validates_project_scoped_ids ... ok
test relationships::tests::sorting_relationships_deduplicates_each_value_set ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Stderr summary:
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (target/debug/deps/atelier_core-8aedec801e3d2ef0)
   Doc-tests atelier_core

