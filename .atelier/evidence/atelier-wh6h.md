---
created_at: "2026-06-15T07:46:31.585892696+00:00"
id: "atelier-wh6h"
evidence_type: "test"
captured_at: "2026-06-15T07:46:31.056535346+00:00"
command: "cargo nextest run -p atelier-core -p atelier-records -p atelier-workflow -p atelier-sqlite"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-uz8g"
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
    bytes: 7918
    summary: "    Blocking waiting for file lock on package cache\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s\n────────────\n Nextest run ID 94dec696-05a1-4bb6-9275-9a7103442636 with nextest profile: default\n    Starting 65 tests across 8 binaries\n        PASS [   0.008s] ( 1/65) atelier-core record_id::tests::legacy_ids_are_project_scoped_base36\n        PASS [   0.009s] ( 2/65) atelier-records activity::tests::timestamp_activity_id_uses_utc_microseconds\n        PASS [   0.009s] ( 3/65) atelier-records activity::tests::lists_issue_activities_in_oldest_first_order\n        PASS [   0.009s] ( 4/65) atelier-records store::tests::issue_parser_reports_schema_and_path_mismatch\n        PASS [   0.010s] ( 5/65) atelier-core::domain_invariants record_ids_are_project_scoped_and_base36\n        PASS [   0.010s] ( 6/65) atelier-core relationships::tests::sorting_relationships_deduplicates_each_value_set\n        PASS [   0.011s] ( 7/65) atelier-records activity::tests::rejects_invalid_schema_version_subject_and_event_type\n        PASS [   0.011s] ( 8/65) atelier-records activity::tests::issue_activity_sidecar_path_is_canonical\n        PASS [   0.011s] ( 9/65) atelier-records activity::tests::allocation_adds_deterministic_suffixes_for_same_timestamp_collisions\n        PASS [   0.011s] (10/65) atelier-core::domain_invariants relationship_sets_are_sorted_and_deduplicated_by_value\n        PASS [   0.013s] (11/65) atelier-core::domain_invariants evidence_target_defaults_to_validation_role_at_domain_boundary\n        PASS [   0.017s] (12/65) atelier-records activity::tests::create_allocates_collision_suffix_and_does_not_overwrite\n        PASS [   0.010s] (13/65) atelier-records store::tests::mission_render_normalizes_legacy_evidence_attachments\n        PASS [   0.020s] (14/65) atelier-records activity::tests::front_matter_and_body_round_trip\n        PASS [   0.020s] (15/65) atelier-records store::tests::issue_parser_contract_rejects_legacy_acceptance_and_evidence_front_matter\n        PASS [   0.021s] (16/65) atelier-records activity::tests::rejects_invalid_schema\n        PASS [   0.021s] (17/65) atelier-records store::tests::issue_record_renders_and_parses_deterministically\n        PASS [   0.013s] (18/65) atelier-records store::tests::milestone_record_renders_and_parses_deterministically_without_data_blob\n        PASS [   0.021s] (19/65) atelier-records store::tests::issue_sections_report_shared_presence_state_and_search_text\n        PASS [   0.014s] (20/65) atelier-records store::tests::legacy_plan_and_milestone_data_records_load_into_typed_front_matter\n        PASS [   0.011s] (21/65) atelier-records store::tests::evidence_record_renders_and_parses_deterministically_without_data_blob\n        PASS [   0.023s] (22/65) atelier-records store::tests::issue_parser_contract_rejects_missing_required_sections\n        PASS [   0.023s] (23/65) atelier-records activity::tests::write_refuses_to_overwrite_existing_activity\n        PASS [   0.024s] (24/65) atelier-core models::tests::issue_serialization_preserves_domain_values\n        PASS [   0.015s] (25/65) atelier-records store::tests::mission_record_renders_and_parses_deterministically_without_data_blob\n        PASS [   0.024s] (26/65) atelier-core models::tests::evidence_target_defaults_to_validates_role\n        PASS [   0.014s] (27/65) atelier-records store::tests::issue_parser_contract_rejects_content_before_first_recognized_heading\n        PASS [   0.026s] (28/65) atelier-records store::tests::issue_parser_contract_accepts_sectioned_body_without_legacy_arrays\n        PASS [   0.026s] (29/65) atelier-records store::tests::issue_parser_contract_rejects_duplicate_recognized_headings\n        PASS [   0.018s] (30/65) atelier-records store::tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships\n        PASS [   0.010s] (31/65) atelier-records store::tests::workflow_validator_kind_is_registered_but_not_canonical_yet\n        PASS [   0.019s] (32/65) atelier-records store::tests::record_store_block_unblock_mutates_blocker_relationships\n        PA"
    truncated: true
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uz8g"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "cargo nextest run -p atelier-core -p atelier-records -p atelier-workflow -p atelier-sqlite"
updated_at: "2026-06-15T07:46:36.744315108+00:00"
---

