---
created_at: "2026-06-15T18:52:33.649717334+00:00"
id: "atelier-l16n"
evidence_type: "validation"
captured_at: "2026-06-15T18:52:07.697100492+00:00"
command: "cargo nextest run"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-zwna"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 0
    summary: ""
    truncated: false
  stderr:
    bytes: 72310
    summary: "   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)\n   Compiling atelier-records v0.2.0 (/root/atelier/crates/atelier-records)\n   Compiling atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)\n   Compiling atelier-app v0.2.0 (/root/atelier/crates/atelier-app)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.53s\n────────────\n Nextest run ID 2a83eeb3-c277-458f-aad5-794c5b4ac68f with nextest profile: default\n    Starting 628 tests across 9 binaries (68 tests skipped)\n        PASS [   0.011s] (  1/628) atelier-app storage_layout::tests::canonical_dir_is_the_atelier_tree\n        PASS [   0.011s] (  2/628) atelier-app workflow_policy::tests::rejects_invalid_evidence_validator_params\n        PASS [   0.012s] (  3/628) atelier-app workflow_policy::tests::parses_valid_policy\n        PASS [   0.012s] (  4/628) atelier-app storage_layout::tests::rebuild_temp_database_paths_are_local_atelier_paths\n        PASS [   0.014s] (  5/628) atelier-app workflow_policy::tests::rejects_invalid_status_category\n        PASS [   0.012s] (  6/628) atelier-app workflow_policy::tests::rejects_unknown_template_variable\n        PASS [   0.011s] (  7/628) atelier-app workflow_policy::tests::rejects_unknown_validator_reference\n        PASS [   0.013s] (  8/628) atelier-app workflow_policy::tests::rejects_missing_issue_type_mapping\n        PASS [   0.010s] (  9/628) atelier-cli command_surface::tests::expands_slash_command_references\n        PASS [   0.014s] ( 10/628) atelier-app workflow_policy::tests::rejects_unknown_top_level_field\n        PASS [   0.033s] ( 11/628) atelier-app tests::app_entrypoint_returns_view_model_without_rendering\n        PASS [   0.041s] ( 12/628) atelier-app command_storage::tests::access_modes_declare_projection_freshness_policy\n        PASS [   0.020s] ( 13/628) atelier-cli command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections\n        PASS [   0.026s] ( 14/628) atelier-cli command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent\n        PASS [   0.122s] ( 15/628) atelier-app rebuild::tests::record_table_rejects_non_canonical_record_kinds\n        PASS [   0.136s] ( 16/628) atelier-app rebuild::tests::rebuild_reports_malformed_front_matter\n        PASS [   0.138s] ( 17/628) atelier-app export::tests::test_canonical_changed_record_export_rewrites_issue\n        PASS [   0.140s] ( 18/628) atelier-app export::tests::test_canonical_check_reports_stale_projection_metadata\n        PASS [   0.142s] ( 19/628) atelier-app export::tests::test_canonical_check_reports_invalid_duplicate_id\n        PASS [   0.146s] ( 20/628) atelier-app export::tests::test_canonical_export_removes_stale_record_file\n        PASS [   0.157s] ( 21/628) atelier-app export::tests::test_canonical_markdown_serialization_stability\n        PASS [   0.168s] ( 22/628) atelier-app rebuild::tests::rebuild_reports_schema_mismatch\n        PASS [   0.143s] ( 23/628) atelier-cli commands::comment::tests::test_add_comment_sql_injection\n        PASS [   0.131s] ( 24/628) atelier-cli commands::comment::tests::test_add_comment_to_nonexistent_issue\n        PASS [   0.173s] ( 25/628) atelier-app export::tests::test_canonical_issue_type_is_explicit_not_label_derived\n        PASS [   0.142s] ( 26/628) atelier-cli commands::comment::tests::test_add_comment_to_existing_issue\n        PASS [   0.180s] ( 27/628) atelier-app rebuild::tests::rebuild_rejects_activity_for_missing_issue\n        PASS [   0.183s] ( 28/628) atelier-app export::tests::test_canonical_noop_export_is_deterministic\n        PASS [   0.183s] ( 29/628) atelier-app export::tests::test_canonical_check_reports_dangling_link\n        PASS [   0.012s] ( 30/628) atelier-cli commands::comment::tests::test_validate_known_kinds\n        PASS [   0.160s] ( 31/628) atelier-cli commands::agent_factory::tests::subissue_summary_counts_statuses_and_priorities\n        PASS [   0.162s] ( 32/628) atelier-app rebuild::tests::rebuild_reports_dangling_dependency_and_duplicate_link\n        PASS [   0.012s] ( 33/628) atelier-cl"
    truncated: true
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zwna"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "cargo nextest run passes for thin CLI shell closeout"
updated_at: "2026-06-15T18:52:37.243805354+00:00"
---

