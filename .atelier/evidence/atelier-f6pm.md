---
created_at: "2026-06-15T07:46:31.073686823+00:00"
id: "atelier-f6pm"
evidence_type: "validation"
captured_at: "2026-06-15T07:46:31.038729195+00:00"
command: "sh -c 'find crates/atelier-core/tests crates/atelier-records/tests crates/atelier-workflow/tests crates/atelier-sqlite/tests -type f -maxdepth 1 -print | sort && rg -n \"record_ids_are_project_scoped|issue_records_round_trip|domain_records_render_parse|starter_policy_exposes|invalid_transition_statuses|source_snapshots_track|projection_freshness_reports|query_projection_covers\" crates/atelier-core/tests crates/atelier-records/tests crates/atelier-workflow/tests crates/atelier-sqlite/tests'"
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
    bytes: 1117
    summary: "crates/atelier-core/tests/domain_invariants.rs\ncrates/atelier-records/tests/canonical_records.rs\ncrates/atelier-sqlite/tests/projection_contract.rs\ncrates/atelier-workflow/tests/policy_contract.rs\ncrates/atelier-sqlite/tests/projection_contract.rs:67:fn source_snapshots_track_only_canonical_state_not_runtime_files() {\ncrates/atelier-sqlite/tests/projection_contract.rs:97:fn projection_freshness_reports_rebuild_boundary_changes() {\ncrates/atelier-sqlite/tests/projection_contract.rs:128:fn query_projection_covers_records_links_workflow_and_mission_summary() {\ncrates/atelier-workflow/tests/policy_contract.rs:13:fn starter_policy_exposes_task_transitions_and_categories() {\ncrates/atelier-workflow/tests/policy_contract.rs:50:fn invalid_transition_statuses_are_rejected_when_loading_policy() {\ncrates/atelier-records/tests/canonical_records.rs:40:fn issue_records_round_trip_through_canonical_store() {\ncrates/atelier-records/tests/canonical_records.rs:66:fn domain_records_render_parse_and_mutate_relationships() {\ncrates/atelier-core/tests/domain_invariants.rs:7:fn record_ids_are_project_scoped_and_base36() {\n"
    truncated: false
  stderr:
    bytes: 0
    summary: ""
    truncated: false
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
title: "sh -c 'find crates/atelier-core/tests crates/atelier-records/tests crates/atelier-workflow/tests crates/atelier-sqlite/tests -type f -maxdepth 1 -print | sort && rg -n \"record_ids_are_project_scoped|issue_records_round_trip|domain_records_render_parse|starter_policy_exposes|invalid_transition_statuses|source_snapshots_track|projection_freshness_reports|query_projection_covers\" crates/atelier-core/tests crates/atelier-records/tests crates/atelier-workflow/tests crates/atelier-sqlite/tests'"
updated_at: "2026-06-15T07:46:34.308352553+00:00"
---

sh -c 'find crates/atelier-core/tests crates/atelier-records/tests crates/atelier-workflow/tests crates/atelier-sqlite/tests -type f -maxdepth 1 -print | sort && rg -n "record_ids_are_project_scoped|issue_records_round_trip|domain_records_render_parse|starter_policy_exposes|invalid_transition_statuses|source_snapshots_track|projection_freshness_reports|query_projection_covers" crates/atelier-core/tests crates/atelier-records/tests crates/atelier-workflow/tests crates/atelier-sqlite/tests'

Command: sh -c 'find crates/atelier-core/tests crates/atelier-records/tests crates/atelier-workflow/tests crates/atelier-sqlite/tests -type f -maxdepth 1 -print | sort && rg -n "record_ids_are_project_scoped|issue_records_round_trip|domain_records_render_parse|starter_policy_exposes|invalid_transition_statuses|source_snapshots_track|projection_freshness_reports|query_projection_covers" crates/atelier-core/tests crates/atelier-records/tests crates/atelier-workflow/tests crates/atelier-sqlite/tests'
Exit status: 0

Stdout summary:
crates/atelier-core/tests/domain_invariants.rs
crates/atelier-records/tests/canonical_records.rs
crates/atelier-sqlite/tests/projection_contract.rs
crates/atelier-workflow/tests/policy_contract.rs
crates/atelier-sqlite/tests/projection_contract.rs:67:fn source_snapshots_track_only_canonical_state_not_runtime_files() {
crates/atelier-sqlite/tests/projection_contract.rs:97:fn projection_freshness_reports_rebuild_boundary_changes() {
crates/atelier-sqlite/tests/projection_contract.rs:128:fn query_projection_covers_records_links_workflow_and_mission_summary() {
crates/atelier-workflow/tests/policy_contract.rs:13:fn starter_policy_exposes_task_transitions_and_categories() {
crates/atelier-workflow/tests/policy_contract.rs:50:fn invalid_transition_statuses_are_rejected_when_loading_policy() {
crates/atelier-records/tests/canonical_records.rs:40:fn issue_records_round_trip_through_canonical_store() {
crates/atelier-records/tests/canonical_records.rs:66:fn domain_records_render_parse_and_mutate_relationships() {
crates/atelier-core/tests/domain_invariants.rs:7:fn record_ids_are_project_scoped_and_base36() {

Stderr summary:
(none)

