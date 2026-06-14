---
created_at: "2026-06-14T00:14:23.920737264+00:00"
id: "atelier-wo62"
evidence_type: "validation"
captured_at: "2026-06-14T00:14:23.920621930+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4u5h"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Extracted issue workflow orientation helpers into src/commands/issue_workflow.rs. Focused tests passed: cargo nextest run -E 'test(test_root_status_summarizes_checkout_orientation) | test(test_root_status_reports_active_mission_contract_fields) | test(test_root_status_guides_active_work_to_transition_or_abandon) | test(test_root_status_no_ready_work_suggests_valid_blocked_list) | test(test_ready_issues) | test(test_quiet_issue_list_ready_outputs_ids_only) | test(test_issue_list_ready_rejects_closed_status) | test(test_issue_list_ready_treats_internal_epic_blockers_as_ready) | test(test_issue_ready_queue_requires_allowed_in_progress_transition) | test(test_mission_status_shows_ignored_product_behavior_closeout_blocker) | test(test_mission_audit_reports_missing_partial_and_ready_proof) | test(test_mission_status_names_concrete_closeout_blockers) | test(test_mission_status_names_stale_and_malformed_record_blockers) | test(test_mission_list_human_overview_orders_and_summarizes) | test(test_mission_status_cli_reports_control_state) | test(test_mission_list_default_current_empty_state) | test(test_work_lifecycle_human_output_and_guards)' (17 passed). Checks passed: cargo fmt -- --check; git diff --check; target/debug/atelier lint; target/debug/atelier export --check. Residue proof: rg -n 'commands::agent_factory|agent_factory::' src/commands/mission.rs returned no matches."
updated_at: "2026-06-14T00:14:26.039193605+00:00"
---

Extracted issue workflow orientation helpers into src/commands/issue_workflow.rs. Focused tests passed: cargo nextest run -E 'test(test_root_status_summarizes_checkout_orientation) | test(test_root_status_reports_active_mission_contract_fields) | test(test_root_status_guides_active_work_to_transition_or_abandon) | test(test_root_status_no_ready_work_suggests_valid_blocked_list) | test(test_ready_issues) | test(test_quiet_issue_list_ready_outputs_ids_only) | test(test_issue_list_ready_rejects_closed_status) | test(test_issue_list_ready_treats_internal_epic_blockers_as_ready) | test(test_issue_ready_queue_requires_allowed_in_progress_transition) | test(test_mission_status_shows_ignored_product_behavior_closeout_blocker) | test(test_mission_audit_reports_missing_partial_and_ready_proof) | test(test_mission_status_names_concrete_closeout_blockers) | test(test_mission_status_names_stale_and_malformed_record_blockers) | test(test_mission_list_human_overview_orders_and_summarizes) | test(test_mission_status_cli_reports_control_state) | test(test_mission_list_default_current_empty_state) | test(test_work_lifecycle_human_output_and_guards)' (17 passed). Checks passed: cargo fmt -- --check; git diff --check; target/debug/atelier lint; target/debug/atelier export --check. Residue proof: rg -n 'commands::agent_factory|agent_factory::' src/commands/mission.rs returned no matches.
