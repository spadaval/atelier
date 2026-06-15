---
created_at: "2026-06-15T04:27:27.335435353+00:00"
id: "atelier-ps86"
evidence_type: "test"
captured_at: "2026-06-15T04:27:21.587535919+00:00"
command: "cargo test --test cli_integration test_work_lifecycle_human_output_and_guards -- --exact"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-l8r9"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 170
    summary: "\nrunning 1 test\ntest test_work_lifecycle_human_output_and_guards ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 322 filtered out; finished in 0.97s\n\n"
    truncated: false
  stderr:
    bytes: 9057
    summary: "    Blocking waiting for file lock on package cache\n    Blocking waiting for file lock on artifact directory\n   Compiling atelier-tracker v0.2.0 (/root/atelier)\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2442:23\n     |\n2442 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);\n     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n     |\n     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2566:19\n     |\n2566 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2619:19\n     |\n2619 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"abandon\", &issue_id]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2660:19\n     |\n2660 |     let (success, stdout, stderr) =\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2683:19\n     |\n2683 |     let (success, stdout, stderr) =\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2714:19\n     |\n2714 |     let (success, stdout, stderr) =\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:2740:23\n     |\n2740 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);\n     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:4733:19\n     |\n4733 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"issue\", \"ready\"]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:4763:19\n     |\n4763 |     let (success, stdout, _stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:8388:19\n     |\n8388 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:8431:19\n     |\n8431 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:8539:19\n     |\n8539 |     let (success, stdout, stderr) = run_atelier(\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:8950:19\n     |\n8950 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"workflow\", \"check\"]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n    --> tests/cli_integration.rs:9213:19\n     |\n9213 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"workflow\", \"check\"]);\n     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused variable: `stdout`\n     --> tests/cli_integration.rs:11062:19\n      |\n11062 |     let (success, stdout, stderr) = run_atelier(dir.path(), &[\"export\", \"--check\"]);\n      |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`\n\nwarning: unused "
    truncated: true
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-l8r9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "cargo test --test cli_integration test_work_lifecycle_human_output_and_guards -- --exact"
updated_at: "2026-06-15T04:27:30.242010094+00:00"
---

cargo test --test cli_integration test_work_lifecycle_human_output_and_guards -- --exact

Command: cargo test --test cli_integration test_work_lifecycle_human_output_and_guards -- --exact
Exit status: 0

Stdout summary:

running 1 test
test test_work_lifecycle_human_output_and_guards ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 322 filtered out; finished in 0.97s

Stderr summary (truncated):
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on artifact directory
   Compiling atelier-tracker v0.2.0 (/root/atelier)
warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2442:23
     |
2442 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2566:19
     |
2566 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2619:19
     |
2619 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["abandon", &issue_id]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2660:19
     |
2660 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2683:19
     |
2683 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2714:19
     |
2714 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2740:23
     |
2740 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:4733:19
     |
4733 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "ready"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:4763:19
     |
4763 |     let (success, stdout, _stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8388:19
     |
8388 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8431:19
     |
8431 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8539:19
     |
8539 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8950:19
     |
8950 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:9213:19
     |
9213 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
     --> tests/cli_integration.rs:11062:19
      |
11062 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["export", "--check"]);
      |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused

