---
created_at: "2026-06-13T23:01:47.179726680+00:00"
id: "atelier-idru"
evidence_type: "test"
captured_at: "2026-06-13T23:01:38.889783015+00:00"
command: "cargo clippy --all-targets -- -A warnings -D dead_code -D unused_variables -D unused_imports -D unused_mut"
exit_status: "101"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-e723"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo clippy --all-targets -- -A warnings -D dead_code -D unused_variables -D unused_imports -D unused_mut"
updated_at: "2026-06-13T23:01:49.430786517+00:00"
---

cargo clippy --all-targets -- -A warnings -D dead_code -D unused_variables -D unused_imports -D unused_mut

Command: cargo clippy --all-targets -- -A warnings -D dead_code -D unused_variables -D unused_imports -D unused_mut
Exit status: 101

Stdout summary:
(none)

Stderr summary (truncated):
   Compiling atelier-tracker v0.2.0 (/root/atelier)
error: unused variable: `stdout`
    --> tests/cli_integration.rs:1703:23
     |
1703 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`
     |
     = note: requested on the command line with `-D unused-variables`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:1827:19
     |
1827 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:1880:19
     |
1880 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["abandon", &issue_id]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:1921:19
     |
1921 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:1944:19
     |
1944 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:1975:19
     |
1975 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:2001:23
     |
2001 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:3805:19
     |
3805 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "ready"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:3835:19
     |
3835 |     let (success, stdout, _stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:7150:19
     |
7150 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:7193:19
     |
7193 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:7244:19
     |
7244 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:7288:19
     |
7288 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:7305:19
     |
7305 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:7681:19
     |
7681 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

error: unused variable: `stdout`
    --> tests/cli_integration.rs:7774:19
     |
7774 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
     |                   ^^^^^^ help: if this is intentional, prefix it

