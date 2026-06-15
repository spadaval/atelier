---
created_at: "2026-06-15T07:40:33.328394414+00:00"
id: "atelier-m75w"
evidence_type: "test"
captured_at: "2026-06-15T07:40:33.104970225+00:00"
command: "cargo test -p atelier-records"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-8wvr"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 3469
    summary: "\nrunning 39 tests\ntest activity::tests::issue_activity_sidecar_path_is_canonical ... ok\ntest activity::tests::allocation_adds_deterministic_suffixes_for_same_timestamp_collisions ... ok\ntest activity::tests::rejects_invalid_schema ... ok\ntest activity::tests::front_matter_and_body_round_trip ... ok\ntest activity::tests::timestamp_activity_id_uses_utc_microseconds ... ok\ntest activity::tests::lists_issue_activities_in_oldest_first_order ... ok\ntest store::tests::issue_parser_contract_accepts_sectioned_body_without_legacy_arrays ... ok\ntest store::tests::issue_parser_contract_rejects_legacy_acceptance_and_evidence_front_matter ... ok\ntest store::tests::issue_parser_contract_rejects_content_before_first_recognized_heading ... ok\ntest activity::tests::create_allocates_collision_suffix_and_does_not_overwrite ... ok\ntest store::tests::issue_parser_contract_rejects_duplicate_recognized_headings ... ok\ntest store::tests::issue_parser_contract_rejects_empty_present_sections ... ok\ntest activity::tests::write_refuses_to_overwrite_existing_activity ... ok\ntest store::tests::issue_parser_reports_malformed_front_matter ... ok\ntest store::tests::issue_parser_contract_rejects_unknown_top_level_sections ... ok\ntest store::tests::issue_parser_contract_rejects_missing_required_sections ... ok\ntest activity::tests::rejects_invalid_schema_version_subject_and_event_type ... ok\ntest store::tests::evidence_record_renders_and_parses_deterministically_without_data_blob ... ok\ntest store::tests::issue_record_renders_and_parses_deterministically ... ok\ntest store::tests::issue_sections_report_shared_presence_state_and_search_text ... ok\ntest store::tests::legacy_plan_and_milestone_data_records_load_into_typed_front_matter ... ok\ntest store::tests::issue_record_round_trips_explicit_closed_at_for_done_status ... ok\ntest store::tests::milestone_record_renders_and_parses_deterministically_without_data_blob ... ok\ntest store::tests::issue_parser_reports_schema_and_path_mismatch ... ok\ntest store::tests::mission_render_normalizes_legacy_evidence_attachments ... ok\ntest store::tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships ... ok\ntest store::tests::plan_record_renders_and_parses_deterministically_without_data_blob ... ok\ntest store::tests::mission_record_renders_and_parses_deterministically_without_data_blob ... ok\ntest store::tests::record_store_allocates_ids_across_canonical_dirs ... ok\ntest store::tests::registered_first_class_record_kinds_have_canonical_contracts ... ok\ntest store::tests::workflow_validator_kind_is_registered_but_not_canonical_yet ... ok\ntest store::tests::write_issue_atomic_rejects_path_traversal_ids_before_writing ... ok\ntest store::tests::legacy_evidence_data_record_loads_into_typed_front_matter ... ok\ntest store::tests::write_issue_atomic_ignores_stale_fixed_temp_artifact ... ok\ntest store::tests::record_store_block_rejects_cycles_and_self_blocks ... ok\ntest store::tests::record_store_block_unblock_mutates_blocker_relationships ... ok\ntest store::tests::record_store_relate_unrelate_mutates_both_issue_records ... ok\ntest store::tests::record_store_discovers_and_rejects_noncanonical_issue_paths ... ok\ntest store::tests::record_store_label_unlabel_mutates_issue_front_matter ... ok\n\ntest result: ok. 39 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\n"
    truncated: false
  stderr:
    bytes: 189
    summary: "    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s\n     Running unittests src/lib.rs (target/debug/deps/atelier_records-0109ccf0701167e5)\n   Doc-tests atelier_records\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-8wvr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier-records owns canonical record parsing, rendering, record-kind contracts, relationships, and activity sidecars; focused crate tests pass."
