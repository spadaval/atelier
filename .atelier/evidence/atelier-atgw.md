---
created_at: "2026-06-14T07:10:44.287540178+00:00"
id: "atelier-atgw"
evidence_type: "test"
captured_at: "2026-06-14T07:10:34.219224134+00:00"
command: "cargo nextest run --test cli_integration -E 'test(test_issue_closeout_rejects_evidence_attached_to_another_issue) | test(test_issue_closeout_requires_claim_specific_evidence) | test(test_validation_issue_closeout_requires_contract_audit_evidence) | test(test_evidence_record_help_shows_issue_targeted_manual_and_command_flows) | test(test_evidence_capture_records_command_metadata_and_attaches_targets) | test(test_mission_audit_reports_parent_proof_coverage_classifications)'"
exit_status: "100"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-bqau"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 0
    summary: ""
    truncated: false
  stderr:
    bytes: 9718
    summary: "    Blocking waiting for file lock on artifact directory\n   Compiling atelier-tracker v0.2.0 (/root/atelier)\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:1857:23\n     |\n1857 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);\n     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n     |\n     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:1981:19\n     |\n1981 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2034:19\n     |\n2034 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"abandon\", &issue_id]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2075:19\n     |\n2075 |     let (success, stdout, stderr) =\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2098:19\n     |\n2098 |     let (success, stdout, stderr) =\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2129:19\n     |\n2129 |     let (success, stdout, stderr) =\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2155:23\n     |\n2155 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);\n     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:4023:19\n     |\n4023 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"issue\", \"ready\"]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:4053:19\n     |\n4053 |     let (success, stdout, _stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:7387:19\n     |\n7387 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:7430:19\n     |\n7430 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:7539:19\n     |\n7539 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:7582:19\n     |\n7582 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:7600:19\n     |\n7600 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:8204:19\n     |\n8204 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"workflow\", \"check\"]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:8347:19\n     |\n8347 |     let (success, stdout, stderr) = run_atelier(dir.pat"
    truncated: true
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-bqau"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "fail"
title: "cargo nextest run --test cli_integration -E 'test(test_issue_closeout_rejects_evidence_attached_to_another_issue) | test(test_issue_closeout_requires_claim_specific_evidence) | test(test_validation_issue_closeout_requires_contract_audit_evidence) | test(test_evidence_record_help_shows_issue_targeted_manual_and_command_flows) | test(test_evidence_capture_records_command_metadata_and_attaches_targets) | test(test_mission_audit_reports_parent_proof_coverage_classifications)'"
updated_at: "2026-06-14T07:10:45.781663156+00:00"
---

cargo nextest run --test cli_integration -E 'test(test_issue_closeout_rejects_evidence_attached_to_another_issue) | test(test_issue_closeout_requires_claim_specific_evidence) | test(test_validation_issue_closeout_requires_contract_audit_evidence) | test(test_evidence_record_help_shows_issue_targeted_manual_and_command_flows) | test(test_evidence_capture_records_command_metadata_and_attaches_targets) | test(test_mission_audit_reports_parent_proof_coverage_classifications)'

Command: cargo nextest run --test cli_integration -E 'test(test_issue_closeout_rejects_evidence_attached_to_another_issue) | test(test_issue_closeout_requires_claim_specific_evidence) | test(test_validation_issue_closeout_requires_contract_audit_evidence) | test(test_evidence_record_help_shows_issue_targeted_manual_and_command_flows) | test(test_evidence_capture_records_command_metadata_and_attaches_targets) | test(test_mission_audit_reports_parent_proof_coverage_classifications)'
Exit status: 100

Stdout summary:
(none)

Stderr summary (truncated):
    Blocking waiting for file lock on artifact directory
   Compiling atelier-tracker v0.2.0 (/root/atelier)
warning: unused variable: `stdout`
    --> tests/cli_integration.rs:1857:23
     |
1857 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:1981:19
     |
1981 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2034:19
     |
2034 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["abandon", &issue_id]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2075:19
     |
2075 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2098:19
     |
2098 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2129:19
     |
2129 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2155:23
     |
2155 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:4023:19
     |
4023 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "ready"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:4053:19
     |
4053 |     let (success, stdout, _stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:7387:19
     |
7387 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:7430:19
     |
7430 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:7539:19
     |
7539 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:7582:19
     |
7582 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:7600:19
     |
7600 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8204:19
     |
8204 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8347:19
     |
8347 |     let (success, stdout, stderr) = run_atelier(dir.pat