cargo nextest run -p atelier-core -p atelier-records -p atelier-workflow -p atelier-sqlite

Command: cargo nextest run -p atelier-core -p atelier-records -p atelier-workflow -p atelier-sqlite
Exit status: 0

Stdout summary:
(none)

Stderr summary (truncated):
    Blocking waiting for file lock on package cache
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID 94dec696-05a1-4bb6-9275-9a7103442636 with nextest profile: default
    Starting 65 tests across 8 binaries
        PASS [   0.008s] ( 1/65) atelier-core record_id::tests::legacy_ids_are_project_scoped_base36
        PASS [   0.009s] ( 2/65) atelier-records activity::tests::timestamp_activity_id_uses_utc_microseconds
        PASS [   0.009s] ( 3/65) atelier-records activity::tests::lists_issue_activities_in_oldest_first_order
        PASS [   0.009s] ( 4/65) atelier-records store::tests::issue_parser_reports_schema_and_path_mismatch
        PASS [   0.010s] ( 5/65) atelier-core::domain_invariants record_ids_are_project_scoped_and_base36
        PASS [   0.010s] ( 6/65) atelier-core relationships::tests::sorting_relationships_deduplicates_each_value_set
        PASS [   0.011s] ( 7/65) atelier-records activity::tests::rejects_invalid_schema_version_subject_and_event_type
        PASS [   0.011s] ( 8/65) atelier-records activity::tests::issue_activity_sidecar_path_is_canonical
        PASS [   0.011s] ( 9/65) atelier-records activity::tests::allocation_adds_deterministic_suffixes_for_same_timestamp_collisions
        PASS [   0.011s] (10/65) atelier-core::domain_invariants relationship_sets_are_sorted_and_deduplicated_by_value
        PASS [   0.013s] (11/65) atelier-core::domain_invariants evidence_target_defaults_to_validation_role_at_domain_boundary
        PASS [   0.017s] (12/65) atelier-records activity::tests::create_allocates_collision_suffix_and_does_not_overwrite
        PASS [   0.010s] (13/65) atelier-records store::tests::mission_render_normalizes_legacy_evidence_attachments
        PASS [   0.020s] (14/65) atelier-records activity::tests::front_matter_and_body_round_trip
        PASS [   0.020s] (15/65) atelier-records store::tests::issue_parser_contract_rejects_legacy_acceptance_and_evidence_front_matter
        PASS [   0.021s] (16/65) atelier-records activity::tests::rejects_invalid_schema
        PASS [   0.021s] (17/65) atelier-records store::tests::issue_record_renders_and_parses_deterministically
        PASS [   0.013s] (18/65) atelier-records store::tests::milestone_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.021s] (19/65) atelier-records store::tests::issue_sections_report_shared_presence_state_and_search_text
        PASS [   0.014s] (20/65) atelier-records store::tests::legacy_plan_and_milestone_data_records_load_into_typed_front_matter
        PASS [   0.011s] (21/65) atelier-records store::tests::evidence_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.023s] (22/65) atelier-records store::tests::issue_parser_contract_rejects_missing_required_sections
        PASS [   0.023s] (23/65) atelier-records activity::tests::write_refuses_to_overwrite_existing_activity
        PASS [   0.024s] (24/65) atelier-core models::tests::issue_serialization_preserves_domain_values
        PASS [   0.015s] (25/65) atelier-records store::tests::mission_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.024s] (26/65) atelier-core models::tests::evidence_target_defaults_to_validates_role
        PASS [   0.014s] (27/65) atelier-records store::tests::issue_parser_contract_rejects_content_before_first_recognized_heading
        PASS [   0.026s] (28/65) atelier-records store::tests::issue_parser_contract_accepts_sectioned_body_without_legacy_arrays
        PASS [   0.026s] (29/65) atelier-records store::tests::issue_parser_contract_rejects_duplicate_recognized_headings
        PASS [   0.018s] (30/65) atelier-records store::tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships
        PASS [   0.010s] (31/65) atelier-records store::tests::workflow_validator_kind_is_registered_but_not_canonical_yet
        PASS [   0.019s] (32/65) atelier-records store::tests::record_store_block_unblock_mutates_blocker_relationships
        PA

