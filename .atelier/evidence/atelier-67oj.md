---
created_at: "2026-06-14T17:16:20.837573544+00:00"
id: "atelier-67oj"
evidence_type: "validation"
captured_at: "2026-06-14T17:16:09.181816766+00:00"
command: "bash -lc 'cd /root/atelier/.atelier-worktrees/atelier-ux3k && cargo test test_mission_close_ignores_tracker_generated_issue_closeout_bookkeeping --test cli_integration && cargo test test_mission_close_still_blocks_hand_edited_issue_markdown --test cli_integration && cargo test test_dirty_worktree_blocks_mission_closeout --test cli_integration && git diff --check && atelier lint'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ux3k"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ux3k"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Focused closeout-validator proof from /root/atelier/.atelier-worktrees/atelier-ux3k: tracker-generated issue closeout bookkeeping no longer blocks mission close, hand-edited canonical issue markdown still blocks, dirty non-tracker files still block, and git diff --check plus atelier lint pass."
updated_at: "2026-06-14T17:16:22.378724723+00:00"
---

## Summary

Focused closeout-validator proof from /root/atelier/.atelier-worktrees/atelier-ux3k: tracker-generated issue closeout bookkeeping no longer blocks mission close, hand-edited canonical issue markdown still blocks, dirty non-tracker files still block, and git diff --check plus atelier lint pass.

## Command

```console
bash -lc 'cd /root/atelier/.atelier-worktrees/atelier-ux3k && cargo test test_mission_close_ignores_tracker_generated_issue_closeout_bookkeeping --test cli_integration && cargo test test_mission_close_still_blocks_hand_edited_issue_markdown --test cli_integration && cargo test test_dirty_worktree_blocks_mission_closeout --test cli_integration && git diff --check && atelier lint'
```

Exit status: 0

## Stdout

Bytes: 566
Truncated: no

```text

running 1 test
test test_mission_close_ignores_tracker_generated_issue_closeout_bookkeeping ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 320 filtered out; finished in 1.72s


running 1 test
test test_mission_close_still_blocks_hand_edited_issue_markdown ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 320 filtered out; finished in 1.32s


running 1 test
test test_dirty_worktree_blocks_mission_closeout ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 320 filtered out; finished in 0.45s

Lint passed.
```

## Stderr

Bytes: 26766
Truncated: yes

```text
   Compiling atelier-tracker v0.2.0 (/root/atelier/.atelier-worktrees/atelier-ux3k)
warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2384:23
     |
2384 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2508:19
     |
2508 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2561:19
     |
2561 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["abandon", &issue_id]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2602:19
     |
2602 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2625:19
     |
2625 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2656:19
     |
2656 |     let (success, stdout, stderr) =
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:2682:23
     |
2682 |         let (success, stdout, stderr) = run_atelier(dir.path(), &args);
     |                       ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:4675:19
     |
4675 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["issue", "ready"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:4705:19
     |
4705 |     let (success, stdout, _stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8330:19
     |
8330 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8373:19
     |
8373 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8482:19
     |
8482 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8525:19
     |
8525 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:8561:19
     |
8561 |     let (success, stdout, stderr) = run_atelier(
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:9143:19
     |
9143 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check"]);
     |                   ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stdout`

warning: unused variable: `stdout`
    --> tests/cli_integration.rs:9406:19
     |
9406 |     let (success, stdout, stderr) = run_atelier(dir.path(), &["workflow", "check
```
