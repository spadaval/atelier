---
created_at: "2026-07-06T18:05:10.878894494+00:00"
id: "atelier-oxct"
evidence_type: "test"
captured_at: "2026-07-06T18:05:02.182248277+00:00"
command: "env CARGO_INCREMENTAL=0 cargo nextest run -E 'test(/prune/) or test(squash_subject_without_equivalent_patch_does_not_prove_integration) or test(historical_reverted_squash_patch_does_not_prove_later_integration)'"
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
title: "env CARGO_INCREMENTAL=0 cargo nextest run -E 'test(/prune/) or test(squash_subject_without_equivalent_patch_does_not_prove_integration) or test(historical_reverted_squash_patch_does_not_prove_later_integration)'"
updated_at: "2026-07-06T18:05:15.045824588+00:00"
---

## Summary

env CARGO_INCREMENTAL=0 cargo nextest run -E 'test(/prune/) or test(squash_subject_without_equivalent_patch_does_not_prove_integration) or test(historical_reverted_squash_patch_does_not_prove_later_integration)'

## Command

```console
env CARGO_INCREMENTAL=0 cargo nextest run -E 'test(/prune/) or test(squash_subject_without_equivalent_patch_does_not_prove_integration) or test(historical_reverted_squash_patch_does_not_prove_later_integration)'
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 3675
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-t3h3/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.14s
────────────
 Nextest run ID d35ec000-4587-4296-beee-01c88eba44c9 with nextest profile: default
    Starting 24 tests across 9 binaries (710 tests skipped)
        PASS [   0.010s] ( 1/24) atelier-app project_config::tests::rejects_unreasonable_prune_retention_days
        PASS [   0.017s] ( 2/24) atelier-cli::cli_integration setup_guidance::test_prune_help_describes_both_retention_classes
        PASS [   0.023s] ( 3/24) atelier-cli::cli_integration setup_guidance::test_prune_dry_run_reports_diagnostics_without_removing_logs
        PASS [   0.018s] ( 4/24) atelier-cli commands::prune::tests::failed_activity_sidecar_removal_is_not_reported_as_removed
        PASS [   0.030s] ( 5/24) atelier-app project_config::tests::parses_prune_canonical_retention_days
        PASS [   0.024s] ( 6/24) atelier-cli::cli_integration setup_guidance::test_prune_apply_removes_only_expired_diagnostics_logs
        PASS [   0.052s] ( 7/24) atelier-cli commands::prune::tests::branch_without_upstream_is_protected
        PASS [   0.120s] ( 8/24) atelier-cli commands::prune::tests::historical_reverted_squash_patch_does_not_prove_later_integration
        PASS [   0.149s] ( 9/24) atelier-cli commands::prune::tests::squash_subject_without_equivalent_patch_does_not_prove_integration
        PASS [   0.177s] (10/24) atelier-cli commands::prune::tests::branch_with_unpushed_commits_is_protected
        PASS [   0.330s] (11/24) atelier-cli::cli_integration setup_guidance::test_prune_protects_active_projection_rebuild_artifacts
        PASS [   0.354s] (12/24) atelier-cli::cli_integration setup_guidance::test_prune_protects_non_current_dirty_worktree
        PASS [   0.365s] (13/24) atelier-cli::cli_integration setup_guidance::test_prune_protects_current_base_and_unowned_git_branches
        PASS [   0.399s] (14/24) atelier-cli::cli_integration setup_guidance::test_prune_removes_ignored_orphaned_runtime_artifacts_but_protects_locks
        PASS [   0.687s] (15/24) atelier-cli::cli_integration setup_guidance::test_prune_dry_run_reports_terminal_canonical_issue_without_removing_it
        PASS [   0.690s] (16/24) atelier-cli::cli_integration setup_guidance::test_prune_uses_configured_canonical_retention_days_by_default
        PASS [   0.708s] (17/24) atelier-cli::cli_integration setup_guidance::test_prune_apply_refuses_canonical_cleanup_when_tracked_state_is_dirty
        PASS [   0.727s] (18/24) atelier-cli::cli_integration setup_guidance::test_prune_retention_flag_overrides_project_config_for_canonical_records
        PASS [   0.868s] (19/24) atelier-cli::cli_integration setup_guidance::test_prune_apply_removes_terminal_canonical_issue_and_keeps_git_recovery
        PASS [   0.938s] (20/24) atelier-cli::cli_integration setup_guidance::test_prune_protects_active_and_recent_terminal_owner_branches
        PASS [   1.119s] (21/24) atelier-cli::cli_integration setup_guidance::test_prune_protects_old_evidence_attached_to_retained_issue
        PASS [   1.132s] (22/24) atelier-cli::cli_integration setup_guidance::test_prune_protects_unmerged_and_unpushed_terminal_owner_branches
        PASS [   1.166s] (23/24) atelier-cli::cli_integration setup_guidance::test_prune_protects_terminal_epic_branch_with_active_descendant
        PASS [   1.240s] (24/24) atelier-cli::cli_integration setup_guidance::test_prune_removes_merged_and_pushed_terminal_owner_branch_and_worktree
────────────
     Summary [   1.242s] 24 tests run: 24 passed, 710 skipped
```
