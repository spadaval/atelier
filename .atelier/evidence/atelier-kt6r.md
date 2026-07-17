---
created_at: "2026-07-16T23:44:02.016787302+00:00"
id: "atelier-kt6r"
evidence_type: "validation"
captured_at: "2026-07-16T23:43:56.864581587+00:00"
command: "cargo nextest run --workspace -E 'test(/mission_plan_review|mission_plan_migration|mission_readiness|dependency_closure|ready_work_hides|transitive_dependency|bundle_apply|bundle_backup|bundle_stage|bundle_install|bundle_recovery|import_beads_exclusive/)'"
exit_status: "0"
agent_identity: "agent-factory.validate"
target:
  kind: "issue"
  id: "atelier-p2wk"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p2wk"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run --workspace -E 'test(/mission_plan_review|mission_plan_migration|mission_readiness|dependency_closure|ready_work_hides|transitive_dependency|bundle_apply|bundle_backup|bundle_stage|bundle_install|bundle_recovery|import_beads_exclusive/)'"
updated_at: "2026-07-16T23:44:02.019179739+00:00"
---

## Summary

cargo nextest run --workspace -E 'test(/mission_plan_review|mission_plan_migration|mission_readiness|dependency_closure|ready_work_hides|transitive_dependency|bundle_apply|bundle_backup|bundle_stage|bundle_install|bundle_recovery|import_beads_exclusive/)'

## Command

```console
cargo nextest run --workspace -E 'test(/mission_plan_review|mission_plan_migration|mission_readiness|dependency_closure|ready_work_hides|transitive_dependency|bundle_apply|bundle_backup|bundle_stage|bundle_install|bundle_recovery|import_beads_exclusive/)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 8840
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.28s
────────────
 Nextest run ID 10964c97-48c0-41fd-bd26-cfa5b00bbdde with nextest profile: default
    Starting 57 tests across 9 binaries (724 tests skipped)
        PASS [   0.011s] ( 1/57) atelier-cli commands::bundle::tests::bundle_recovery_scan_rejects_unsafe_stage_and_backup_types
        PASS [   0.016s] ( 2/57) atelier-app objective_graph::tests::dependency_closure_reports_complete_direct_and_transitive_paths
        PASS [   0.016s] ( 3/57) atelier-cli commands::bundle::tests::bundle_backup_allocator_never_reuses_or_removes_unsafe_collisions
        PASS [   0.016s] ( 4/57) atelier-cli commands::bundle::tests::bundle_install_rejects_symlink_and_special_file_destinations_without_touching_targets
        PASS [   0.019s] ( 5/57) atelier-app objective_graph::tests::dependency_closure_fails_safely_with_an_actionable_cycle_path
        PASS [   0.027s] ( 6/57) atelier-app mission_plan_migration::tests::target_workflow_preserves_custom_policy_and_installs_review_gate
        PASS [   0.037s] ( 7/57) atelier-app mission_plan_review::tests::invalid_graph_references_fail_before_append_and_leave_rebuild_valid
        PASS [   0.087s] ( 8/57) atelier-app mission_plan_migration::tests::collision_and_unknown_status_fail_without_changes
        PASS [   0.170s] ( 9/57) atelier-app mission_readiness::tests::configured_terminal_mission_with_scope_never_receives_execution_gate
        PASS [   0.191s] (10/57) atelier-cli::cli_integration mission_cache_worktree::test_bundle_apply_mid_apply_failure_leaves_canonical_files_unchanged
        PASS [   0.235s] (11/57) atelier-app mission_plan_migration::tests::concurrent_mutation_is_rejected_without_overwrite_or_activation
        PASS [   0.247s] (12/57) atelier-app mission_plan_migration::tests::no_active_mission_omits_empty_manifest
        PASS [   0.229s] (13/57) atelier-cli::cli_integration mission_plan_review_lifecycle::public_plan_review_requires_authenticated_actor_binding
        PASS [   0.274s] (14/57) atelier-cli::cli_integration mission_cache_worktree::test_bundle_apply_accepts_non_executable_initial_statuses
        PASS [   0.010s] (15/57) atelier-records mission_plan_review::tests::cutover_manifest_rejects_sub_microsecond_precision_before_receipt_hashing
        PASS [   0.024s] (16/57) atelier-records mission_plan_review::tests::direct_mission_blocked_by_change_stales_approval
        PASS [   0.014s] (17/57) atelier-records mission_plan_review::tests::every_material_edit_class_changes_revision_but_notes_and_order_do_not
        PASS [   0.324s] (18/57) atelier-app rebuild::tests::rebuild_preserves_complete_mission_plan_review_projection_deterministically
        PASS [   0.020s] (19/57) atelier-records mission_plan_review::tests::material_change_stales_approval_and_requires_attribution_for_new_revision
        PASS [   0.022s] (20/57) atelier-records mission_plan_review::tests::grandfather_requires_exact_cutover_status_and_migration_provenance
        PASS [   0.350s] (21/57) atelier-app mission_plan_migration::tests::writer_transaction_started_before_cutover_cannot_overwrite_post_cutover_state
        PASS [   0.010s] (22/57) atelier-records mission_plan_review::tests::rejects_default_ignorable_and_non_nfc_actor_aliases_before_independence
        PASS [   0.012s] (23/57) atelier-records mission_plan_review::tests::rejects_forged_late_duplicate_and_wrong_mission_grandfather_receipts
        PASS [   0.009s] (24/57) atelier-records mission_plan_review::tests::rejects_grandfather_receipt_with_wrong_revision_or_legacy_status
        PASS [   0.012s] (25/57) atelier-records mission_plan_review::tests::rejects_malformed_typed_activity_metadata
        PASS [   0.011s] (26/57) atelier-records mission_plan_review::tests::rejects_provenance_loss_non_independent_approval_and_unresolved_findings
        PASS [   0.013s] (27/57) atelier-records mission_plan_review::tests::rejects_whitespace_varia
```
