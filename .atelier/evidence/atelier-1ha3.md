---
created_at: "2026-10-17T06:57:42.075530751+00:00"
id: "atelier-1ha3"
evidence_type: "test"
commit: "354f65160eb9b6402120c9fe88f9daf1986db2e6"
result: "pass"
captured_at: "2026-10-17T06:57:18.867041264+00:00"
command: "cargo nextest run"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-a76d"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-a76d"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes without clap's env feature"
updated_at: "2026-10-17T06:57:42.076223001+00:00"
---

## Summary

Full nextest suite passes without clap's env feature

## Command

```console
cargo nextest run
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 84839
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-a76d/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 10.38s
────────────
 Nextest run ID ac825b9a-f896-4524-8243-a5612c94572f with nextest profile: default
    Starting 678 tests across 10 binaries
        PASS [   0.023s] (  1/678) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.053s] (  2/678) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.088s] (  3/678) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.149s] (  4/678) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.167s] (  5/678) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.182s] (  6/678) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.143s] (  7/678) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.308s] (  8/678) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.410s] (  9/678) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.434s] ( 10/678) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.430s] ( 11/678) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.329s] ( 12/678) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.192s] ( 13/678) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.537s] ( 14/678) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.582s] ( 15/678) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.377s] ( 16/678) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.188s] ( 17/678) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.127s] ( 18/678) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.300s] ( 19/678) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.022s] ( 20/678) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.012s] ( 21/678) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.289s] ( 22/678) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.022s] ( 23/678) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.017s] ( 24/678) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.017s] ( 25/678) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.280s] ( 26/678) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.024s] ( 27/678) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.019s] ( 28/678) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.020s] ( 29/678) ate
```

