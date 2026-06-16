---
created_at: "2026-06-14T17:14:37.882287054+00:00"
id: "atelier-cjys"
evidence_type: "validation"
captured_at: "2026-06-14T17:14:32.633670503+00:00"
command: "cargo test --test cli_integration closeout"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-mllk"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-c4uz"
    role: "validates"
  - kind: "issue"
    id: "atelier-mllk"
    role: "validates"
  - kind: "issue"
    id: "atelier-wbed"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "focused closeout regression suite passes for shell mission and workflow approval paths"
updated_at: "2026-06-14T17:14:48.380230997+00:00"
---

## Summary

focused closeout regression suite passes for shell mission and workflow approval paths

## Command

```console
cargo test --test cli_integration closeout
```

Exit status: 0

## Stdout

Bytes: 1080
Truncated: no

```text

running 13 tests
test test_mission_status_help_exposes_closeout_drilldown ... ok
test test_issue_closeout_refuses_structurally_invalid_issue ... ok
test test_mission_closeout_blocks_undeferred_obsolete_command_test ... ok
test test_issue_closeout_requires_passing_evidence_records ... ok
test test_mission_status_names_concrete_closeout_blockers ... ok
test test_issue_closeout_uses_attached_pass_evidence_not_evidence_text ... ok
test test_validation_issue_closeout_uses_workflow_approval_not_contract_audit_terms ... ok
test test_issue_closeout_rejects_evidence_attached_to_another_issue ... ok
test test_dirty_worktree_blocks_mission_closeout ... ok
test test_mission_closeout_accepts_shell_mission_without_direct_mission_evidence ... ok
test test_mission_closeout_enforces_gates_and_reopen_skips_close_validators ... ok
test test_mission_status_shows_ignored_product_behavior_closeout_blocker ... ok
test test_mission_audit_reports_shell_closeout_and_explicit_approval ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 299 filtered out; finished in 2.65s
```

## Stderr

Bytes: 8980
Truncated: yes

```text
   Compiling atelier-tracker v0.2.0 (/root/atelier/.atelier-worktrees/atelier-c4uz)
warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2311:23
     |
2311 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2435:19
     |
2435 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2488:19
     |
2488 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["abandon", &issue_id]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2529:19
     |
2529 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2552:19
     |
2552 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2583:19
     |
2583 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2609:23
     |
2609 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:4602:19
     |
4602 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "ready"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:4632:19
     |
4632 |     let (success, stdout, _stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8257:19
     |
8257 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8300:19
     |
8300 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8408:19
     |
8408 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8819:19
     |
8819 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8962:19
     |
8962 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
     --> tests/cli_integration.rs:10692:19
      |
10692 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["export", "--check"]);
      |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
     --> tests/cli_integration.rs:10947:19
      |
10947 |
```
