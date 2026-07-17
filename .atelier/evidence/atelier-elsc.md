---
created_at: "2026-07-17T00:12:43.149607497+00:00"
id: "atelier-elsc"
evidence_type: "validation"
captured_at: "2026-07-17T00:12:36.484915956+00:00"
command: "cargo nextest run dependency --no-fail-fast"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-t876"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t876"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run dependency --no-fail-fast"
updated_at: "2026-07-17T00:12:43.151476512+00:00"
---

## Summary

cargo nextest run dependency --no-fail-fast

## Command

```console
cargo nextest run dependency --no-fail-fast
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1961
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p4z2/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.63s
────────────
 Nextest run ID c552dd48-a8fe-4b4f-b768-e8549b53356b with nextest profile: default
    Starting 11 tests across 9 binaries (770 tests skipped)
        PASS [   0.009s] ( 1/11) atelier-app objective_graph::tests::dependency_closure_fails_safely_with_an_actionable_cycle_path
        PASS [   0.009s] ( 2/11) atelier-app objective_graph::tests::dependency_closure_reports_complete_direct_and_transitive_paths
        PASS [   0.095s] ( 3/11) atelier-cli commands::issue::tests::dependency_rows_include_context_and_open_blocker_marker
        PASS [   0.230s] ( 4/11) atelier-cli::cli_integration mission_cache_worktree::test_focused_lint_validates_dependency_cycles
        PASS [   0.326s] ( 5/11) atelier-cli::smoke_tests smoke::lifecycle::test_circular_dependency_prevented
        PASS [   0.377s] ( 6/11) atelier-cli::cli_integration mission_plan_review_lifecycle::review_approval_does_not_override_open_dependency_blockers
        PASS [   0.428s] ( 7/11) atelier-cli::cli_integration mission_plan_review_lifecycle::transitive_dependency_diagnosis_names_full_path_and_terminal_next_command
        PASS [   0.874s] ( 8/11) atelier-cli::cli_integration mission_cache_worktree::test_issue_ready_work_and_direct_start_require_transitive_dependency_closure
        PASS [   1.172s] ( 9/11) atelier-cli::smoke_tests smoke::lifecycle::test_dependency_chain_and_ready
        PASS [   2.151s] (10/11) atelier-cli::cli_integration mission_cache_worktree::test_mission_start_requires_cycle_safe_transitive_dependency_closure
        PASS [   3.599s] (11/11) atelier-app objective_graph::tests::batch_dependency_readiness_memoizes_shared_graph_for_large_candidate_set
────────────
     Summary [   3.601s] 11 tests run: 11 passed, 770 skipped
```
