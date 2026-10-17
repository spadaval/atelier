---
created_at: "2026-10-17T07:45:33.029868197+00:00"
id: "atelier-9mea"
evidence_type: "test"
commit: "a8e0b6d0d29ab7c236fc761517bbaf684348a950"
result: "pass"
captured_at: "2026-10-17T07:44:23.987094990+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-6xxb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-6xxb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after the review fixes (atelier-f780, atelier-jalb), including test_hints_name_real_ids_instead_of_placeholders with two ready/list candidates and both mission-parent cases"
updated_at: "2026-10-17T07:45:33.030637554+00:00"
---

## Summary

Full nextest suite passes after the review fixes (atelier-f780, atelier-jalb), including test_hints_name_real_ids_instead_of_placeholders with two ready/list candidates and both mission-parent cases

## Command

```console
cargo nextest run -j 4
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 89765
Truncated: yes

```text
   Compiling atelier-app v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-6xxb/crates/atelier-app)
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-6xxb/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 12.26s
────────────
 Nextest run ID 2d8d7ec3-6ef6-495c-9f58-562b2ec5e3ac with nextest profile: default
    Starting 711 tests across 10 binaries
        PASS [   2.922s] (  1/711) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   4.714s] (  2/711) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   4.830s] (  3/711) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.153s] (  4/711) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.047s] (  5/711) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   4.881s] (  6/711) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.005s] (  7/711) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.024s] (  8/711) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.028s] (  9/711) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.049s] ( 10/711) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.158s] ( 11/711) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.354s] ( 12/711) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.195s] ( 13/711) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   2.981s] ( 14/711) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.443s] ( 15/711) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   1.144s] ( 16/711) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   2.294s] ( 17/711) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   2.517s] ( 18/711) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   7.435s] ( 19/711) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        SLOW [> 10.000s] (───────) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        SLOW [> 10.000s] (───────) atelier-app git_freshness::tests::native_edges_match_git_diff_for_rename_configuration_and_weird_paths
        PASS [   9.735s] ( 20/711) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [  12.567s] ( 21/711) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   6.247s] ( 22/711) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.092s] ( 23/711) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.118s] ( 24/711) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.112s] ( 25/711) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0
```

