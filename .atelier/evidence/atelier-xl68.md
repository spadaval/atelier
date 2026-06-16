---
created_at: "2026-06-15T17:04:13.782936252+00:00"
id: "atelier-xl68"
evidence_type: "validation"
captured_at: "2026-06-15T17:04:09.533030277+00:00"
command: "cargo nextest run -p atelier-cli test_worktree_setup_failure_does_not_associate_and_can_retry test_work_lifecycle_human_output_and_guards test_root_repair_is_removed_and_does_not_clear_runtime_association test_separate_worktrees_can_have_different_active_issues"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-rjua"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rjua"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Worktree flow no longer uses runtime work_associations as source of truth: worktree status derives associated issues from codex/<issue-id> branch plus canonical issue status, worktree setup activates canonical issue state, and focused worktree tests pass."
updated_at: "2026-06-15T17:04:16.043987117+00:00"
---

## Summary

Worktree flow no longer uses runtime work_associations as source of truth: worktree status derives associated issues from codex/<issue-id> branch plus canonical issue status, worktree setup activates canonical issue state, and focused worktree tests pass.

## Command

```console
cargo nextest run -p atelier-cli test_worktree_setup_failure_does_not_associate_and_can_retry test_work_lifecycle_human_output_and_guards test_root_repair_is_removed_and_does_not_clear_runtime_association test_separate_worktrees_can_have_different_active_issues
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 885
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.21s
────────────
 Nextest run ID dc36b1d0-2e7e-4934-8ea1-22adb730648a with nextest profile: default
    Starting 4 tests across 4 binaries (673 tests skipped)
        PASS [   0.256s] (1/4) atelier-cli::cli_integration test_root_repair_is_removed_and_does_not_clear_runtime_association
        PASS [   0.842s] (2/4) atelier-cli::cli_integration test_worktree_setup_failure_does_not_associate_and_can_retry
        PASS [   1.688s] (3/4) atelier-cli::cli_integration test_work_lifecycle_human_output_and_guards
        PASS [   1.766s] (4/4) atelier-cli::cli_integration test_separate_worktrees_can_have_different_active_issues
────────────
     Summary [   1.768s] 4 tests run: 4 passed, 673 skipped
```