updated_at: "2026-06-15T07:40:39.224429462+00:00"
---

atelier-records owns canonical record parsing, rendering, record-kind contracts, relationships, and activity sidecars; focused crate tests pass.

Command: cargo test -p atelier-records
Exit status: 0

Stdout summary:

running 39 tests
test activity::tests::issue_activity_sidecar_path_is_canonical ... ok
test activity::tests::allocation_adds_deterministic_suffixes_for_same_timestamp_collisions ... ok
test activity::tests::rejects_invalid_schema ... ok
test activity::tests::front_matter_and_body_round_trip ... ok
test activity::tests::timestamp_activity_id_uses_utc_microseconds ... ok
test activity::tests::lists_issue_activities_in_oldest_first_order ... ok
test store::tests::issue_parser_contract_accepts_sectioned_body_without_legacy_arrays ... ok
test store::tests::issue_parser_contract_rejects_legacy_acceptance_and_evidence_front_matter ... ok
test store::tests::issue_parser_contract_rejects_content_before_first_recognized_heading ... ok
test activity::tests::create_allocates_collision_suffix_and_does_not_overwrite ... ok
test store::tests::issue_parser_contract_rejects_duplicate_recognized_headings ... ok
test store::tests::issue_parser_contract_rejects_empty_present_sections ... ok
test activity::tests::write_refuses_to_overwrite_existing_activity ... ok
test store::tests::issue_parser_reports_malformed_front_matter ... ok
test store::tests::issue_parser_contract_rejects_unknown_top_level_sections ... ok
test store::tests::issue_parser_contract_rejects_missing_required_sections ... ok
test activity::tests::rejects_invalid_schema_version_subject_and_event_type ... ok
test store::tests::evidence_record_renders_and_parses_deterministically_without_data_blob ... ok
test store::tests::issue_record_renders_and_parses_deterministically ... ok
test store::tests::issue_sections_report_shared_presence_state_and_search_text ... ok
test store::tests::legacy_plan_and_milestone_data_records_load_into_typed_front_matter ... ok
test store::tests::issue_record_round_trips_explicit_closed_at_for_done_status ... ok
test store::tests::milestone_record_renders_and_parses_deterministically_without_data_blob ... ok
test store::tests::issue_parser_reports_schema_and_path_mismatch ... ok
test store::tests::mission_render_normalizes_legacy_evidence_attachments ... ok
test store::tests::legacy_mission_data_record_loads_into_typed_sections_and_relationships ... ok
test store::tests::plan_record_renders_and_parses_deterministically_without_data_blob ... ok
test store::tests::mission_record_renders_and_parses_deterministically_without_data_blob ... ok
test store::tests::record_store_allocates_ids_across_canonical_dirs ... ok
test store::tests::registered_first_class_record_kinds_have_canonical_contracts ... ok
test store::tests::workflow_validator_kind_is_registered_but_not_canonical_yet ... ok
test store::tests::write_issue_atomic_rejects_path_traversal_ids_before_writing ... ok
test store::tests::legacy_evidence_data_record_loads_into_typed_front_matter ... ok
test store::tests::write_issue_atomic_ignores_stale_fixed_temp_artifact ... ok
test store::tests::record_store_block_rejects_cycles_and_self_blocks ... ok
test store::tests::record_store_block_unblock_mutates_blocker_relationships ... ok
test store::tests::record_store_relate_unrelate_mutates_both_issue_records ... ok
test store::tests::record_store_discovers_and_rejects_noncanonical_issue_paths ... ok
test store::tests::record_store_label_unlabel_mutates_issue_front_matter ... ok

test result: ok. 39 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Stderr summary:
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-0109ccf0701167e5)
   Doc-tests atelier_records

