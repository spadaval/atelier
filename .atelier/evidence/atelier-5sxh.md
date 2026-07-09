---
created_at: "2026-07-06T17:55:33.310341918+00:00"
id: "atelier-5sxh"
evidence_type: "test"
captured_at: "2026-07-06T17:55:24.905917014+00:00"
command: "cargo nextest run -E 'test(/prune/) or test(squash_subject_without_equivalent_patch_does_not_prove_integration)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-w1z8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-w1z8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -E 'test(/prune/) or test(squash_subject_without_equivalent_patch_does_not_prove_integration)'"
updated_at: "2026-07-06T17:55:41.834525523+00:00"
---

## Summary

cargo nextest run -E 'test(/prune/) or test(squash_subject_without_equivalent_patch_does_not_prove_integration)'

## Command

```console
cargo nextest run -E 'test(/prune/) or test(squash_subject_without_equivalent_patch_does_not_prove_integration)'
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 3398
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-t3h3/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.21s
────────────
 Nextest run ID d3c6e533-fb18-4bd9-a72a-7458abba282a with nextest profile: default
    Starting 22 tests across 9 binaries (710 tests skipped)
        PASS [   0.020s] ( 1/22) atelier-app project_config::tests::rejects_unreasonable_prune_retention_days
        PASS [   0.024s] ( 2/22) atelier-app project_config::tests::parses_prune_canonical_retention_days
        PASS [   0.027s] ( 3/22) atelier-cli commands::prune::tests::failed_activity_sidecar_removal_is_not_reported_as_removed
        PASS [   0.032s] ( 4/22) atelier-cli::cli_integration setup_guidance::test_prune_dry_run_reports_diagnostics_without_removing_logs
        PASS [   0.058s] ( 5/22) atelier-cli::cli_integration setup_guidance::test_prune_apply_removes_only_expired_diagnostics_logs
        PASS [   0.079s] ( 6/22) atelier-cli::cli_integration setup_guidance::test_prune_help_describes_both_retention_classes
        PASS [   0.116s] ( 7/22) atelier-cli commands::prune::tests::branch_without_upstream_is_protected
        PASS [   0.319s] ( 8/22) atelier-cli commands::prune::tests::squash_subject_without_equivalent_patch_does_not_prove_integration
        PASS [   0.385s] ( 9/22) atelier-cli commands::prune::tests::branch_with_unpushed_commits_is_protected
        PASS [   0.751s] (10/22) atelier-cli::cli_integration setup_guidance::test_prune_protects_active_projection_rebuild_artifacts
        PASS [   0.841s] (11/22) atelier-cli::cli_integration setup_guidance::test_prune_protects_non_current_dirty_worktree
        PASS [   0.864s] (12/22) atelier-cli::cli_integration setup_guidance::test_prune_removes_ignored_orphaned_runtime_artifacts_but_protects_locks
        PASS [   0.888s] (13/22) atelier-cli::cli_integration setup_guidance::test_prune_protects_current_base_and_unowned_git_branches
        PASS [   1.898s] (14/22) atelier-cli::cli_integration setup_guidance::test_prune_dry_run_reports_terminal_canonical_issue_without_removing_it
        PASS [   1.883s] (15/22) atelier-cli::cli_integration setup_guidance::test_prune_uses_configured_canonical_retention_days_by_default
        PASS [   1.920s] (16/22) atelier-cli::cli_integration setup_guidance::test_prune_retention_flag_overrides_project_config_for_canonical_records
        PASS [   2.000s] (17/22) atelier-cli::cli_integration setup_guidance::test_prune_apply_refuses_canonical_cleanup_when_tracked_state_is_dirty
        PASS [   2.299s] (18/22) atelier-cli::cli_integration setup_guidance::test_prune_apply_removes_terminal_canonical_issue_and_keeps_git_recovery
        PASS [   2.620s] (19/22) atelier-cli::cli_integration setup_guidance::test_prune_protects_active_and_recent_terminal_owner_branches
        PASS [   2.877s] (20/22) atelier-cli::cli_integration setup_guidance::test_prune_protects_unmerged_and_unpushed_terminal_owner_branches
        PASS [   2.958s] (21/22) atelier-cli::cli_integration setup_guidance::test_prune_removes_merged_and_pushed_terminal_owner_branch_and_worktree
        PASS [   3.650s] (22/22) atelier-cli::cli_integration setup_guidance::test_prune_protects_old_evidence_attached_to_retained_issue
────────────
     Summary [   3.670s] 22 tests run: 22 passed, 710 skipped
```
