---
created_at: "2026-10-22T20:42:11.840837302+00:00"
id: "atelier-z7cy"
evidence_type: "test"
commit: "7e8bce656c7ecc7c26455f55c276603366eb1dc2"
result: "pass"
captured_at: "2026-10-22T20:41:28.938108202+00:00"
command: "env CARGO_TARGET_DIR=/home/supadava/.cache/atelier-tmp/isyf/foreign python3 -I /home/supadava/.cache/atelier-tmp/isyf/run.py /home/supadava/.cache/atelier-tmp/isyf/tree"
exit_status: "0"
agent_identity: "claude-implementer-isyf"
target:
  kind: "issue"
  id: "atelier-isyf"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-isyf"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "All five merge_checks commands, extracted from .atelier/workflow.yaml and run in a detached checkout of the policy commit with CARGO_TARGET_DIR exported to a foreign directory, pass and leave the foreign directory uncreated"
updated_at: "2026-10-22T20:42:11.841244601+00:00"
---

## Summary

All five merge_checks commands, extracted from .atelier/workflow.yaml and run in a detached checkout of the policy commit with CARGO_TARGET_DIR exported to a foreign directory, pass and leave the foreign directory uncreated

## Command

```console
env CARGO_TARGET_DIR=/home/supadava/.cache/atelier-tmp/isyf/foreign python3 -I /home/supadava/.cache/atelier-tmp/isyf/run.py /home/supadava/.cache/atelier-tmp/isyf/tree
```

Exit status: 0

## Stdout

Bytes: 359
Truncated: no

```text
RUN git diff --check HEAD^ HEAD -- . ':!.atelier'
EXIT 0
RUN cargo fmt -- --check
EXIT 0
RUN cargo metadata --locked --format-version 1 > /dev/null
EXIT 0
RUN c=${XDG_CACHE_HOME:-$HOME/.cache}; mkdir -p "$c" && t=$(mktemp -d "$c/atelier-merge-XXXXXX
EXIT 0
RUN c=${XDG_CACHE_HOME:-$HOME/.cache}; mkdir -p "$c" && t=$(mktemp -d "$c/atelier-merge-XXXXXX
EXIT 0
```

## Stderr

Bytes: 205065
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.cache/atelier-tmp/isyf/tree/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.62s
────────────
 Nextest run ID ac4881ee-ce08-4795-8655-8283372faa4e with nextest profile: default
    Starting 816 tests across 10 binaries
        PASS [   0.013s] (  1/816) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.016s] (  2/816) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.056s] (  3/816) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.094s] (  4/816) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.130s] (  5/816) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.127s] (  6/816) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.143s] (  7/816) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.232s] (  8/816) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.114s] (  9/816) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.323s] ( 10/816) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.311s] ( 11/816) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.370s] ( 12/816) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.131s] ( 13/816) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.254s] ( 14/816) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.348s] ( 15/816) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.475s] ( 16/816) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.265s] ( 17/816) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.181s] ( 18/816) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.137s] ( 19/816) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.162s] ( 20/816) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.011s] ( 21/816) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.088s] ( 22/816) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.159s] ( 23/816) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.016s] ( 24/816) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.008s] ( 25/816) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.008s] ( 26/816) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.007s] ( 27/816) atelier-app issue_read::tests::health_is_done_only_when_nothing_in_scope_is_open
        PASS [   0.020s] ( 28/816) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.013s] ( 29/816) atelier-app issue_read:
```

