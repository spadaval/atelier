---
created_at: "2026-06-15T07:48:56.194825733+00:00"
id: "atelier-q1l7"
evidence_type: "test"
captured_at: "2026-06-15T07:48:55.718004168+00:00"
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
    bytes: 8138
    summary: "    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s\n────────────\n Nextest run ID e20ccee5-ee3e-421f-b354-cd3ca44cba98 with nextest profile: default\n    Starting 67 tests across 8 binaries\n        PASS [   0.008s] ( 1/67) atelier-core relationships::tests::sorting_relationships_deduplicates_each_value_set\n        PASS [   0.008s] ( 2/67) atelier-core models::tests::evidence_target_defaults_to_validates_role\n        PASS [   0.008s] ( 3/67) atelier-core::domain_invariants evidence_target_defaults_to_validation_role_at_domain_boundary\n        PASS [   0.009s] ( 4/67) atelier-records activity::tests::allocation_adds_deterministic_suffixes_for_same_timestamp_collisions\n        PASS [   0.009s] ( 5/67) atelier-records store::tests::issue_record_round_trips_explicit_closed_at_for_done_status\n        PASS [   0.009s] ( 6/67) atelier-records activity::tests::rejects_invalid_schema\n        PASS [   0.009s] ( 7/67) atelier-records activity::tests::write_refuses_to_overwrite_existing_activity\n        PASS [   0.010s] ( 8/67) atelier-records store::tests::issue_parser_contract_rejects_duplicate_recognized_headings\n        PASS [   0.012s] ( 9/67) atelier-core record_id::tests::validates_project_scoped_ids\n        PASS [   0.013s] (10/67) atelier-core record_id::tests::legacy_ids_are_project_scoped_base36\n        PASS [   0.017s] (11/67) atelier-records store::tests::issue_parser_reports_malformed_front_matter\n        PASS [   0.017s] (12/67) atelier-records store::tests::legacy_evidence_data_record_loads_into_typed_front_matter\n        PASS [   0.018s] (13/67) atelier-records activity::tests::lists_issue_activities_in_oldest_first_order\n        PASS [   0.010s] (14/67) atelier-records store::tests::mission_render_normalizes_legacy_evidence_attachments\n        PASS [   0.019s] (15/67) atelier-core::domain_invariants record_ids_are_project_scoped_and_base36\n        PASS [   0.019s] (16/67) atelier-records activity::tests::issue_activity_sidecar_path_is_canonical\n        PASS [   0.020s] (17/67) atelier-records activity::tests::timestamp_activity_id_uses_utc_microseconds\n        PASS [   0.012s] (18/67) atelier-records store::tests::milestone_record_renders_and_parses_deterministically_without_data_blob\n        PASS [   0.020s] (19/67) atelier-records store::tests::issue_parser_contract_rejects_content_before_first_recognized_heading\n        PASS [   0.020s] (20/67) atelier-records activity::tests::front_matter_and_body_round_trip\n        PASS [   0.013s] (21/67) atelier-records store::tests::legacy_plan_and_milestone_data_records_load_into_typed_front_matter\n        PASS [   0.021s] (22/67) atelier-records store::tests::issue_parser_contract_rejects_empty_present_sections\n        PASS [   0.013s] (23/67) atelier-records store::tests::plan_record_renders_and_parses_deterministically_without_data_blob\n        PASS [   0.021s] (24/67) atelier-records store::tests::evidence_record_renders_and_parses_deterministically_without_data_blob\n        PASS [   0.022s] (25/67) atelier-records store::tests::issue_parser_contract_accepts_sectioned_body_without_legacy_arrays\n        PASS [   0.022s] (26/67) atelier-core::domain_invariants relationship_sets_are_sorted_and_deduplicated_by_value\n        PASS [   0.022s] (27/67) atelier-records activity::tests::rejects_invalid_schema_version_subject_and_event_type\n        PASS [   0.012s] (28/67) atelier-records store::tests::record_store_create_issue_record_writes_markdown_and_parent_relationship\n        PASS [   0.015s] (29/67) atelier-records store::tests::issue_record_renders_and_parses_deterministically\n        PASS [   0.023s] (30/67) atelier-records store::tests::issue_parser_contract_rejects_missing_required_sections\n        PASS [   0.016s] (31/67) atelier-records store::tests::mission_record_renders_and_parses_deterministically_without_data_blob\n        PASS [   0.016s] (32/67) atelier-records store::tests::record_store_allocates_ids_across_canonical_dirs\n        PASS [   0.013s] (33/67) atelier-records store::tests::record_store_discovers_and_rejects_"
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
updated_at: "2026-06-15T07:48:59.963988921+00:00"
---

