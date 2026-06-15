---
created_at: "2026-06-15T17:20:20.365305877+00:00"
id: "atelier-f9dh"
evidence_type: "validation"
captured_at: "2026-06-15T17:20:17.510458237+00:00"
command: "sh -c 'cargo test -p atelier-workflow && cargo nextest run -p atelier-cli test_issue_transition_options_and_successful_execution_follow_workflow_policy test_issue_transition_requires_workflow_policy_file && ! rg \"atelier::workflow_policy::STARTER_POLICY_YAML\" crates/atelier-cli crates/atelier-workflow'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-fjmw"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 445
    summary: "\nrunning 4 tests\ntest tests::starter_policy_lives_in_workflow_crate ... ok\ntest tests::status_categories_match_workflow_groups ... ok\ntest tests::transition_name_keeps_text ... ok\ntest tests::transition_name_rejects_empty_values ... ok\n\ntest result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n"
    truncated: false
  stderr:
    bytes: 857
    summary: "    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s\n     Running unittests src/lib.rs (target/debug/deps/atelier_workflow-833c343d904bf7d5)\n   Doc-tests atelier_workflow\n   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.14s\n────────────\n Nextest run ID 196239a3-ad30-4f5a-b5a5-ef8697fa532c with nextest profile: default\n    Starting 2 tests across 4 binaries (675 tests skipped)\n        PASS [   0.129s] (1/2) atelier-cli::cli_integration test_issue_transition_requires_workflow_policy_file\n        PASS [   0.246s] (2/2) atelier-cli::cli_integration test_issue_transition_options_and_successful_execution_follow_workflow_policy\n────────────\n     Summary [   0.247s] 2 tests run: 2 passed, 675 skipped\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fjmw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Starter workflow policy and status category semantics now live in atelier-workflow; CLI re-exports the starter policy during migration and integration tests use atelier_workflow::STARTER_POLICY_YAML directly; representative transition CLI tests pass."
updated_at: "2026-06-15T17:20:23.739938216+00:00"
---

Starter workflow policy and status category semantics now live in atelier-workflow; CLI re-exports the starter policy during migration and integration tests use atelier_workflow::STARTER_POLICY_YAML directly; representative transition CLI tests pass.

Command: sh -c 'cargo test -p atelier-workflow && cargo nextest run -p atelier-cli test_issue_transition_options_and_successful_execution_follow_workflow_policy test_issue_transition_requires_workflow_policy_file && ! rg "atelier::workflow_policy::STARTER_POLICY_YAML" crates/atelier-cli crates/atelier-workflow'
Exit status: 0

Stdout summary:

running 4 tests
test tests::starter_policy_lives_in_workflow_crate ... ok
test tests::status_categories_match_workflow_groups ... ok
test tests::transition_name_keeps_text ... ok
test tests::transition_name_rejects_empty_values ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Stderr summary:
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_workflow-833c343d904bf7d5)
   Doc-tests atelier_workflow
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.14s
────────────
 Nextest run ID 196239a3-ad30-4f5a-b5a5-ef8697fa532c with nextest profile: default
    Starting 2 tests across 4 binaries (675 tests skipped)
        PASS [   0.129s] (1/2) atelier-cli::cli_integration test_issue_transition_requires_workflow_policy_file
        PASS [   0.246s] (2/2) atelier-cli::cli_integration test_issue_transition_options_and_successful_execution_follow_workflow_policy
────────────
     Summary [   0.247s] 2 tests run: 2 passed, 675 skipped

