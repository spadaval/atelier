---
created_at: "2026-07-09T16:26:41.765098463+00:00"
id: "atelier-9d5v"
evidence_type: "test"
captured_at: "2026-07-09T16:26:38.104278060+00:00"
command: "cargo nextest run -p atelier-cli -E 'test(/prune/)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-t3h3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-f55w"
    role: "validates"
  - kind: "issue"
    id: "atelier-rgpl"
    role: "validates"
  - kind: "issue"
    id: "atelier-t3h3"
    role: "validates"
  - kind: "issue"
    id: "atelier-w1z8"
    role: "validates"
  - kind: "issue"
    id: "atelier-x3dy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli -E 'test(/prune/)'"
updated_at: "2026-07-09T16:27:44.009556747+00:00"
---

## Summary

cargo nextest run -p atelier-cli -E 'test(/prune/)'

## Command

```console
cargo nextest run -p atelier-cli -E 'test(/prune/)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 4021
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/94e2/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.44s
────────────
 Nextest run ID e8b5c87a-6fa9-4834-89c6-dad17c862c6d with nextest profile: default
    Starting 26 tests across 4 binaries (453 tests skipped)
        PASS [   0.010s] ( 1/26) atelier-cli commands::prune::tests::failed_activity_sidecar_removal_is_not_reported_as_removed
        PASS [   0.034s] ( 2/26) atelier-cli::cli_integration setup_guidance::test_prune_apply_removes_only_expired_diagnostics_logs
        PASS [   0.020s] ( 3/26) atelier-cli::cli_integration setup_guidance::test_prune_help_describes_both_retention_classes
        PASS [   0.035s] ( 4/26) atelier-cli::cli_integration setup_guidance::test_prune_dry_run_reports_diagnostics_without_removing_logs
        PASS [   0.066s] ( 5/26) atelier-cli commands::prune::tests::branch_without_upstream_is_protected
        PASS [   0.124s] ( 6/26) atelier-cli commands::prune::tests::historical_reverted_squash_patch_does_not_prove_later_integration
        PASS [   0.137s] ( 7/26) atelier-cli commands::prune::tests::squash_subject_without_equivalent_patch_does_not_prove_integration
        PASS [   0.210s] ( 8/26) atelier-cli commands::prune::tests::branch_with_unpushed_commits_is_protected
        PASS [   0.311s] ( 9/26) atelier-cli::cli_integration setup_guidance::test_prune_quiet_reports_compact_equivalent_counts
        PASS [   0.313s] (10/26) atelier-cli::cli_integration setup_guidance::test_prune_protects_aged_open_and_locked_cache_temp
        PASS [   0.327s] (11/26) atelier-cli::cli_integration setup_guidance::test_prune_protects_active_projection_rebuild_artifacts
        PASS [   0.327s] (12/26) atelier-cli::cli_integration setup_guidance::test_prune_protects_fresh_local_artifacts_and_preserves_cache_health
        PASS [   0.349s] (13/26) atelier-cli::cli_integration setup_guidance::test_prune_protects_current_base_and_unowned_git_branches
        PASS [   0.346s] (14/26) atelier-cli::cli_integration setup_guidance::test_prune_protects_non_current_dirty_worktree
        PASS [   0.376s] (15/26) atelier-cli::cli_integration setup_guidance::test_prune_protects_missing_locked_worktree_and_branch_without_inspecting_path
        PASS [   0.409s] (16/26) atelier-cli::cli_integration setup_guidance::test_prune_removes_ignored_orphaned_runtime_artifacts_but_protects_locks
        PASS [   0.466s] (17/26) atelier-cli::cli_integration setup_guidance::test_prune_uses_configured_canonical_retention_days_by_default
        PASS [   0.465s] (18/26) atelier-cli::cli_integration setup_guidance::test_prune_dry_run_reports_terminal_canonical_issue_without_removing_it
        PASS [   0.519s] (19/26) atelier-cli::cli_integration setup_guidance::test_prune_apply_refuses_canonical_cleanup_when_tracked_state_is_dirty
        PASS [   0.529s] (20/26) atelier-cli::cli_integration setup_guidance::test_prune_apply_removes_terminal_canonical_issue_and_keeps_git_recovery
        PASS [   0.533s] (21/26) atelier-cli::cli_integration setup_guidance::test_prune_retention_flag_overrides_project_config_for_canonical_records
        PASS [   0.563s] (22/26) atelier-cli::cli_integration setup_guidance::test_prune_protects_old_evidence_attached_to_retained_issue
        PASS [   0.626s] (23/26) atelier-cli::cli_integration setup_guidance::test_prune_protects_active_and_recent_terminal_owner_branches
        PASS [   0.704s] (24/26) atelier-cli::cli_integration setup_guidance::test_prune_protects_terminal_epic_branch_with_active_descendant
        PASS [   0.749s] (25/26) atelier-cli::cli_integration setup_guidance::test_prune_protects_unmerged_and_unpushed_terminal_owner_branches
        PASS [   0.921s] (26/26) atelier-cli::cli_integration setup_guidance::test_prune_removes_merged_and_pushed_terminal_owner_branch_and_worktree
────────────
     Summary [   0.922s] 26 tests run: 26 passed, 453 skipped
```

