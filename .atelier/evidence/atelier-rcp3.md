---
created_at: "2026-06-14T06:42:41.317156896+00:00"
id: "atelier-rcp3"
evidence_type: "validation"
captured_at: "2026-06-14T06:42:34.256032847+00:00"
command: "cargo nextest run --test cli_integration test_evidence_record_help_shows_issue_targeted_manual_and_command_flows test_evidence_capture_records_command_metadata_and_attaches_targets"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 0
    summary: ""
    truncated: false
  stderr:
    bytes: 5669
    summary: "    Blocking waiting for file lock on package cache\n    Blocking waiting for file lock on package cache\n    Blocking waiting for file lock on artifact directory\n   Compiling atelier-tracker v0.2.0 (/root/atelier)\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:1808:23\n     |\n1808 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);\n     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n     |\n     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:1932:19\n     |\n1932 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:1985:19\n     |\n1985 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"abandon\", &issue_id]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2026:19\n     |\n2026 |     let (success, stdout, stderr) =\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2049:19\n     |\n2049 |     let (success, stdout, stderr) =\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2080:19\n     |\n2080 |     let (success, stdout, stderr) =\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2106:23\n     |\n2106 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);\n     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:3973:19\n     |\n3973 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"issue\", \"ready\"]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:4003:19\n     |\n4003 |     let (success, stdout, _stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:7337:19\n     |\n7337 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:7380:19\n     |\n7380 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:7489:19\n     |\n7489 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:7532:19\n     |\n7532 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:7550:19\n     |\n7550 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:7927:19\n     |\n7927 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"workflow\", \"check\"]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --"
    truncated: true
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uu2o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "evidence record help and targeted manual flow stay unified"
updated_at: "2026-06-14T06:42:42.932934409+00:00"
---

evidence record help and targeted manual flow stay unified

Command: cargo nextest run --test cli_integration test_evidence_record_help_shows_issue_targeted_manual_and_command_flows test_evidence_capture_records_command_metadata_and_attaches_targets
Exit status: 0

Stdout summary:
(none)

Stderr summary (truncated):
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on artifact directory
   Compiling atelier-tracker v0.2.0 (/root/atelier)
warning: unused variable: `stdout`
    --> tests/cli_integration.rs:1808:23
     |
1808 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:1932:19
     |
1932 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:1985:19
     |
1985 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["abandon", &issue_id]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2026:19
     |
2026 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2049:19
     |
2049 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2080:19
     |
2080 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2106:23
     |
2106 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:3973:19
     |
3973 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "ready"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:4003:19
     |
4003 |     let (success, stdout, _stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:7337:19
     |
7337 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:7380:19
     |
7380 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:7489:19
     |
7489 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:7532:19
     |
7532 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:7550:19
     |
7550 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:7927:19
     |
7927 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --

