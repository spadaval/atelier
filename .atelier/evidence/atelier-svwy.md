---
created_at: "2026-07-17T00:12:31.138296890+00:00"
id: "atelier-svwy"
evidence_type: "validation"
captured_at: "2026-07-17T00:12:26.178410180+00:00"
command: "cargo nextest run mission_plan --no-fail-fast"
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
title: "cargo nextest run mission_plan --no-fail-fast"
updated_at: "2026-07-17T00:12:31.140807680+00:00"
---

## Summary

cargo nextest run mission_plan --no-fail-fast

## Command

```console
cargo nextest run mission_plan --no-fail-fast
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 5929
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p4z2/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.31s
────────────
 Nextest run ID 48961c48-5e7a-46bb-b45e-12d417d081b3 with nextest profile: default
    Starting 37 tests across 9 binaries (744 tests skipped)
        PASS [   0.010s] ( 1/37) atelier-records mission_plan_review::tests::rejects_default_ignorable_and_non_nfc_actor_aliases_before_independence
        PASS [   0.010s] ( 2/37) atelier-records mission_plan_review::tests::rejects_grandfather_receipt_with_wrong_revision_or_legacy_status
        PASS [   0.027s] ( 3/37) atelier-records mission_plan_review::tests::cutover_manifest_rejects_sub_microsecond_precision_before_receipt_hashing
        PASS [   0.013s] ( 4/37) atelier-records mission_plan_review::tests::rejects_forged_late_duplicate_and_wrong_mission_grandfather_receipts
        PASS [   0.030s] ( 5/37) atelier-records mission_plan_review::tests::every_material_edit_class_changes_revision_but_notes_and_order_do_not
        PASS [   0.034s] ( 6/37) atelier-app mission_plan_migration::tests::target_workflow_preserves_custom_policy_and_installs_review_gate
        PASS [   0.035s] ( 7/37) atelier-records mission_plan_review::tests::material_change_stales_approval_and_requires_attribution_for_new_revision
        PASS [   0.016s] ( 8/37) atelier-records mission_plan_review::tests::rejects_malformed_typed_activity_metadata
        PASS [   0.022s] ( 9/37) atelier-records mission_plan_review::tests::direct_mission_blocked_by_change_stales_approval
        PASS [   0.012s] (10/37) atelier-records mission_plan_review::tests::rejects_provenance_loss_non_independent_approval_and_unresolved_findings
        PASS [   0.012s] (11/37) atelier-records mission_plan_review::tests::rejects_whitespace_variant_self_approval
        PASS [   0.046s] (12/37) atelier-records mission_plan_review::tests::grandfather_requires_exact_cutover_status_and_migration_provenance
        PASS [   0.065s] (13/37) atelier-app mission_plan_migration::tests::collision_and_unknown_status_fail_without_changes
        PASS [   0.052s] (14/37) atelier-app mission_plan_review::tests::invalid_graph_references_fail_before_append_and_leave_rebuild_valid
        PASS [   0.053s] (15/37) atelier-records mission_plan_review::tests::typed_events_round_trip_and_project_fresh_independent_approval
        PASS [   0.177s] (16/37) atelier-cli::cli_integration mission_plan_review_lifecycle::public_plan_review_requires_authenticated_actor_binding
        PASS [   0.185s] (17/37) atelier-app mission_plan_migration::tests::concurrent_mutation_is_rejected_without_overwrite_or_activation
        PASS [   0.209s] (18/37) atelier-app mission_plan_migration::tests::no_active_mission_omits_empty_manifest
        PASS [   0.286s] (19/37) atelier-app rebuild::tests::rebuild_preserves_complete_mission_plan_review_projection_deterministically
        PASS [   0.296s] (20/37) atelier-app mission_plan_migration::tests::writer_transaction_started_before_cutover_cannot_overwrite_post_cutover_state
        PASS [   0.383s] (21/37) atelier-cli::cli_integration mission_plan_review_lifecycle::forged_generic_transition_activity_cannot_authorize_stale_direct_start
        PASS [   0.403s] (22/37) atelier-app mission_plan_migration::tests::migrates_each_legacy_class_and_is_idempotent
        PASS [   0.540s] (23/37) atelier-cli::cli_integration mission_plan_review_lifecycle::review_approval_does_not_override_open_dependency_blockers
        PASS [   0.564s] (24/37) atelier-cli::cli_integration mission_plan_review_lifecycle::public_plan_review_surface_records_findings_resolutions_changes_and_approval
        PASS [   0.554s] (25/37) atelier-cli::cli_integration mission_plan_review_lifecycle::rework_does_not_retarget_stale_prior_approval
        PASS [   0.582s] (26/37) atelier-cli::cli_integration mission_plan_review_lifecycle::transitive_dependency_diagnosis_names_full_path_and_terminal_next_command
        PASS [   0.598s] (27/37) atelier-cli::cli_inte
```