cargo nextest run passes for thin CLI shell closeout

Command: cargo nextest run
Exit status: 0

Stdout summary:
(none)

Stderr summary (truncated):
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
   Compiling atelier-records v0.2.0 (/root/atelier/crates/atelier-records)
   Compiling atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.53s
────────────
 Nextest run ID 2a83eeb3-c277-458f-aad5-794c5b4ac68f with nextest profile: default
    Starting 628 tests across 9 binaries (68 tests skipped)
        PASS [   0.011s] (  1/628) atelier-app storage_layout::tests::canonical_dir_is_the_atelier_tree
        PASS [   0.011s] (  2/628) atelier-app workflow_policy::tests::rejects_invalid_evidence_validator_params
        PASS [   0.012s] (  3/628) atelier-app workflow_policy::tests::parses_valid_policy
        PASS [   0.012s] (  4/628) atelier-app storage_layout::tests::rebuild_temp_database_paths_are_local_atelier_paths
        PASS [   0.014s] (  5/628) atelier-app workflow_policy::tests::rejects_invalid_status_category
        PASS [   0.012s] (  6/628) atelier-app workflow_policy::tests::rejects_unknown_template_variable
        PASS [   0.011s] (  7/628) atelier-app workflow_policy::tests::rejects_unknown_validator_reference
        PASS [   0.013s] (  8/628) atelier-app workflow_policy::tests::rejects_missing_issue_type_mapping
        PASS [   0.010s] (  9/628) atelier-cli command_surface::tests::expands_slash_command_references
        PASS [   0.014s] ( 10/628) atelier-app workflow_policy::tests::rejects_unknown_top_level_field
        PASS [   0.033s] ( 11/628) atelier-app tests::app_entrypoint_returns_view_model_without_rendering
        PASS [   0.041s] ( 12/628) atelier-app command_storage::tests::access_modes_declare_projection_freshness_policy
        PASS [   0.020s] ( 13/628) atelier-cli command_surface::tests::extracts_visible_roots_without_removed_or_hidden_sections
        PASS [   0.026s] ( 14/628) atelier-cli command_surface::tests::obsolete_test_command_requires_metadata_or_negative_intent
        PASS [   0.122s] ( 15/628) atelier-app rebuild::tests::record_table_rejects_non_canonical_record_kinds
        PASS [   0.136s] ( 16/628) atelier-app rebuild::tests::rebuild_reports_malformed_front_matter
        PASS [   0.138s] ( 17/628) atelier-app export::tests::test_canonical_changed_record_export_rewrites_issue
        PASS [   0.140s] ( 18/628) atelier-app export::tests::test_canonical_check_reports_stale_projection_metadata
        PASS [   0.142s] ( 19/628) atelier-app export::tests::test_canonical_check_reports_invalid_duplicate_id
        PASS [   0.146s] ( 20/628) atelier-app export::tests::test_canonical_export_removes_stale_record_file
        PASS [   0.157s] ( 21/628) atelier-app export::tests::test_canonical_markdown_serialization_stability
        PASS [   0.168s] ( 22/628) atelier-app rebuild::tests::rebuild_reports_schema_mismatch
        PASS [   0.143s] ( 23/628) atelier-cli commands::comment::tests::test_add_comment_sql_injection
        PASS [   0.131s] ( 24/628) atelier-cli commands::comment::tests::test_add_comment_to_nonexistent_issue
        PASS [   0.173s] ( 25/628) atelier-app export::tests::test_canonical_issue_type_is_explicit_not_label_derived
        PASS [   0.142s] ( 26/628) atelier-cli commands::comment::tests::test_add_comment_to_existing_issue
        PASS [   0.180s] ( 27/628) atelier-app rebuild::tests::rebuild_rejects_activity_for_missing_issue
        PASS [   0.183s] ( 28/628) atelier-app export::tests::test_canonical_noop_export_is_deterministic
        PASS [   0.183s] ( 29/628) atelier-app export::tests::test_canonical_check_reports_dangling_link
        PASS [   0.012s] ( 30/628) atelier-cli commands::comment::tests::test_validate_known_kinds
        PASS [   0.160s] ( 31/628) atelier-cli commands::agent_factory::tests::subissue_summary_counts_statuses_and_priorities
        PASS [   0.162s] ( 32/628) atelier-app rebuild::tests::rebuild_reports_dangling_dependency_and_duplicate_link
        PASS [   0.012s] ( 33/628) atelier-cl