cargo nextest run -p atelier-core -p atelier-records -p atelier-workflow -p atelier-sqlite

Command: cargo nextest run -p atelier-core -p atelier-records -p atelier-workflow -p atelier-sqlite
Exit status: 0

Stdout summary:
(none)

Stderr summary (truncated):
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
────────────
 Nextest run ID e20ccee5-ee3e-421f-b354-cd3ca44cba98 with nextest profile: default
    Starting 67 tests across 8 binaries
        PASS [   0.008s] ( 1/67) atelier-core relationships::tests::sorting_relationships_deduplicates_each_value_set
        PASS [   0.008s] ( 2/67) atelier-core models::tests::evidence_target_defaults_to_validates_role
        PASS [   0.008s] ( 3/67) atelier-core::domain_invariants evidence_target_defaults_to_validation_role_at_domain_boundary
        PASS [   0.009s] ( 4/67) atelier-records activity::tests::allocation_adds_deterministic_suffixes_for_same_timestamp_collisions
        PASS [   0.009s] ( 5/67) atelier-records store::tests::issue_record_round_trips_explicit_closed_at_for_done_status
        PASS [   0.009s] ( 6/67) atelier-records activity::tests::rejects_invalid_schema
        PASS [   0.009s] ( 7/67) atelier-records activity::tests::write_refuses_to_overwrite_existing_activity
        PASS [   0.010s] ( 8/67) atelier-records store::tests::issue_parser_contract_rejects_duplicate_recognized_headings
        PASS [   0.012s] ( 9/67) atelier-core record_id::tests::validates_project_scoped_ids
        PASS [   0.013s] (10/67) atelier-core record_id::tests::legacy_ids_are_project_scoped_base36
        PASS [   0.017s] (11/67) atelier-records store::tests::issue_parser_reports_malformed_front_matter
        PASS [   0.017s] (12/67) atelier-records store::tests::legacy_evidence_data_record_loads_into_typed_front_matter
        PASS [   0.018s] (13/67) atelier-records activity::tests::lists_issue_activities_in_oldest_first_order
        PASS [   0.010s] (14/67) atelier-records store::tests::mission_render_normalizes_legacy_evidence_attachments
        PASS [   0.019s] (15/67) atelier-core::domain_invariants record_ids_are_project_scoped_and_base36
        PASS [   0.019s] (16/67) atelier-records activity::tests::issue_activity_sidecar_path_is_canonical
        PASS [   0.020s] (17/67) atelier-records activity::tests::timestamp_activity_id_uses_utc_microseconds
        PASS [   0.012s] (18/67) atelier-records store::tests::milestone_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.020s] (19/67) atelier-records store::tests::issue_parser_contract_rejects_content_before_first_recognized_heading
        PASS [   0.020s] (20/67) atelier-records activity::tests::front_matter_and_body_round_trip
        PASS [   0.013s] (21/67) atelier-records store::tests::legacy_plan_and_milestone_data_records_load_into_typed_front_matter
        PASS [   0.021s] (22/67) atelier-records store::tests::issue_parser_contract_rejects_empty_present_sections
        PASS [   0.013s] (23/67) atelier-records store::tests::plan_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.021s] (24/67) atelier-records store::tests::evidence_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.022s] (25/67) atelier-records store::tests::issue_parser_contract_accepts_sectioned_body_without_legacy_arrays
        PASS [   0.022s] (26/67) atelier-core::domain_invariants relationship_sets_are_sorted_and_deduplicated_by_value
        PASS [   0.022s] (27/67) atelier-records activity::tests::rejects_invalid_schema_version_subject_and_event_type
        PASS [   0.012s] (28/67) atelier-records store::tests::record_store_create_issue_record_writes_markdown_and_parent_relationship
        PASS [   0.015s] (29/67) atelier-records store::tests::issue_record_renders_and_parses_deterministically
        PASS [   0.023s] (30/67) atelier-records store::tests::issue_parser_contract_rejects_missing_required_sections
        PASS [   0.016s] (31/67) atelier-records store::tests::mission_record_renders_and_parses_deterministically_without_data_blob
        PASS [   0.016s] (32/67) atelier-records store::tests::record_store_allocates_ids_across_canonical_dirs
        PASS [   0.013s] (33/67) atelier-records store::tests::record_store_discovers_and_rejects_

