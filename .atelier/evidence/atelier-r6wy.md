---
created_at: "2026-06-15T07:40:33.320435481+00:00"
id: "atelier-r6wy"
evidence_type: "test"
captured_at: "2026-06-15T07:40:33.149342678+00:00"
command: "cargo test -p atelier-workflow"
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
    bytes: 590
    summary: "\nrunning 7 tests\ntest tests::rejects_missing_issue_type_mapping ... ok\ntest tests::rejects_invalid_evidence_validator_params ... ok\ntest tests::parses_valid_policy ... ok\ntest tests::rejects_invalid_status_category ... ok\ntest tests::rejects_unknown_validator_reference ... ok\ntest tests::rejects_unknown_template_variable ... ok\ntest tests::rejects_unknown_top_level_field ... ok\n\ntest result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n"
    truncated: false
  stderr:
    bytes: 191
    summary: "    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s\n     Running unittests src/lib.rs (target/debug/deps/atelier_workflow-f1f084596002ed90)\n   Doc-tests atelier_workflow\n"
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
title: "atelier-workflow owns workflow policy parsing and validation; root workflow module is a narrow adapter and focused crate tests pass."
updated_at: "2026-06-15T07:40:36.598868754+00:00"
---

atelier-workflow owns workflow policy parsing and validation; root workflow module is a narrow adapter and focused crate tests pass.

Command: cargo test -p atelier-workflow
Exit status: 0

Stdout summary:

running 7 tests
test tests::rejects_missing_issue_type_mapping ... ok
test tests::rejects_invalid_evidence_validator_params ... ok
test tests::parses_valid_policy ... ok
test tests::rejects_invalid_status_category ... ok
test tests::rejects_unknown_validator_reference ... ok
test tests::rejects_unknown_template_variable ... ok
test tests::rejects_unknown_top_level_field ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Stderr summary:
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/atelier_workflow-f1f084596002ed90)
   Doc-tests atelier_workflow

